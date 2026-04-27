use crate::model::{ServerCatalog, ServerEntry};

pub fn load_server_catalog() -> Result<Vec<ServerEntry>, serde_json::Error> {
    let catalog: ServerCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/test_servers.json"
    )))?;
    Ok(catalog
        .servers
        .into_iter()
        .filter(|server| server.enabled)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::load_server_catalog;

    #[test]
    fn loads_non_empty_catalog() {
        let servers = load_server_catalog().expect("catalog should load");
        assert!(!servers.is_empty(), "catalog should not be empty");
        assert!(servers.iter().all(|server| server.enabled));
    }
}
