use std::{collections::HashMap, error::Error};
//use framework::{canvas::canvas::Canvas, sdl2::context::Context};

use crate::{structs::{NodeType, Type, parse_type, CommandQuery, Command, GError, ERROR, Scope, Stack, Globals, QueryW, Roots}, gerr, canvas::Canvas, commands::scopes::run};


#[allow(dead_code)]
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
    line : &String
    ) -> NodeType {

    


    let balanced = balanced_braces(&[(*line).clone()], '[', ']');

    if balanced.is_empty() {
        return NodeType::Value((*line).clone());
    }

    let mut n = NodeType::Nested(Box::new(make_tree(&balanced[0])), vec![]);

    if let NodeType::Nested(_, ref mut childern) = n {

        for i in 1..balanced.len() {
            childern.push(Box::new(make_tree(&balanced[i])))
        }
    }

    n
}


pub fn traverse(node : &NodeType, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
    ) -> Result<Type, Box<dyn std::error::Error>> {
    match node {
        NodeType::Value(value) => {
            let o = Ok(parse_type(value, &glb)?);
            return o;
        }

        NodeType::Nested(command, childern) => {
            let command = traverse(command, roots, query, glb, scope, cnv)?;
            let Type::STR(ref name) = command else {
                return gerr!("Error: Command is [{command:?}] instead of STR");
            };

            if "while" == name {
                return run_command(roots,query, &name, vec![Type::NODE(childern[0].clone())], glb, scope, cnv)
            }

            if name == "singleif" {
                let mut args : Vec<Type> = vec![];
                for i in 0..childern.len() - 1  {
                    args.push(traverse(&childern[i], roots, query, glb, scope, cnv)?);
                }
                args.push(Type::NODE(childern.last().unwrap().clone()));
                return run_command(roots, query, &name, args, glb, scope, cnv)
            }

            let mut args : Vec<Type> = vec![];
            for child in childern  {
                args.push(traverse(&child, roots,  query, glb, scope, cnv)?);
            }

            //println!("{command:?} {args:?}\n");
            run_command(roots, query, &name, args, glb, scope, cnv)
        },
    }

}


pub fn add_command(
    query : &mut CommandQuery, 
    command : Command, 
    name : &str, 
    num_args : Option<usize>) {
    query.insert(
        name.to_string(), 
        (num_args,command));
        //.unwrap_or_else(||panic!("ERROR: Could not add command: [{}]", name));
}

pub fn run_command(roots : &Roots,query : &QueryW,name : &String, args: Vec<Type>, glb : &mut Globals, scope : &Scope, 
    cnv : &mut Option<Canvas>)
-> Result<Type, Box<dyn Error>>{

    let a = Type::STR(name.clone());
    let Type::STR(name) = get_variable(&a, &glb.stack)? else {
        return gerr!("Error: Trying to pass [{:?}] as a command", a);
    };

    let Some(command) = query.0.get(&name) else {
        if !roots.contains_key(&name) {
            return gerr!("ERROR: The command [{}] could not be found",name);
        };

        let mut tmp = vec![Type::STR(name)];
        for a in args {tmp.push(a.clone());}
        return run(tmp, roots, glb, query, cnv)
    };

    if let Some(limit) = command.0 {
        let len = args.len();
        if limit != len {
            return gerr!("ERROR: [{}] requires [{}] arguments but [{}] were supplied", name, limit, len);
        }
    }

    command.1(args, roots, glb, scope, &query, cnv)
}

pub fn get_variable(val : &Type, stack : &Stack) -> Result<Type, ERROR> {

    let Type::STR(name) = val else {
        return Ok((*val).clone());
    };

    if let Some('$') = name.chars().next() { // variable
        let name = name[1..].to_string();
        
        let Some(value) = stack.get(&name) else {
            return gerr!("Error: Variable [{}] does not exist", name);
        };

        Ok((*value).clone())
    }
    else {
        Ok(Type::STR((*name).clone()))
    }


}

pub fn args_to_vars(v : &Vec<Type>, stack : &Stack) -> Result<Vec<Type>, ERROR> {
    let mut n_v = vec![];
    for a in v {
        n_v.push(get_variable(a, stack)?)
    }

    Ok(n_v)
}
pub fn find_root_scopes(lines : &Vec<String>) -> 
    Result<HashMap<String, (usize, usize, Vec<String>)>, Box<dyn Error>>
{
    let mut map : HashMap<String, (usize, Option<usize>, Vec<String>)>= HashMap::new();


    for (i, line) in lines.iter().enumerate() {
        let Some('#') = line.chars().next() else {
            continue;
        };

        let mut chars = line.chars();
        chars.next();
        // Check for arg list

        if !chars.as_str().contains("|") {
            let name = chars.as_str().trim().to_string();
            if let Some(ref mut scope) = map.get_mut(&name) {
                scope.1 = Some(i);
                continue;
            }

            return gerr!("Error: No arguments \'|\' given in root scope [{}]",chars.as_str())
        }


        let (name, args) = {
            let s : Vec<String >= chars.as_str().split("|").map(|a| a.to_string()).collect();
            let n = s[0].to_string();
            let args : Vec<String>= s[1].split_whitespace().map(|a| a.to_string()).collect();


            (n, args)
        };
        //println!("{} {:?}", name, args);

        map.insert(name.clone().trim_end().to_string(), (i, None, args));
    }

    let mut rmap = HashMap::new();
    for (name, (s, e, v)) in map {
        if e.is_none() {
            return gerr!("Error: could not find closing [#{}]", name);
        }
        rmap.insert(name, (s,e.unwrap(), v));
    }

    Ok(rmap)
}

