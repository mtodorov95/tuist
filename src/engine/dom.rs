use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag: String,
    pub attrs: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        return self.attrs.get("id");
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attrs.get("class") {
            Some(class_list) => class_list.split(" ").collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    return Node {
        children,
        node_type: NodeType::Element(ElementData { tag: name, attrs }),
    };
}

pub fn text(data: String) -> Node {
    return Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    };
}
