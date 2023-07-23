use std::{collections::HashMap, fmt::{Display, Debug}, str::FromStr};
use device_query::{DeviceState, Keycode};

use crate::{gerr, canvas::Canvas, user_type::UserType};


//#[derive(Debug)]
//pub struct Node {
//    pub command : String,
//    pub id : usize,
//    pub parent : usize,
//    pub childern : Vec<usize>
//}


#[derive(Debug, Clone)]
pub enum NodeType {
    Value(String),
    Nested(
        Box<NodeType>, // command
        Vec<Box<NodeType>> // childern
    )
}

#[derive(Debug, Clone)]
pub enum Type {
    VOID(),
    I32(i32),
    F32(f32),
    BOOL(bool),
    CHAR(char),
    STR(String),
    UTYPE(UserType),
    NODE(Box<NodeType>),
    LIST(Vec<Type>)
}


impl Display for Type {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::VOID() => write!(f, "()"),
            Self::I32(a) => write!(f, "{a}"),
            Self::F32(a) => write!(f, "{a}"),
            Self::BOOL(a)=> write!(f, "{a}"),
            Self::CHAR(a)=> write!(f, "{a}"),
            Self::STR(a) => write!(f, "{a}"),
            Self::UTYPE(a) => {
                write!(f, "{}[", a.type_name)?;
                for (i, fld) in a.field_order.iter().enumerate() {
                    let tmp = a.fields.get(fld).unwrap();
                    write!(f, "{} : ", fld)?;
                    match tmp {
                        Type::STR(s)  => write!(f, "\"{}\"", s)?, 
                        Type::CHAR(c) => write!(f, "\'{}\'", c)?, 
                        _=> write!(f, "{}", tmp)?
                    }

                    if i < a.field_order.len() -1 { write!(f, ", ")?; }
                }

                write!(f, "]")
            },
            Self::NODE(a) => write!(f, "{a:?}"),
            Self::LIST(l) => write!(f, "{l:?}"),
       }
   } 
}



impl Type {
    pub fn dis(&self) -> usize {
        match self {
           Self::VOID()   => 0,
           Self::I32(_)   => 1, 
           Self::F32(_)   => 2, 
           Self::BOOL(_)  => 3, 
           Self::CHAR(_)  => 4, 
           Self::STR(_)   => 5, 
           Self::UTYPE(_) => 6, 
           Self::NODE(_)  => 7, 
           Self::LIST(_)  => 8, 
        }
    } 

    pub fn to_string(&self) -> Result<String, ERROR> {
        match self {
           Self::VOID() => Ok(String::from("()")),
           Self::I32(v) => Ok(v.to_string()),
           Self::F32(v) => Ok(v.to_string()),
           Self::BOOL(v)=> Ok(v.to_string()),
           Self::CHAR(v)=> Ok(v.to_string()),
           Self::STR(s) => Ok(s.clone()), 
           Self::LIST(l) => Ok(format!("{l:?}")),
           _ => gerr!("Error: Cannot turn [{:?}] into String", self),
        }
    }
}

pub fn parse_type(value : &String, glb : &Globals) -> Result<Type, ERROR>{
    if let Ok(i) = value.parse::<i32>() {
        return Ok(Type::I32(i));
    } else if let Ok(f) = value.parse::<f32>() {
        return Ok(Type::F32(f));
    } else if let Ok(b) = value.parse::<bool>() {
        return Ok(Type::BOOL(b));
    }
    if value.trim_start().starts_with("'") && value.trim_end().ends_with("'") {
        let mut v = value.clone(); 
        v.pop();
        v.remove(0);
        if let Ok(c) = v.parse::<char>() {
            return Ok(Type::CHAR(c));
        }
    }

    let mut s = value.clone();//snailquote::unescape(value)?.clone();
    if s.len() < 3 { return Ok(Type::STR(s)); }
    if !s.starts_with("~*") { return Ok(Type::STR(s)); }
    s = s[2..].to_string();

    if s.trim_start().starts_with("{") && s.trim_end().ends_with("}") { 
        let mut lst  = vec![];
        let (start, end) = (
            s.find("{").unwrap() + 1, 
            s.rfind("}").unwrap()
            );
        if end < start { return gerr!("Error: tyring to parse invalid list string: [{}]", s); }
        let s = s.get(start..end).expect(&format!("Error: tyring to parse invalid list string: [{}]", s));

        for element in s.split(",") {
            lst.push(parse_type(&element.trim().to_string(), glb)?);
        }

        return Ok(Type::LIST(lst))
    }

    if glb.registered_types.get(&s).is_none() { return Ok(Type::STR(s)); }
    
    Ok(Type::UTYPE(glb.registered_types.get(&s).unwrap().clone()))
}

#[allow(dead_code)]
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum TypeIndex {
    VOID = 0,
    I32,
    F32 ,
    BOOL,
    CHAR,
    STR ,
    UTYPE,
    NODE, 
}

impl FromStr for TypeIndex {
    type Err = ERROR;

   fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s.to_uppercase().as_str() {
        "VOID" => return Ok(Self::VOID),
        "I32"  => return Ok(Self::I32),
        "F32"  => return Ok(Self::F32),
        "BOOL" => return Ok(Self::BOOL),
        "CHAR" => return Ok(Self::CHAR),
        "STR"  => return Ok(Self::STR),
        "UTYPE" => return Ok(Self::UTYPE),
        "NODE" => return Ok(Self::NODE),
        _ => gerr!("Error: Could not parse [{}] as TypeIndex", s)
       }
   } 
}

#[derive(Debug)]
pub struct GError {
    message : String
}
impl GError {
    pub fn make(msg : &str) -> Self {
        GError { message: msg.to_string()}
    }
}

impl Display for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ERROR: {}\n",self.message))
    }
}

impl std::error::Error for GError {}

pub type ERROR = Box<dyn std::error::Error>;

pub struct QueryW(pub CommandQuery);
pub type Command = fn(Vec<Type>, &Roots, &mut Globals, &Scope, &QueryW, &mut Option<Canvas>) -> Result<Type, ERROR>;
pub type CommandQuery = HashMap<String, (Option<usize>, Command)>;
pub type Stack = HashMap<String, Type>;


#[derive(Debug, Default)]
pub struct Scope {
    pub nodes : Vec<Option<NodeType>>,
    pub children : HashMap<usize, Scope>
}

pub struct Globals<'a> {
    pub stack : Stack,
    pub curr : usize,
    pub s : &'a str,
    pub device_state : DeviceState,
    pub keys : Vec<Keycode>,
    pub canvas_should_close : bool,
    pub args_list : HashMap<String, Vec<String>>,

    pub registered_types : HashMap<String, UserType>,
}
pub type Roots = HashMap<String, Scope>;
