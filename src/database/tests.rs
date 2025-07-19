use crate::database::{
    Database,
    node::{Node, Type},
};

use serde_json::Value;

#[test]
fn create_database() {
    let node = Node::new(Type::Domain, String::from("krypton.ninja"));
    let secondary_node = Node::new(Type::Domain, String::from("go.krypton.ninja"));
    let mut database = Database::new(node.clone());
    assert!(database.get_root().equals(&node));
    assert!(!database.get_root().equals(&secondary_node));
}

#[test]
fn add_node() {
    let node = Node::new(Type::Domain, String::from("krypton.ninja"));
    let mut database = Database::new(node.clone());
    let get_node = database
        .search(Type::Domain, String::from("krypton.ninja"))
        .unwrap();
    get_node.add(Type::Domain, String::from("go.krypton.ninja"));
    get_node.add(Type::Domain, String::from("status.krypton.ninja"));
    assert_eq!(database.get_root().get_connections().len(), 2);
}

#[test]
fn add_node_to_root() {
    let node = Node::new(Type::Domain, String::from("krypton.ninja"));
    let mut database = Database::new(node.clone());
    let root_node = database.get_root();
    root_node.add(Type::Domain, String::from("go.krypton.ninja"));
    root_node.add(Type::Domain, String::from("status.krypton.ninja"));
    assert_eq!(database.get_root().get_connections().len(), 2);
}

#[test]
fn add_node_data() {
    let mut node = Node::new(Type::Domain, String::from("krypton.ninja"));
    node.add_data(String::from("is_root"), Value::Bool(true));
    let mut database = Database::new(node.clone());
    assert!(database.get_root().get_data("is_root").is_some());
    assert!(database.get_root().get_data("test").is_none());
}

#[test]
fn search_node() {
    let node = Node::new(Type::Domain, String::from("krypton.ninja"));
    let mut database = Database::new(node.clone());
    let root_node = database.get_root();
    root_node.add(Type::Domain, String::from("go.krypton.ninja"));
    root_node.add(Type::Domain, String::from("status.krypton.ninja"));
    assert!(
        database
            .search(Type::Domain, String::from("go.krypton.ninja"))
            .is_some()
    );
    assert!(
        database
            .search(Type::Domain, String::from("status.krypton.ninja"))
            .is_some()
    );
    assert!(
        database
            .search(Type::Domain, String::from("www.krypton.ninja"))
            .is_none()
    );
}

#[test]
fn search_node_root() {
    let node = Node::new(Type::Domain, String::from("krypton.ninja"));
    let mut database = Database::new(node.clone());
    assert!(
        database
            .search(Type::Domain, String::from("krypton.ninja"))
            .is_some()
    );
    assert!(
        database
            .search(Type::Domain, String::from("go.krypton.ninja"))
            .is_none()
    );
}
