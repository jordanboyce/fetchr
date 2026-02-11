use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<KeyValue>,
    pub body: String,
    pub body_type: String,
    pub auth_type: String,
    pub auth_data: AuthData,
    pub form_data: Option<Vec<FormDataField>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormDataField {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub enabled: bool,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuthData {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub key: Option<String>,
    pub value_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub response_time: u128,
    pub size: usize,
    pub cookies: Vec<Cookie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
}

pub async fn send_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .map_err(|e| e.to_string())?;

    let method = request
        .method
        .parse::<reqwest::Method>()
        .map_err(|e| e.to_string())?;

    let start = Instant::now();

    // Build headers
    let mut headers = HeaderMap::new();
    for header in request.headers.iter().filter(|h| h.enabled) {
        let name = HeaderName::from_bytes(header.key.as_bytes()).map_err(|e| e.to_string())?;
        let value = HeaderValue::from_str(&header.value).map_err(|e| e.to_string())?;
        headers.insert(name, value);
    }

    // Build request
    let mut req_builder = client.request(method, &request.url).headers(headers);

    // Add authentication
    match request.auth_type.as_str() {
        "basic" => {
            if let (Some(username), Some(password)) =
                (&request.auth_data.username, &request.auth_data.password)
            {
                req_builder = req_builder.basic_auth(username, Some(password));
            }
        }
        "bearer" => {
            if let Some(token) = &request.auth_data.token {
                req_builder = req_builder.bearer_auth(token);
            }
        }
        "apikey" => {
            if let (Some(key), Some(value)) =
                (&request.auth_data.key, &request.auth_data.value_field)
            {
                req_builder = req_builder.header(key.as_str(), value.as_str());
            }
        }
        _ => {}
    }

    // Add body
    match request.body_type.as_str() {
        "json" if !request.body.is_empty() => {
            req_builder = req_builder
                .header("Content-Type", "application/json")
                .body(request.body.clone());
        }
        "raw" if !request.body.is_empty() => {
            req_builder = req_builder.body(request.body.clone());
        }
        "form" => {
            if let Some(form_fields) = &request.form_data {
                let mut form = reqwest::multipart::Form::new();
                for field in form_fields.iter().filter(|f| f.enabled) {
                    match field.field_type.as_str() {
                        "file" => {
                            if let Some(file_path) = &field.file_path {
                                // Read file from disk
                                match std::fs::read(file_path) {
                                    Ok(file_bytes) => {
                                        let filename = std::path::Path::new(file_path)
                                            .file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("file");
                                        let part = reqwest::multipart::Part::bytes(file_bytes)
                                            .file_name(filename.to_string());
                                        form = form.part(field.key.clone(), part);
                                    }
                                    Err(e) => {
                                        return Err(format!("Failed to read file {}: {}", file_path, e));
                                    }
                                }
                            }
                        }
                        _ => {
                            // text field
                            form = form.text(field.key.clone(), field.value.clone());
                        }
                    }
                }
                req_builder = req_builder.multipart(form);
            }
        }
        _ => {}
    }

    // Send request
    let response = req_builder.send().await.map_err(|e| e.to_string())?;

    let elapsed = start.elapsed().as_millis();

    let status = response.status();
    let status_code = status.as_u16();
    let status_text = status.canonical_reason().unwrap_or("Unknown").to_string();

    // Extract headers
    let response_headers: HashMap<String, String> = response
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or("").to_string(),
            )
        })
        .collect();

    // Extract cookies (basic extraction from Set-Cookie headers)
    let cookies: Vec<Cookie> = response
        .headers()
        .get_all("set-cookie")
        .iter()
        .filter_map(|v| {
            let cookie_str = v.to_str().ok()?;
            let parts: Vec<&str> = cookie_str.split(';').collect();
            if let Some(name_value) = parts.first() {
                let kv: Vec<&str> = name_value.splitn(2, '=').collect();
                if kv.len() == 2 {
                    return Some(Cookie {
                        name: kv[0].trim().to_string(),
                        value: kv[1].trim().to_string(),
                        domain: None,
                        path: None,
                    });
                }
            }
            None
        })
        .collect();

    // Get body
    let body_bytes = response.bytes().await.map_err(|e| e.to_string())?;
    let size = body_bytes.len();
    let body = String::from_utf8_lossy(&body_bytes).to_string();

    Ok(HttpResponse {
        status: status_code,
        status_text,
        headers: response_headers,
        body,
        response_time: elapsed,
        size,
        cookies,
    })
}
