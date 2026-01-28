use crate::handlers::common::get_opts;
use custom_logger as log;
use serde_derive::{Deserialize, Serialize};

pub trait DocumentformInterface {
    async fn save_document(key: String, data: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_document(key: String) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {}

impl DocumentformInterface for Document {
    async fn save_document(key: String, data: String) -> Result<(), Box<dyn std::error::Error>> {
        db_upsert(key, data).await?;
        Ok(())
    }
    async fn get_document(key: String) -> Result<String, Box<dyn std::error::Error>> {
        let res = db_read(key).await?;
        Ok(res)
    }
}

async fn db_upsert(id: String, data: String) -> Result<String, Box<dyn std::error::Error>> {
    let db_key = id.split("-").nth(0).unwrap_or("");
    let tree = get_opts(db_key.to_string())?;
    // start transaction
    let mut txn = tree.begin()?;
    txn.set_durability(surrealkv::Durability::Eventual);
    txn.set(id.as_bytes(), data.as_bytes())?;
    // commit transaction
    txn.commit().await?;
    tree.close().await?;
    let msg = format!("form data {} created/updated successfully", id);
    Ok(msg)
}

async fn db_read(id: String) -> Result<String, Box<dyn std::error::Error>> {
    let db_key = id.split("-").nth(0).unwrap_or("");
    let tree = get_opts(db_key.to_string())?;
    // start transaction
    let mut txn = tree.begin()?;
    log::debug!("[db_read] key {}", id);
    let result = txn.get(id.as_bytes())?;
    // commit transaction
    txn.commit().await?;
    tree.close().await?;
    let res = match result {
        Some(value) => String::from_utf8(value.to_vec()).unwrap(),
        None => "".to_string(),
    };
    Ok(res)
}

#[allow(unused)]
async fn db_delete(db: String, id: String) -> Result<(), Box<dyn std::error::Error>> {
    let db_key = id.split("-").nth(0).unwrap_or("");
    let tree = get_opts(db_key.to_string())?;
    // start transaction
    let mut txn = tree.begin()?;
    log::debug!("[db_read] key {}", id);
    txn.delete(id.as_bytes())?;
    // commit transaction
    txn.commit().await?;
    tree.close().await?;
    Ok(())
}
