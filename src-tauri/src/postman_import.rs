use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct PostmanCollection {
    pub info: PostmanInfo,
    pub item: Vec<PostmanItem>,
}

#[derive(Debug, Deserialize)]
pub struct PostmanInfo {
    pub name: String,
    #[serde(rename = "_postman_id")]
    pub postman_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostmanItem {
    pub name: String,
    pub item: Option<Vec<PostmanItem>>, // For folders
    pub request: Option<PostmanRequest>,
}

#[derive(Debug, Deserialize)]
pub struct PostmanRequest {
    pub method: String,
    pub header: Option<Vec<PostmanHeader>>,
    pub body: Option<PostmanBody>,
    pub url: PostmanUrl,
    pub auth: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct PostmanHeader {
    pub key: String,
    pub value: String,
    #[serde(default = "default_enabled")]
    pub disabled: bool,
}

fn default_enabled() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct PostmanBody {
    pub mode: String,
    pub raw: Option<String>,
    pub formdata: Option<Vec<PostmanFormData>>,
    pub urlencoded: Option<Vec<PostmanFormData>>,
}

#[derive(Debug, Deserialize)]
pub struct PostmanFormData {
    pub key: String,
    pub value: Option<String>,
    #[serde(rename = "type")]
    pub field_type: Option<String>,
    pub src: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum PostmanUrl {
    String(String),
    Object(PostmanUrlObject),
}

#[derive(Debug, Deserialize)]
pub struct PostmanUrlObject {
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedCollection {
    pub name: String,
    pub folders: Vec<ImportedFolder>,
    pub requests: Vec<ImportedRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedFolder {
    pub name: String,
    pub parent_path: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedRequest {
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<ImportedHeader>,
    pub body: String,
    pub body_type: String,
    pub auth_type: String,
    pub auth_data: String,
    pub form_data: Vec<ImportedFormData>,
    pub folder_path: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedHeader {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedFormData {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub enabled: bool,
    pub file_path: Option<String>,
}

pub fn parse_postman_collection(json_content: &str) -> Result<ImportedCollection, String> {
    let collection: PostmanCollection =
        serde_json::from_str(json_content).map_err(|e| format!("Invalid Postman collection: {}", e))?;

    let mut folders = Vec::new();
    let mut requests = Vec::new();

    process_items(&collection.item, &mut folders, &mut requests, &mut Vec::new())?;

    Ok(ImportedCollection {
        name: collection.info.name,
        folders,
        requests,
    })
}

fn process_items(
    items: &[PostmanItem],
    folders: &mut Vec<ImportedFolder>,
    requests: &mut Vec<ImportedRequest>,
    current_path: &mut Vec<String>,
) -> Result<(), String> {
    for item in items {
        if let Some(ref subitems) = item.item {
            // This is a folder
            folders.push(ImportedFolder {
                name: item.name.clone(),
                parent_path: current_path.clone(),
            });

            current_path.push(item.name.clone());
            process_items(subitems, folders, requests, current_path)?;
            current_path.pop();
        } else if let Some(ref request) = item.request {
            // This is a request
            let url = match &request.url {
                PostmanUrl::String(s) => s.clone(),
                PostmanUrl::Object(obj) => obj.raw.clone(),
            };

            let headers = request
                .header
                .as_ref()
                .map(|h| {
                    h.iter()
                        .map(|header| ImportedHeader {
                            key: header.key.clone(),
                            value: header.value.clone(),
                            enabled: !header.disabled,
                        })
                        .collect()
                })
                .unwrap_or_default();

            let (body, body_type, form_data) = if let Some(ref body) = request.body {
                match body.mode.as_str() {
                    "raw" => (body.raw.clone().unwrap_or_default(), "json".to_string(), Vec::new()),
                    "formdata" => {
                        let form_fields = body
                            .formdata
                            .as_ref()
                            .map(|fields| {
                                fields
                                    .iter()
                                    .map(|field| ImportedFormData {
                                        key: field.key.clone(),
                                        value: field.value.clone().unwrap_or_default(),
                                        field_type: field.field_type.clone().unwrap_or_else(|| "text".to_string()),
                                        enabled: true,
                                        file_path: field.src.clone(),
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        (String::new(), "form".to_string(), form_fields)
                    }
                    "urlencoded" => {
                        let form_fields = body
                            .urlencoded
                            .as_ref()
                            .map(|fields| {
                                fields
                                    .iter()
                                    .map(|field| ImportedFormData {
                                        key: field.key.clone(),
                                        value: field.value.clone().unwrap_or_default(),
                                        field_type: "text".to_string(),
                                        enabled: true,
                                        file_path: None,
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        (String::new(), "urlencoded".to_string(), form_fields)
                    }
                    _ => (String::new(), "none".to_string(), Vec::new()),
                }
            } else {
                (String::new(), "none".to_string(), Vec::new())
            };

            let (auth_type, auth_data) = if let Some(ref auth) = request.auth {
                parse_auth(auth)
            } else {
                ("none".to_string(), "{}".to_string())
            };

            requests.push(ImportedRequest {
                name: item.name.clone(),
                method: request.method.clone(),
                url,
                headers,
                body,
                body_type,
                auth_type,
                auth_data,
                form_data,
                folder_path: current_path.clone(),
            });
        }
    }

    Ok(())
}

fn parse_auth(auth: &Value) -> (String, String) {
    if let Some(auth_type) = auth.get("type").and_then(|t| t.as_str()) {
        match auth_type {
            "basic" => {
                if let Some(basic) = auth.get("basic").and_then(|b| b.as_array()) {
                    let mut username = String::new();
                    let mut password = String::new();

                    for item in basic {
                        if let (Some(key), Some(value)) = (
                            item.get("key").and_then(|k| k.as_str()),
                            item.get("value").and_then(|v| v.as_str()),
                        ) {
                            match key {
                                "username" => username = value.to_string(),
                                "password" => password = value.to_string(),
                                _ => {}
                            }
                        }
                    }

                    let auth_data = serde_json::json!({
                        "username": username,
                        "password": password
                    });
                    return ("basic".to_string(), auth_data.to_string());
                }
            }
            "bearer" => {
                if let Some(bearer) = auth.get("bearer").and_then(|b| b.as_array()) {
                    for item in bearer {
                        if let (Some(key), Some(value)) = (
                            item.get("key").and_then(|k| k.as_str()),
                            item.get("value").and_then(|v| v.as_str()),
                        ) {
                            if key == "token" {
                                let auth_data = serde_json::json!({
                                    "token": value
                                });
                                return ("bearer".to_string(), auth_data.to_string());
                            }
                        }
                    }
                }
            }
            "apikey" => {
                if let Some(apikey) = auth.get("apikey").and_then(|a| a.as_array()) {
                    let mut key = String::new();
                    let mut value = String::new();

                    for item in apikey {
                        if let (Some(k), Some(v)) = (
                            item.get("key").and_then(|k| k.as_str()),
                            item.get("value").and_then(|v| v.as_str()),
                        ) {
                            match k {
                                "key" => key = v.to_string(),
                                "value" => value = v.to_string(),
                                _ => {}
                            }
                        }
                    }

                    let auth_data = serde_json::json!({
                        "key": key,
                        "value_field": value
                    });
                    return ("apikey".to_string(), auth_data.to_string());
                }
            }
            _ => {}
        }
    }

    ("none".to_string(), "{}".to_string())
}
