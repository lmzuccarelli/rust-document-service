use crate::MAP_LOOKUP;
use custom_logger as log;
use surrealkv::{Tree, TreeBuilder};

pub fn get_opts() -> Result<Tree, Box<dyn std::error::Error>> {
    let db_path = get_item("db_path")?;
    let tree = TreeBuilder::new()
        .with_path(format!("{}/documents.kv", db_path).into())
        .with_max_memtable_size(100 * 1024 * 1024)
        .with_block_size(4096)
        .with_level_count(1);
    let t = tree.build()?;
    log::info!("[get_opts] tree built");
    Ok(t)
}

fn get_item(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hm_guard = MAP_LOOKUP.lock().map_err(|_| "mutex lock failed")?;
    let value = match hm_guard.as_ref() {
        Some(res) => {
            let item_value = res.get(name);
            match item_value {
                Some(final_value) => final_value,
                None => {
                    return Err(Box::from(format!(
                        "[get_item] hashmap lookup {} not found",
                        name
                    )));
                }
            }
        }
        None => {
            return Err(Box::from("[get_item] error validating hashmap lookup"));
        }
    };
    Ok(value.to_string())
}
