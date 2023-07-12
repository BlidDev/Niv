use std::collections::HashMap;

use crate::{structs::{Type, Globals, ERROR, GError, parse_type}, gerr};



#[derive(Debug, Clone)]
pub struct UserType {
    pub feilds    : HashMap<String,Type>,
    pub field_order : Vec<String>,
    pub type_name : String
}


pub fn find_user_types(lines : &Vec<String>) -> 
    Result<HashMap<String, (usize, usize)>, ERROR>
{
    let mut map : HashMap<String, (usize, Option<usize>)>= HashMap::new();


    for (i, line) in lines.iter().enumerate() {
        let Some('@') = line.chars().next() else {
            continue;
        };

        let mut chars = line.chars();
        chars.next();

        let name = chars.as_str().to_string();
        if let Some(ref mut scope) = map.get_mut(&name) {
            scope.1 = Some(i);
            continue;
        }

        map.insert(name.clone().trim_end().to_string(), (i, None));
    }

    let mut rmap = HashMap::new();
    for (name, (s, e)) in map {
        if e.is_none() {
            return gerr!("Error: could not find closing [@{}]", name);
        }
        rmap.insert(name, (s,e.unwrap()));
    }

    Ok(rmap)
}


pub fn range_to_user_type(
    lines : &Vec<String>,
    (start, end) : &(usize, usize),
) -> Result<UserType, ERROR> {
    let mut fields = HashMap::new();
    let mut field_order = vec![];

    for i in *start+1..*end {
        let mut line = lines[i].clone();
        line = line.trim().to_string();
        if line.is_empty() { continue; }

        let tmp = line.split(":").map(|s| s.trim().to_string()).collect::<Vec<String>>();
        let [name, val] = tmp.as_slice()
        else {
            return gerr!("Error: invalid line in user type [{}]", line);
        };

        if field_order.contains(name) {
            return gerr!("Error: field [{}] is defined more than once in [{}]", name, lines[*start]);
        }

        fields.insert(name.clone(), Type::STR(val.clone()));
        field_order.push(name.clone());

    }

    Ok(UserType { feilds : fields, field_order, type_name : "".to_owned()})
}


pub fn register_type(name : String, user_type : UserType, glb : &mut Globals) -> Result<(), ERROR> {
    if glb.registered_types.get(&name).is_some() {
        return gerr!("Error: user type [{:?}] already exists", name);
    };

    let mut user_type = user_type.clone();
    user_type.type_name = name.clone();


    glb.registered_types.insert(
        name,
        user_type
    );

    Ok(())
}

pub fn register_types(lines : &Vec<String>, glb : &mut Globals) -> Result<(), ERROR> {

    let ranges = find_user_types(lines)?;
    for (name, range) in ranges {
        let user_type = range_to_user_type(lines, &range)?;
        register_type(name, user_type, glb)?;
    }

    let mut registered_types = glb.registered_types.clone();

    for _ in 0..200 {
        for (_, t) in registered_types.iter_mut() {
            for (_, f) in t.feilds.iter_mut() {
                if let Type::STR(s) = f {
                    *f = parse_type(&s, glb)?;
                }
                else if let Type::UTYPE(ut) = f {
                    let mut s = "~*".to_string();
                    s.extend(ut.type_name.chars());
                    *f = parse_type(&s, glb)?;
                }

            }
        }

        glb.registered_types = registered_types.clone();
    }

    Ok(())
}

// [set][obj][[make][type]...]
