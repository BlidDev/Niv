use crate::{structs::{Type, ERROR, Globals, GError}, gerr, util::args_to_vars};


pub fn make_type(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let args = args_to_vars(&args, &glb.stack)?;
     if args.len() < 1 { return gerr!("Error: [makes] requires at least 1 args but [{}] were provided", args.len()); }

     let type_name = match &args[0] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid user_type type name [{:?}] given to [make] ", args[0])
     };

    let template = glb.registered_types.get(&type_name).expect(&format!("Error: could not find user_type type [{}]",type_name));

    if template.fields.len() != args.len() - 1 {
        return gerr!("Error: user type object of type [{}] requires [{}] arguments in order to be created but [{}] were provided", type_name, template.fields.len(), args.len() - 1);
    }

    let mut user_type = template.clone();


    for (i,field) in user_type.field_order.iter().enumerate() {
        *user_type.fields.get_mut(field).expect(&format!("Error: could not find field [{}] in user_type object of type [{}]", field, user_type.type_name)) = args[i+1].clone();
    }



    Ok(Type::UTYPE(user_type))
}


// [setf][obj][field][val]

pub fn setf(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

     let name = match &args[0] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid user_type object name [{:?}] given to [setf] ", args[0])
     };


     let fname = match &args[1] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid user_type field name [{:?}] given to [setf] ", args[0])
     };

     let Some(obj) = glb.stack.get_mut(&name) else {
         return gerr!("Error: could not find object [{}] in stack", name);
     };

     let Type::UTYPE(obj) = obj else {
         return gerr!("Error: tyring to run [setf] on incompatible type [{:?}]", obj);
     };


     if !obj.fields.contains_key(&fname) {
         return gerr!("Error: user_type object of type [{}] does not have a field called [{}]"
             ,obj.type_name, fname);
     }

    let field = obj.fields.get_mut(&fname).unwrap();
    *field = args[2].clone();



    Ok(args[2].clone())
}



pub fn getf(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

     let name = match &args[0] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid user_type object name [{:?}] given to [getf] ", args[0])
     };


     let fname = match &args[1] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid user_type field name [{:?}] given to [setf] ", args[0])
     };

     let Some(obj) = glb.stack.get_mut(&name) else {
         return gerr!("Error: could not find object [{}] in stack", name);
     };

     let Type::UTYPE(obj) = obj else {
         return gerr!("Error: tyring to run [getf] on incompatible type [{:?}]", obj);
     };


     if !obj.fields.contains_key(&fname) {
         return gerr!("Error: user_type object of type [{}] does not have a field called [{}]"
             ,obj.type_name, fname);
     }

    let field = obj.fields.get_mut(&fname).unwrap();



    Ok(field.clone())
}



pub fn getf_c(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

    let Type::LIST(lst) = &args[0] else {
        return gerr!("Error: invalid name list given to [getf_c]: [{:?}]", args[0]);
    };

    if lst.len() < 2 {
        return gerr!("Error: list given to [getf_c] is too short: [{:?}]", lst);
    }
    
    let name = match &lst[0] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid user_type object name [{:?}] given to [getf_c]", lst[0])
    };


    let fname = match &lst[1] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid user_type field name [{:?}] given to [getf_c]", lst[1])
    };

    let Some(obj) = glb.stack.get_mut(&name) else {
        return gerr!("Error: could not find object [{}] in stack", name);
    };

    let Type::UTYPE(obj) = obj else {
        return gerr!("Error: tyring to run [getf_c] on incompatible type [{:?}]", obj);
    };


    if !obj.fields.contains_key(&fname) {
        return gerr!("Error: user_type object of type [{}] does not have a field called [{}]"
            ,obj.type_name, fname);
    }

    let mut field = obj.fields.get_mut(&fname).unwrap();

    for f in lst[2..].iter() {

        let Type::UTYPE(obj) = field else {
            return gerr!("Error: tyring to run [getf_c] on incompatible type [{:?}]", obj);
        };

        let fname = match f {

           Type::CHAR(ref c) => c.clone().to_string(),
           Type::STR(ref s) => s.clone(),
            _ => return gerr!("Error: invalid user_type field name [{:?}] given to [getf_c] ", args[0])
        };

        if !obj.fields.contains_key(&fname) {
            return gerr!("Error: user_type object of type [{}] does not have a field called [{}]"
                ,obj.type_name, fname);
        }

        field = obj.fields.get_mut(&fname).unwrap();
    }


    Ok(field.clone())
}

pub fn setf_c(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

    let Type::LIST(lst) = &args[0] else {
        return gerr!("Error: invalid name list given to [setf_c]: [{:?}]", args[0]);
    };
    if lst.len() < 2 {
        return gerr!("Error: list given to [setf_c] is too short: [{:?}]", lst);
    }
    
    let name = match &lst[0] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid user_type object name [{:?}] given to [setf_c] ", args[0])
    };


    let fname = match &lst[1] {

       Type::CHAR(ref c) => c.clone().to_string(),
       Type::STR(ref s) => s.clone(),
        _ => return gerr!("Error: invalid user_type field name [{:?}] given to [setf_c]", lst[1])
    };

    let Some(obj) = glb.stack.get_mut(&name) else {
        return gerr!("Error: could not find object [{}] in stack", name);
    };

    let Type::UTYPE(obj) = obj else {
        return gerr!("Error: tyring to run [getf_c] on incompatible type [{:?}]", obj);
    };


    if !obj.fields.contains_key(&fname) {
        return gerr!("Error: user_type object of type [{}] does not have a field called [{}]"
            ,obj.type_name, fname);
    }

    let mut field = obj.fields.get_mut(&fname).unwrap();

    for f in lst[2..].iter() {

        let Type::UTYPE(obj) = field else {
            return gerr!("Error: tyring to run [setf_c] on incompatible type [{:?}]", obj);
        };

        let fname = match f {

           Type::CHAR(ref c) => c.clone().to_string(),
           Type::STR(ref s) => s.clone(),
            _ => return gerr!("Error: invalid user_type field name [{:?}] given to [setf_c]", f)
        };

        if !obj.fields.contains_key(&fname) {
            return gerr!("Error: user_type object of type [{}] does not have a field called [{}]"
                ,obj.type_name, fname);
        }

        field = obj.fields.get_mut(&fname).unwrap();
    }

    *field = args[1].clone();
    Ok(args[1].clone())
}
