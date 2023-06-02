use leptos::*;
use crate::types::{Collection, CollectionFilter};

#[server(ListCollections, "/api", "Cbor")]
pub async fn list_collections(filter: CollectionFilter) -> Result<Vec<Collection>, ServerFnError> {
    // TODO : implement filtering
    Ok(vec![
        Collection {
            title: "Paper 1".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            timestamp: 0,
            slug: "paper-1".to_string(),
            papers: vec!["22-anchor-hope".to_string()],
        },
        Collection {
            title: "Paper 1".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            timestamp: 2,
            slug: "paper-2".to_string(),
            ..Default::default()
        },
        Collection {
            title: "Paper 2".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            timestamp: 1,
            slug: "paper-3".to_string(),
            ..Default::default()
        },
    ])
}


#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = ListCollections::register();
}
