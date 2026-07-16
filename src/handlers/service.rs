use crate::handlers::document::Document;
use crate::handlers::document::DocumentformInterface;
use custom_logger as log;
use http::{Method, Request, Response, StatusCode};
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use std::collections::HashMap;

pub async fn endpoints(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let mut response = Response::new(Full::default());
    let uri = req.uri().path();
    match *req.method() {
        Method::POST => match uri {
            x if x.contains("write") => {
                let headers = req.headers();
                let res_access = headers.get("unikernel-access");
                match res_access {
                    Some(access) => {
                        let access_value = access.to_str().unwrap_or_default();
                        if access_value.is_empty() || access_value != "valid" {
                            *response.status_mut() = StatusCode::FORBIDDEN;
                            *response.body_mut() = Full::from("[endpoints] : access forbidden");
                        } else {
                            let data = req.into_body().collect().await?.to_bytes();
                            let result = aggregate_write_payload(data).await;
                            match result {
                                Ok(content) => {
                                    *response.body_mut() = Full::from(content);
                                }
                                Err(e) => {
                                    log::error!("[endpoints] {}", e);
                                    *response.body_mut() =
                                        Full::from(format!("[endpoints] {}\n", e));
                                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                }
                            }
                        }
                    }
                    None => {
                        log::error!("[endpoints] access forbidden ");
                        *response.status_mut() = StatusCode::FORBIDDEN;
                        *response.body_mut() = Full::from("[endpoints] : access forbidden\n");
                    }
                }
            }
            x if x.contains("read") => {
                let data = req.into_body().collect().await?.to_bytes();
                let result = aggregate_read_payload(data).await;
                match result {
                    Ok(content) => {
                        *response.body_mut() = Full::from(content);
                    }
                    Err(e) => {
                        log::error!("[endpoints] {}", e);
                        *response.body_mut() = Full::from(format!("[endpoints] {}\n", e));
                        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    }
                }
            }
            x if x.contains("search") => {
                let data = req.into_body().collect().await?.to_bytes();
                let result = aggregate_list_payload(data).await;
                match result {
                    Ok(content) => {
                        *response.body_mut() = Full::from(content);
                    }
                    Err(e) => {
                        log::error!("[endpoints] {}", e);
                        *response.body_mut() = Full::from(format!("[endpoints] {}\n", e));
                        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    }
                }
            }
            &_ => {}
        },
        Method::GET => {
            let params: HashMap<String, String> = req
                .uri()
                .query()
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_default();
            if params.is_empty() && req.uri().path().contains("/v1/health") {
                let content = format!(
                    r##"{{ "status":"ok", "appplication": "{}", "version": "{}" }}"##,
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                );
                *response.body_mut() = Full::from(content);
            } else {
                log::debug!("[endpoints] params {:?}", params);
                let empty = &"".to_string();
                let document = params.get("document").unwrap_or(empty);
                let result = aggregate_read(document.to_string()).await;
                match result {
                    Ok(content) => {
                        *response.body_mut() = Full::from(content);
                    }
                    Err(e) => {
                        log::error!("[endpoints] {}", e);
                        *response.body_mut() = Full::from(format!("[endpoints] {}\n", e));
                        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    }
                }
            }
        }
        _ => {
            log::error!("[endpoints] method/path not implemented");
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

async fn aggregate_write_payload(data: Bytes) -> Result<String, Box<dyn std::error::Error>> {
    let hm: HashMap<String, String> = serde_json::from_slice(&data)?;
    let empty = "".to_string();
    let document_name = &hm.get("document").unwrap_or(&empty);
    let content = &hm.get("content").unwrap_or(&empty);
    if document_name.is_empty() || content.is_empty() {
        return Err(Box::from(
            "[aggregate_write_payload] json payload (document && content) are empty",
        ));
    }
    let result = Document::save_document(document_name.to_string(), content.to_string()).await?;
    Ok(result)
}

async fn aggregate_read_payload(data: Bytes) -> Result<String, Box<dyn std::error::Error>> {
    let hm: HashMap<String, String> = serde_json::from_slice(&data)?;
    let empty = "".to_string();
    let document_name = &hm.get("document").unwrap_or(&empty);
    if document_name.is_empty() {
        return Err(Box::from(
            "[aggregate_read_payload] json payload (document) is empty",
        ));
    }
    let result = Document::get_document(document_name.to_string()).await?;
    Ok(result)
}

async fn aggregate_list_payload(data: Bytes) -> Result<String, Box<dyn std::error::Error>> {
    let hm: HashMap<String, String> = serde_json::from_slice(&data)?;
    let empty = "".to_string();
    let search = &hm.get("search").unwrap_or(&empty);
    if search.is_empty() {
        return Err(Box::from(
            "[aggregate_read_payload] json payload (document) is empty",
        ));
    }
    let result = Document::search_document(search.to_string()).await?;
    Ok(result)
}

async fn aggregate_read(document: String) -> Result<String, Box<dyn std::error::Error>> {
    if document.is_empty() {
        return Err(Box::from("parameter document is empty"));
    }
    let result = Document::get_document(document).await?;
    Ok(result)
}
