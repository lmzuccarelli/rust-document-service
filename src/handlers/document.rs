use crate::handlers::common::get_opts;
use custom_logger as log;
use serde_derive::{Deserialize, Serialize};
use surrealkv::{LSMIterator, ReadOptions};

pub trait DocumentformInterface {
    async fn save_document(key: String, data: String)
    -> Result<String, Box<dyn std::error::Error>>;
    async fn get_document(key: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn search_document(search: String) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyResponse {
    content: Vec<String>,
}

impl DocumentformInterface for Document {
    async fn save_document(
        key: String,
        data: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let res = db_upsert(key, data).await?;
        Ok(res)
    }
    async fn get_document(key: String) -> Result<String, Box<dyn std::error::Error>> {
        let res = db_read(key).await?;
        Ok(res)
    }
    async fn search_document(category: String) -> Result<String, Box<dyn std::error::Error>> {
        let res = db_search(category).await?;
        Ok(res)
    }
}

async fn db_upsert(id: String, data: String) -> Result<String, Box<dyn std::error::Error>> {
    let tree = get_opts()?;
    // start transaction
    let mut txn = tree.begin()?;
    txn.set_durability(surrealkv::Durability::Eventual);
    txn.set(id.as_bytes(), data.as_bytes())?;
    // commit transaction
    txn.commit().await?;
    tree.close().await?;
    // real lazy way creating json response
    let content = format!(
        r##"{{ "content": "form data {} created/updated successfully" }}"##,
        id
    );
    Ok(format!("{}\n", content))
}

async fn db_read(id: String) -> Result<String, Box<dyn std::error::Error>> {
    let tree = get_opts()?;
    // start transaction
    let mut txn = tree.begin()?;
    log::debug!("[db_read] key {}", id);
    let result = txn.get(id.as_bytes())?;
    // commit transaction
    txn.commit().await?;
    tree.close().await?;
    let res = match result {
        Some(value) => String::from_utf8(value.to_vec())?,
        None => "".to_string(),
    };
    let content = format!(r#"{{ "content": "{}" }}"#, res);
    Ok(format!("{}\n", content))
}

async fn db_search(category: String) -> Result<String, Box<dyn std::error::Error>> {
    let tree = get_opts()?;
    let mut vec_keys = vec![];
    // start transaction
    let txn = tree.begin()?;
    log::debug!("[db_search] key (category) {}", category);
    let category: Vec<&str> = category.split("/").collect();
    let from = format!("{}/a", category[0]);
    let to = format!("{}/z", category[0]);
    let mut options = ReadOptions::new();
    options.set_iterate_lower_bound(Some(from.as_bytes().to_vec()));
    options.set_iterate_upper_bound(Some(to.as_bytes().to_vec()));
    let mut it = txn.range_with_options(&options)?;
    it.seek_first()?;
    while it.valid() {
        let key = it.key();
        let key_val = String::from_utf8(key.user_key().to_vec())?;
        vec_keys.push(key_val);
        it.next()?;
    }
    // TODO: commit transaction
    //txn.commit().await?;
    //tree.close().await?;
    let kr = KeyResponse {
        content: vec_keys.clone(),
    };
    let result = serde_json::to_string(&kr)?;
    Ok(format!("{}\n", result))
}

#[allow(unused)]
async fn db_delete(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let tree = get_opts()?;
    // start transaction
    let mut txn = tree.begin()?;
    log::debug!("[db_read] key {}", id);
    txn.delete(id.as_bytes())?;
    // commit transaction
    txn.commit().await?;
    tree.close().await?;
    Ok(())
}
