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



    Ok(field.clone())
}



