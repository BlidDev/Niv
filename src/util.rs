use std::{collections::HashMap, error::Error};
//use framework::{canvas::canvas::Canvas, sdl2::context::Context};

use crate::{structs::{NodeType, Type, parse_type, CommandQuery, Command, GError, ERROR, Scope, Stack, Globals, QueryW, Roots}, gerr, canvas::Canvas, commands::scopes::run};


#[allow(dead_code)]
pub fn get_value(val : &String) -> String{
    val.clone()
}



/*pub fn make_tree(
    line : &String,
    first : bool
    ) -> NodeType {


    let line = line.clone();
    let tmp = line.trim();
    if tmp.starts_with("\"") && tmp.ends_with("\"") && first {
        let line = snailquote::unescape(&line.clone()).unwrap();
        return NodeType::Value(line.clone());
    }
    


    let balanced = balanced_braces(&[line.clone()], '[', ']');

    if balanced.is_empty() {
        let line = unescape::unescape(&line).unwrap();
        return NodeType::Value(line)
    }

    let mut n = NodeType::Nested(Box::new(make_tree(&balanced[0], true)), vec![]);

    if let NodeType::Nested(_, ref mut childern) = n {

        for i in 1..balanced.len() {
            childern.push(Box::new(make_tree(&balanced[i], true)))
        }
    }
}*/

pub fn smart_split(line : &String) -> Result<Vec<String>, ERROR> {

    let mut v = vec![];
    if line.is_empty() { return Ok(v); }

    let mut brackeys = 0;
    let mut lists = 0;
    let mut instr = false;
    let mut i = 0;

    let mut tmp = "".to_string();

    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        let is_empty = tmp.trim().is_empty();

        match (c, instr, lists == 0, brackeys == 0, c.is_whitespace(), is_empty) {
            (_, false, true, true, true, false)=> { // should push word
                v.push(tmp.clone().trim().to_string());
                tmp.clear();
            },
            ('[', false, true, ..) => { brackeys +=1; tmp.push(c); },
            (']', false, true, ..) => { brackeys -=1; tmp.push(c); },
            ('\"' | '\'', _, true, true, ..) => {
                if !is_escape(i, line) {
                    instr = !instr;
                }
                tmp.push(c);
            },
            ('{', false, true, true, ..) => {
                lists += 1;
                tmp.push(c)
            },
            ('}', false, false, true, ..) => { lists -= 1; tmp.push(c);},
            (c, ..) => tmp.push(c),
        }
        i += 1;
    }
    if brackeys != 0 || lists != 0 || instr {
        return gerr!("Error: trying to parse invalid line: {:?}",line)
    }
    if !tmp.trim().is_empty() { v.push(tmp.trim().to_string()); }



    Ok(v)
}



fn is_escape(index : usize, line : &String) -> bool {
    if index < 2 { return false }
    if line.chars().nth(index - 1).unwrap() == '\\' {
        return true
    }
    false
}

pub fn is_nested(line : &String) -> Option<String> {
    let mut line = line.trim().to_string();
    if line.starts_with("[") && line.ends_with("]") {
        line.pop(); line.remove(0);
        return Some(line);
    }

    None
}



pub fn make_tree(line : &String, first : bool) -> Result<NodeType, ERROR> {

    let trimmed : String = {
        let s : String;
        match is_nested(line) {
            Some(trimmed) => s = trimmed,
            _ => {
                if first { s = line.clone() }
                else {return Ok(NodeType::Value(line.clone()))}
            },
        }
        s
    };

    let splits = smart_split(&trimmed)?;
    if splits.is_empty() { 
        return gerr!("Error: cannot parse empty node ()")
    }

    let mut n = NodeType::Nested(Box::new(make_tree(&splits[0], false)?), vec![]);

    if let NodeType::Nested(_, ref mut childern) = n {

        for i in 1..splits.len() {
            childern.push(Box::new(make_tree(&splits[i], false)?))
        }
    }

    Ok(n)
}

