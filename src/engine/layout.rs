use super::Node;

pub fn get_text_content(node: &Node, out: &mut String) -> String {
    if let crate::engine::NodeType::Text(ref text) = node.node_type {
        out.push_str(text)
    }

    for child in &node.children {
        get_text_content(child, out);
    }

    if let crate::engine::NodeType::Element(data) = &node.node_type {
        match data.tag.as_str() {
            "div" | "p" | "ul" => out.push_str("\n\n"),
            tag if tag.starts_with('h') => out.push_str("\n\n"),
            "a" | "span" => out.push(' '),
            _ => {}
        }
    }
    out.to_string()
}
