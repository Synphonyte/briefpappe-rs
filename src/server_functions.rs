use leptos::*;
use crate::types::{Paper, PaperFilter};

#[server(ListPapers, "/api", "Cbor")]
pub async fn list_papers(filter: PaperFilter) -> Result<Vec<Paper>, ServerFnError> {
    // TODO : implement filtering
    Ok(vec![
        Paper {
            title: "Paper 1".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            timestamp: 0,
        },
        Paper {
            title: "Paper 1".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            timestamp: 2,
        },
        Paper {
            title: "Paper 2".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            timestamp: 1,
        },
    ])
}


#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = ListPapers::register();
}
