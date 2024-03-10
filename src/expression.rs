use crate::{gerr, get_first_and_last, structs::{GError, NodeType, Type, ERROR}};



#[derive(Debug, Clone)]
pub enum Expr {
    Command(Box<Expr>, Vec<Box<Expr>>),
    Const(Type),
    Carry(usize),
    RawVariable(String),
}

pub const REG_LEN : usize = 10;

pub struct Registries {
    pub len : usize,
    pub index : usize,
    pub set : Vec<Type>
}

impl Default for Registries {
    fn default() -> Self {
        Self {
            len : REG_LEN,
            index : 0,
            set : vec![Type::VOID();REG_LEN]
        }
    }
}


pub fn node_tree_to_exprs(tree : &NodeType) -> Result<Expr, ERROR> {

    match tree {
        NodeType::Value(val) => {
            let Some(name) = is_var(val) else {
                return Ok(Expr::Const(to_type(val)?))
            };

            Ok(Expr::RawVariable(name))
        },

        NodeType::Nested(cmd, args) => {
            let cmd = node_tree_to_exprs(cmd)?;

            let args : Vec<Box<Expr>> = args.iter().map(|a| { Box::new(node_tree_to_exprs(&a).unwrap()) }).collect();


            Ok(Expr::Command(Box::new(cmd), args))
        }
    }

}

pub fn flatten(expr : &Expr, v : &mut Vec<Expr>, reg : &mut Registries) -> Result<Expr, ERROR> {

    match expr {
        Expr::Command(cmd, args) => {

           let mut exprs = vec![cmd];
           for a in args { exprs.push(a) }
           
           let mut finals = vec![];
           for a in exprs {
               let tmp = flatten(a,v, reg)?;
               if let Expr::Command(_,_) = tmp {
                   finals.push(Expr::Carry(reg.index - 1))
               }
               else {
                   finals.push(tmp);
               };
           };
           reg.index = (reg.index + 1) % reg.len;
           let cmd = finals[0].clone();
           finals.remove(0);
           let ret = Expr::Command(Box::new(cmd), finals.iter().map(|f| Box::new(f.clone())).collect());
           v.push(ret.clone());
           Ok(ret)
        }
        _ => {Ok(expr.clone())}
    }

}

fn is_literal(s : &String) -> Option<String> {
    let s = s.trim();
    if s.starts_with("\"") && s.ends_with("\"") {
        if s.len() == 2 { return Some("".to_string()) }
        return Some(s[1..s.len()-2].to_string());
    }
    None
}

fn is_var(val : &String) -> Option<String> {
    if is_literal(val).is_some() || !val.trim().starts_with("$"){
        return None;
    }

    Some(val.trim()[1..].to_string())
}

fn to_type(value : &String) -> Result<Type, ERROR> {
    let val = value.to_string();
    match get_first_and_last(&val) {
        (Some('\"') , Some('\"')) => 
        {
            let val = snailquote::unescape(&val)?;
            return Ok(Type::STR(val))
        },
        (Some('\'') , Some('\'')) => {
            let val = snailquote::unescape(&val)?;
            let val = unescape::unescape(&val).unwrap();
            if let Ok(c) = val.parse::<char>() {
                return Ok(Type::CHAR(c));
            }
            return gerr!("Error: cannot parse [{val}] as char");
        },
        _ => {}
    }

    if value.trim_start().starts_with("{") && value.trim_end().ends_with("}") { 
        return Ok(Type::LIST(vec![]))
    }

    let s = snailquote::unescape(value)?.clone();
    if s.len() < 3 { return Ok(Type::STR(s)); }
    if !s.starts_with("~*") { return Ok(Type::STR(s)); }
    
    Ok(Type::VOID())
}
