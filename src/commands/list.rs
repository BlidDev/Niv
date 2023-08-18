use crate::{structs::{Type, ERROR, Globals, GError}, sgerr, util::{args_to_vars, get_variable}, gerr};


pub fn gete(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args,&glb.stack)?;


    let index = &args[1];

    sgerr!(
        Type::I32(ref index),
        index,
        "Error: invalid index given in [gete]: [{:?}]", index
    );

    let name = match &args[0] {
        Type::LIST(ref v) => return Ok(v[index.clone() as usize].clone()),
        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [gete] ", args[0])
    };


    if let Some(l) = glb.stack.get_mut(&name) {
        match l {

            Type::LIST(l) => {
                if *index < 0 || *index as usize >= l.len() {
                    return gerr!("Error: index given to [gete] is out of range: [{}]. Current range is: (0..{})", index,l.len());
                }
                Ok(l.get(*index as usize).unwrap().clone())
            },
            Type::STR(s) => {
                if *index < 0 || *index as usize >= s.len() {
                    return gerr!("Error: index given to [gete] is out of range: [{}]. Current range is: (0..{})", index,s.len());
                }
                Ok(Type::CHAR(s.chars().nth(*index as usize).unwrap()))
            },
            _ => return gerr!("Error: [{}] is not a list nor string but a [{:?}]", name, l)
        }
    }
    else{
        if *index < 0 || *index as usize >= name.len() {
            return gerr!("Error: index given to [gete] is out of range: [{}]. Current range is: (0..{})", index,name.len());
        }
        Ok(Type::CHAR(name.chars().nth(*index as usize).unwrap()))
    }


}

pub fn sete(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args,&glb.stack)?;


    let name = match &args[0] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [sete] ", args[0])
    };

    let index = &args[1];

    sgerr!(
        Type::I32(ref index),
        index,
        "Error: invalid index given in [getf]: [{:?}]", index
    );


    let Some(l) = glb.stack.get_mut(&name) else {
        return gerr!("Error: [{}] does not exist", name);
    };

    match l {

        Type::LIST(l) => {
            if *index < 0 || *index as usize >= l.len() {
                return gerr!("Error: index given to [gete] is out of range: [{}]. Current range is: (0..{})", index,l.len());
            }
            let val = args[2].clone();

            let element = l.get_mut(*index as usize).unwrap();

            *element = val.clone();

            Ok(val)
        },
        Type::STR(s) => {
            if *index < 0 || *index as usize >= s.len() {
                return gerr!("Error: index given to [gete] is out of range: [{}]. Current range is: (0..{})", index,s.len());
            }

            let c =match args[2].clone() {

                Type::STR(s) => {
                    if s.is_empty() { return gerr!("Error: empty value given to string in [sete]") }
                    s.chars().nth(0).unwrap()
                },
                Type::CHAR(v) => v,
                _ => return gerr!("Error: trying to insert non char value into [{}] in [sete]: [{:?}]"
                    ,name,args[2])
            };


            let index = *index as usize;
            s.replace_range(index..index+1, &c.to_string());
            Ok(Type::CHAR(c))
        },
        _ => return gerr!("Error: [{}] is not a list nor string but a [{:?}]", name, l)
    }


}


