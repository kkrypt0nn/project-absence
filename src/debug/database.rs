use crate::database::{Database, node::Node};

pub fn render_compact(database: &mut Database) {
    let root_node = database.get_root();
    println!(". {}", root_node);
    render_nodes(root_node.get_connections(), "")
}

fn render_nodes(nodes: Vec<Node>, prefix: &str) {
    for (i, node) in nodes.iter().enumerate() {
        let is_last = i == nodes.len() - 1;
        let arrow = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, arrow, node);
        render_nodes(node.get_connections(), format!("{}│   ", prefix).as_str());
    }
}
