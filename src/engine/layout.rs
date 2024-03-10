use super::Node;

pub fn get_text_content(node: &Node, out: &mut String) -> String {
    match node.node_type {
        crate::engine::NodeType::Text(ref text) => out.push_str(text),
        _ => {}
    }

    for child in &node.children {
        get_text_content(&child, out);
    }

    match &node.node_type {
        crate::engine::NodeType::Element(data) => match data.tag.as_str() {
            "div" | "p" | "ul" => out.push_str("\n\n"),
            tag if tag.starts_with('h') => out.push_str("\n\n"),
            "a" | "span" => out.push(' '),
            _ => {}
        },
        _ => {}
    }
    out.to_string()
}
