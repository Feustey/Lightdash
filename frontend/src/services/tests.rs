use wasm_bindgen_test::*;
use super::node::{NodeInfo, search_node, get_node_info, get_current_node};
use gloo_storage::{LocalStorage, Storage};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_search_node_with_pubkey() {
    let pubkey = "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b";
    let result = search_node(pubkey).await;
    assert!(result.is_ok(), "La recherche par pubkey devrait réussir");
    
    if let Ok(nodes) = result {
        assert_eq!(nodes.len(), 1, "Devrait retourner exactement un nœud");
        assert_eq!(nodes[0].pubkey, pubkey, "Le pubkey retourné devrait correspondre");
    }
}

#[wasm_bindgen_test]
async fn test_search_node_with_alias() {
    let alias = "ACINQ";
    let result = search_node(alias).await;
    assert!(result.is_ok(), "La recherche par alias devrait réussir");
    
    if let Ok(nodes) = result {
        assert!(!nodes.is_empty(), "Devrait retourner au moins un nœud");
    }
}

#[wasm_bindgen_test]
async fn test_get_node_info() {
    let pubkey = "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b";
    let result = get_node_info(pubkey).await;
    assert!(result.is_ok(), "Devrait pouvoir récupérer les infos du nœud");
    
    if let Ok(node) = result {
        assert_eq!(node.pubkey, pubkey);
        assert!(node.capacity > 0, "La capacité devrait être positive");
        assert!(node.num_channels > 0, "Devrait avoir au moins un canal");
    }
}

#[wasm_bindgen_test]
fn test_get_current_node() {
    // Test avec une valeur par défaut
    LocalStorage::delete("current_node_pubkey");
    let default_node = get_current_node();
    assert_eq!(default_node, "02778f4a4eb3a2344b9fd8ee72e7ec5f03f803e5f5273e2e1a2af508910cf2b12b");

    // Test avec une valeur personnalisée
    let custom_pubkey = "030a58b8653d32b99200a2334cfe913e51dc7d155aa0116c176657a4f1722677a3";
    LocalStorage::set("current_node_pubkey", custom_pubkey).unwrap();
    let stored_node = get_current_node();
    assert_eq!(stored_node, custom_pubkey);
}

#[wasm_bindgen_test]
fn test_node_info_struct() {
    let node = NodeInfo {
        pubkey: "test_pubkey".to_string(),
        alias: "test_alias".to_string(),
        capacity: 1000000,
        num_channels: 10,
        active_channels: 8,
    };

    assert_eq!(node.pubkey, "test_pubkey");
    assert_eq!(node.alias, "test_alias");
    assert_eq!(node.capacity, 1000000);
    assert_eq!(node.num_channels, 10);
    assert_eq!(node.active_channels, 8);
} 