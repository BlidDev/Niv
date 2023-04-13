use std::collections::HashMap;


#[derive(Debug)]
pub struct Node {
    pub command : String,
    pub id : usize,
    pub parent : usize,
    pub childern : Vec<usize>
}


#[derive(Debug)]
pub struct Scope {
    pub nodes : HashMap<usize, NodeType>,
    pub counter : usize
}

impl Default for Scope {
   fn default() -> Self {
       Self { nodes: HashMap::new(), counter: 1}
   } 
}

impl Scope {
    pub fn push(&mut self, node : NodeType) -> usize {
        self.nodes.insert(self.counter, node);
        self.counter = self.counter + 1;
        return self.counter - 1
    }
}

#[derive(Debug)]
pub enum NodeType {
    Value(String),
    Nested(
        Box<NodeType>, // command
        usize,  // id
        usize, //  parent
        Vec<usize> // childern
    )
}