pub fn traverse(node : &NodeType, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
    ) -> Result<Type, Box<dyn std::error::Error>> {
    match node {
        NodeType::Value(value) => {
            let o = Ok(parse_type(value, roots, query, glb, scope, cnv)?);
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

            if "chain" == name {
                let v : Vec<Type> = childern.iter().map(|e| Type::NODE(e.clone())).collect();
                return run_command(roots, query, &name, v, glb, scope, cnv)
            }

            if name == "singleif" {
                let mut args : Vec<Type> = vec![];
                let offset = match childern.len() {
                    4 => 1,
                    5 => 2,
                    _ => return gerr!("Error: [singleif] takes [4] or [5] arguments but [{}] were provided"
                , childern.len())
                };
                for i in 0..childern.len() - offset  {
                    args.push(traverse(&childern[i], roots, query, glb, scope, cnv)?);
                }
                for i in 1..offset+1 {
                    args.push(Type::NODE(childern.get(2 + i).unwrap().clone()));
                }
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
            if name == "else" { return Ok(Type::VOID()) }
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
    Ok(make_scope_tree(&lines, &mut indent)?.0)
}


pub fn make_scope_tree(lines : &Vec<String>, indent : &mut usize) -> Result<(Scope, usize), ERROR>{
    let mut i = 0;
    let mut scope = Scope::default();
    

    while  i < lines.len() {
        let first = lines[i].chars().next();
        //println!("{}{i} - {first:?}", "   ".repeat(*indent));
        match first {
            Some('{') => {
                *indent+=1;
                let (sc, amount) = make_scope_tree(&lines[i+1..].to_vec(), indent)?;
                *indent = (*indent as i32 - 1).max(0) as usize;
                scope.children.insert(i, sc);
                for  _ in 0..amount { scope.nodes.push(None);}
                i += amount;
            },
            Some('}') => {
                return Ok((scope, i+2));
            }
            _ => {
                if smart_split(&lines[i])?.is_empty() {
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
                scope.nodes.push(Some(make_tree(&lines[i].clone(), true)?));
                i+=1;
            },
        }
    }

    Ok((scope, i))
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
) -> Result<Type, ERROR> {
    let s = roots.get(name).expect(&format!("Error: Root scope [{}] not found", name));

    let t = traverse_scope(roots, s, query, glb, cnv)?;

    match t {
        Type::RETURN(v) => Ok(*v),
        _ => Ok(Type::VOID())
    }
}


pub fn traverse_scope(roots : &Roots, scope : &Scope, query : &QueryW, glb : &mut Globals,
    cnv : &mut Option<Canvas>
    ) -> Result<Type, ERROR> {
    let mut i = 0;

    while i < scope.nodes.len() {
        if let Some(ref node) = scope.nodes[i] {
            let t= traverse(node, roots, query, glb, scope, cnv)?;
            if let Type::RETURN(_) = t{
                //i+=1;
                //glb.curr = i;
                return Ok(t);
            }
        }
        i+=1;
        glb.curr = i;
    }

    Ok(Type::VOID())
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




pub fn get_first_and_last(value : &String) -> (Option<char>, Option<char>) {
    let mut chars =value.chars();
    (chars.nth(0), chars.last())
}

pub fn remove_first_and_last(value : &String) -> Result<String, ERROR> {
    if value.len() < 2 {
        return gerr!("Error trying to remove first and last chars from too short string: [{}]",value);
    }
    let mut val = value.trim().to_owned();
    val.remove(0); val.pop();

    Ok(val)
}


pub fn parse_list(value : &String, roots : &Roots,query : &QueryW, glb : &mut Globals, scope : &Scope,
    cnv : &mut Option<Canvas>
) -> Result<Type, ERROR> {
    let mut v = vec![];

    let splits = smart_split(&value)?;

    for split in splits {
        let element = 
            match is_nested(&value) {
                Some(trimmed) => {
                    println!("caugth: {}", trimmed);
                    traverse(&make_tree(&trimmed, true)?, roots, query, glb, scope, cnv)?
                },
                None => parse_type(&split, roots, query, glb, scope, cnv)?
            };
        v.push(element);
    }


    Ok(Type::LIST(v))
}