pub fn root_to_scope_tree(
    lines : &Vec<String>, 
    root : &(usize, usize)) -> Result<Scope, ERROR>
{

    let openers : usize = lines.iter().filter(|l| **l == "{".to_string()).count();
    let closers : usize = lines.iter().filter(|l| **l == "}".to_string()).count();
    if openers != closers {
        return gerr!("Error: Mismatching amount of \'{{'\' and \'}}\'");
    }

    let lines = lines[root.0+1..root.1].to_vec();
    
    let mut indent = 0;
    Ok(make_scope_tree(&lines, &mut indent).0)
}


pub fn make_scope_tree(lines : &Vec<String>, indent : &mut usize) -> (Scope, usize){
    let mut i = 0;
    let mut scope = Scope::default();
    

    while  i < lines.len() {
        let first = lines[i].chars().next();
        //println!("{}{i} - {first:?}", "   ".repeat(*indent));
        match first {
            Some('{') => {
                *indent+=1;
                let (sc, amount) = make_scope_tree(&lines[i+1..].to_vec(), indent);
                *indent = (*indent as i32 - 1).max(0) as usize;
                scope.children.insert(i, sc);
                for  _ in 0..amount { scope.nodes.push(None);}
                i += amount;
            },
            Some('}') => {
                return (scope, i+2);
            }
            _ => {
                if balanced_braces(&[lines[i].clone()], '[', ']').is_empty() {
                    scope.nodes.push(None);
                    i+=1;
                    continue;
                }
                if lines[i].len() >= 2 {
                    if &lines[i][..2] == "//"{
                        scope.nodes.push(None);
                        i+=1;
                        continue;
                    }
                }
                scope.nodes.push(Some(make_tree(&lines[i].clone())));
                i+=1;
            },
        }
    }

    (scope, i)
}

pub fn _print_scope_tree(scope : &Scope, indent : &mut usize, _begin : &usize) {
    for (begin, child) in scope.children.iter() {
        println!("{}[{}] - {}", "   ".repeat(*indent + 1), begin, child.nodes.len());
        *indent+=1;
        _print_scope_tree(child, indent, begin);
        *indent = (*indent as i32 - 1).max(0) as usize;
    }
}


pub fn traverse_root_scope(name : &str, roots : &Roots,query : &QueryW, glb : &mut Globals,
    cnv : &mut Option<Canvas>
) -> Result<(), ERROR> {
    let s = roots.get(name).expect(&format!("Error: Root scope [{}] not found", name));

    traverse_scope(roots, s, query, glb, cnv)?;

    Ok(())
}


pub fn traverse_scope(roots : &Roots, scope : &Scope, query : &QueryW, glb : &mut Globals,
    cnv : &mut Option<Canvas>
    ) -> Result<(), ERROR> {
    let mut i = 0;

    while i < scope.nodes.len() {
        if let Some(ref node) = scope.nodes[i] {
            traverse(node, roots, query, glb, scope, cnv)?;
        }
        i+=1;
        glb.curr = i;
    }

    Ok(())
}


pub fn is_destination(val : &Type, stack : &Stack, name : &str) -> Result<String, ERROR> {
    match get_variable(val, stack)? {
        Type::STR(name) => {return Ok(name.clone());},
        Type::CHAR(name) => {return Ok(name.to_string().clone());},
        _ => {
            return gerr!("Error: Invalid destination [{:?}] in [{name}]", val);
        }
    }
}


pub fn validate_root_scopes_names(roots : &Roots, query : &QueryW) -> Result<(), ERROR> {

    for (name, _) in roots.iter() {
        if query.0.contains_key(name) {
            return gerr!("Error: root scope name is [{}] which is a reserved keyword", name);
        }
    }

    Ok(())
}


pub fn remove_comments_from_lines(lines : &Vec<String>) -> Result<Vec<String>, ERROR> {
    let new_lines = 
        lines.iter()
        .map(|l| {
            let v : Vec<&str>= l.split("//").collect();
            v[0].to_string()
        })
        .map(|l| l.clone())
        .collect();
    Ok(new_lines)
}