pub fn list_clear(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR>{
    let name = get_variable(&args[0], &glb.stack)?;
    let name = match name {
       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [lclear] ", args[0])
    };
    let Some(l) = glb.stack.get_mut(&name) else {
        return gerr!("Error: [{}] does not exist", name);
    };

    match l {
        Type::LIST(l) => l.clear(),
        Type::STR(s)  => s.clear(),
        _ => return gerr!("Error: [{}] is not a list nor string but a [{:?}]", name, l)
    }


    Ok(Type::VOID())
}

pub fn list_len(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR>{
    let name = get_variable(&args[0], &glb.stack)?;
    let name = match name {
       Type::LIST(ref v) => return Ok(Type::I32(v.len() as i32)),
       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [llen] ", args[0])
    };
    if let Some(l) = glb.stack.get_mut(&name) {
        match l {
            Type::LIST(l) => Ok(Type::I32(l.len() as i32)),
            Type::STR(s) => Ok(Type::I32(s.len() as i32)),
            _ => gerr!("Error: [{}] is not a list nor string but a [{:?}]", name, l)
        }
    }
    else {Ok(Type::I32(name.len() as i32))}


}

pub fn list_remove(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args,&glb.stack)?;


    let name = match &args[0] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [lremove] ", args[0])
    };

    let index = &args[1];

    sgerr!(
        Type::I32(ref index),
        index,
        "Error: invalid index given in [lremove]: [{:?}]", index
    );


    let Some(l) = glb.stack.get_mut(&name) else {
        return gerr!("Error: [{}] does not exist", name);
    };

    match l {
        Type::LIST(l) => {
            if *index < 0 || *index as usize >= l.len() {
                return gerr!("Error: index given to [lremove] is out of range: [{}]. Current range is: (0..{})", index,l.len());
            }

            Ok(l.remove(*index as usize))
        },
        Type::STR(s) => {
            if *index < 0 || *index as usize >= s.len() {
                return gerr!("Error: index given to [lremove] is out of range: [{}]. Current range is: (0..{})", index,s.len());
            }

            Ok(Type::CHAR(s.remove(*index as usize)))
        },
        _ => return gerr!("Error: [{}] is not a list but a [{:?}]", name, l)
    }

}


pub fn list_push(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args,&glb.stack)?;


    let name = match &args[0] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [lpush] ", args[0])
    };

    let val = &args[1];


    let Some(l) = glb.stack.get_mut(&name) else {
        return gerr!("Error: [{}] does not exist", name);
    };

    match l {
        Type::LIST(l) => {
            l.push(val.clone());
            Ok(val.clone())
        },
        Type::STR(s) => {
            let c = match val.clone() {
                Type::CHAR(c) => c,
                Type::STR(s) => {
                    if s.is_empty() { 
                        return gerr!("Error: trying to pass empty string as value in [lpush] on string [{}]", name);
                    }
                    s.chars().nth(0).unwrap()
                },
                _ => return gerr!("Error: trying to push non char value to [{}]: [{:?}]",name, val)
            };
            s.push(c.clone());
            Ok(val.clone())
        },
        _ => return gerr!("Error: [{}] is not a list nor string but a [{:?}]", name, l)
    }

}

pub fn list_pop(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args,&glb.stack)?;


    let name = match &args[0] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid list name [{:?}] given to [lpop] ", args[0])
    };

    let Some(l) = glb.stack.get_mut(&name) else {
        return gerr!("Error: [{}] does not exist", name);
    };

    match l {
        Type::LIST(l) => {
            if l.len() == 0 { return gerr!("Error: trying to run [pop] on already empty list [{}]", name); }

            Ok(l.pop().unwrap())
        },
        Type::STR(s) => {
            if s.len() == 0 { return gerr!("Error: trying to run [pop] on already empty string [{}]", name); }

            Ok(Type::CHAR(s.pop().unwrap()))
        },
        _ => return gerr!("Error: [{}] is not a list but a [{:?}]", name, l)
    }

}

pub fn list_empty() -> Result<Type, ERROR>{
    Ok(Type::LIST(Vec::new()))
}

pub fn repeat(args : Vec<Type>, glb : &mut Globals) -> Result<Type,ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

    let times = args[1].clone();

    sgerr!(
        Type::I32(times),
        times,
        "Error: invalid multiplier given to [repeat]: [{:?}]"
        , times
    );


    Ok(Type::LIST(vec![args[0].clone(); times as usize]))
}

pub fn repeatl(args : Vec<Type>, glb : &mut Globals) -> Result<Type,ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(ref times), Type::LIST(ref list)),
        (&args[1], &args[0]),
        "Error: invalid arguments given to [repeatl]: [{:?}]"
        , args
    );


    let list = {
        let mut v = vec![];
        for _ in 0..*times {
            for e in list.clone(){
                v.push(e);
            }
        }
        v
    };

    Ok(Type::LIST(list))
}
