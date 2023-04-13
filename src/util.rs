use std::{collections::HashMap, process::Child};

use crate::sturcts::{NodeType, Scope};


pub fn get_value(val : &String) -> String{
    val.clone()
}


pub fn balanced_braces(args: &[String], open: char, close: char) -> Vec<String> {
    let mut parts = Vec::new();

    for arg in args {
        if !arg.contains(open) {
            continue;
        }

        let mut chars = Vec::new();
        let mut n = 0;

        for c in arg.chars() {
            if c == open {
                if n > 0 {
                    chars.push(c);
                }
                n += 1;
            } else if c == close {
                n -= 1;
                if n > 0 {
                    chars.push(c);
                } else if n == 0 {
                    let part = chars.iter().collect::<String>().trim().to_string();
                    parts.push(part);
                    chars.clear();
                }
            } else if n > 0 {
                chars.push(c);
            }
        }
    }

    parts
}

pub fn make_tree(
    map : &mut Scope,
    line : &String
    ) {

    


    let balanced = balanced_braces(&[(*line).clone()], '[', ']');

    if balanced.is_empty() {
        map.push(NodeType::Value((*line).clone()));
        return;
    }

    let mut n = NodeType::Nested(Box::new(eval_node(&balanced[0])), map.counter, 0, vec![]);

    if let NodeType::Nested(_, id, _, ref mut children) = n {

        for i in 1..balanced.len() {
            let cid = map.push(eval_node(&balanced[i]));
            children.push(cid)
        }
    }
    map.push(n);


}

pub fn eval_node(node : &String) -> NodeType {

    let balanced = balanced_braces(&[(*node).clone()], '[', ']');

    if balanced.is_empty() {return NodeType::Value((*node).clone());}

    NodeType::Nested(Box::new(eval_node(&balanced[0])), 0, 0, vec![])
}
