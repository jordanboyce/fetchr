mod db;
mod http_client;
mod postman_import;

use db::{Collection, Database, Environment, History, Request};
use http_client::{send_request, HttpRequest, HttpResponse};
use postman_import::{parse_postman_collection, ImportedCollection};
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Mutex<Database>,
}

// HTTP Client Commands
#[tauri::command]
async fn send_http_request(request: HttpRequest) -> Result<HttpResponse, String> {
    send_request(request).await
}

#[tauri::command]
fn interpolate_variables(text: String, state: State<AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();
    let env = db.get_active_environment().map_err(|e| e.to_string())?;

    if let Some(env) = env {
        let variables: Vec<serde_json::Value> =
            serde_json::from_str(&env.variables).unwrap_or_default();
        let mut result = text;

        for var in variables {
            if let (Some(key), Some(value)) = (var["key"].as_str(), var["value"].as_str()) {
                let pattern = format!("{{{{{}}}}}", key);
                result = result.replace(&pattern, value);
            }
        }

        Ok(result)
    } else {
        Ok(text)
    }
}

// Collection Commands
#[tauri::command]
fn create_collection(collection: Collection, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.create_collection(&collection).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_all_collections(state: State<AppState>) -> Result<Vec<Collection>, String> {
    let db = state.db.lock().unwrap();
    db.get_all_collections().map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_collection(id: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_collection(&id).map_err(|e| e.to_string())
}

// Request Commands
#[tauri::command]
fn save_request(request: Request, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.save_request(&request).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_requests_by_collection(
    collection_id: String,
    state: State<AppState>,
) -> Result<Vec<Request>, String> {
    let db = state.db.lock().unwrap();
    db.get_requests_by_collection(&collection_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_request(id: String, state: State<AppState>) -> Result<Option<Request>, String> {
    let db = state.db.lock().unwrap();
    db.get_request(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_request(id: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_request(&id).map_err(|e| e.to_string())
}

// Environment Commands
#[tauri::command]
fn save_environment(env: Environment, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.save_environment(&env).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_all_environments(state: State<AppState>) -> Result<Vec<Environment>, String> {
    let db = state.db.lock().unwrap();
    db.get_all_environments().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_active_environment(state: State<AppState>) -> Result<Option<Environment>, String> {
    let db = state.db.lock().unwrap();
    db.get_active_environment().map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_environment(id: String, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_environment(&id).map_err(|e| e.to_string())
}

// History Commands
#[tauri::command]
fn add_history(history: History, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.add_history(&history).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_history(limit: i32, state: State<AppState>) -> Result<Vec<History>, String> {
    let db = state.db.lock().unwrap();
    db.get_history(limit).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_history(state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.clear_history().map_err(|e| e.to_string())
}

// Import/Export Commands
#[tauri::command]
async fn import_postman_collection(json_content: String) -> Result<ImportedCollection, String> {
    parse_postman_collection(&json_content)
}

#[tauri::command]
fn save_imported_collection(
    imported: ImportedCollection,
    state: State<AppState>,
) -> Result<String, String> {
    let db = state.db.lock().unwrap();

    // Create root collection
    let root_id = uuid::Uuid::new_v4().to_string();
    let root_collection = Collection {
        id: root_id.clone(),
        name: imported.name,
        parent_id: None,
        is_folder: true,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    db.create_collection(&root_collection).map_err(|e| e.to_string())?;

    // Create folders and track their IDs
    let mut folder_map = std::collections::HashMap::new();
    folder_map.insert(Vec::<String>::new(), root_id.clone());

    for folder in imported.folders {
        let folder_id = uuid::Uuid::new_v4().to_string();
        let parent_id = folder_map.get(&folder.parent_path).cloned();

        let collection = Collection {
            id: folder_id.clone(),
            name: folder.name.clone(),
            parent_id,
            is_folder: true,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        db.create_collection(&collection).map_err(|e| e.to_string())?;

        let mut path = folder.parent_path.clone();
        path.push(folder.name);
        folder_map.insert(path, folder_id);
    }

    // Create requests
    for request in imported.requests {
        let collection_id = folder_map
            .get(&request.folder_path)
            .cloned()
            .unwrap_or_else(|| root_id.clone());

        let req = Request {
            id: uuid::Uuid::new_v4().to_string(),
            collection_id,
            name: request.name,
            method: request.method,
            url: request.url,
            headers: serde_json::to_string(&request.headers).unwrap_or_default(),
            body: request.body,
            body_type: request.body_type,
            auth_type: request.auth_type,
            auth_data: request.auth_data,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        db.save_request(&req).map_err(|e| e.to_string())?;
    }

    Ok(root_id)
}

#[tauri::command]
fn export_collection(collection_id: String, state: State<AppState>) -> Result<String, String> {
    let db = state.db.lock().unwrap();

    // Get the collection
    let collections = db.get_all_collections().map_err(|e| e.to_string())?;
    let collection = collections.iter()
        .find(|c| c.id == collection_id)
        .ok_or_else(|| "Collection not found".to_string())?;

    // Get all requests in this collection
    let requests = db.get_requests_by_collection(&collection_id)
        .map_err(|e| e.to_string())?;

    // Build export JSON
    let mut export_data = serde_json::json!({
        "name": collection.name,
        "requests": []
    });

    if let Some(requests_array) = export_data["requests"].as_array_mut() {
        for request in requests {
            requests_array.push(serde_json::json!({
                "name": request.name,
                "method": request.method,
                "url": request.url,
                "headers": serde_json::from_str::<serde_json::Value>(&request.headers).unwrap_or(serde_json::json!([])),
                "body": request.body,
                "body_type": request.body_type,
                "auth_type": request.auth_type,
                "auth_data": serde_json::from_str::<serde_json::Value>(&request.auth_data).unwrap_or(serde_json::json!({})),
            }));
        }
    }

    serde_json::to_string_pretty(&export_data).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Initialize database
            let app_data_dir = app.path().app_data_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."));
            let db_path = app_data_dir.join("fetchr.db");

            std::fs::create_dir_all(db_path.parent().unwrap()).ok();

            let db = Database::new(db_path.to_str().unwrap()).expect("Failed to initialize database");
            app.manage(AppState { db: Mutex::new(db) });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_http_request,
            interpolate_variables,
            create_collection,
            get_all_collections,
            delete_collection,
            save_request,
            get_requests_by_collection,
            get_request,
            delete_request,
            save_environment,
            get_all_environments,
            get_active_environment,
            delete_environment,
            add_history,
            get_history,
            clear_history,
            import_postman_collection,
            save_imported_collection,
            export_collection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
