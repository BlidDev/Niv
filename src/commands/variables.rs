use crate::{structs::{Globals, Type, ERROR, GError, Stack}, util::is_destination, gerr};


pub fn set(args : Vec<Type>, glb : &mut Globals) ->Result<Type, ERROR> {
    let name = is_destination(&args[0], &glb.stack, "set")?;
    let v = args[1].clone();

    //println!("Setting [{}] = [{:?}]", name, v.clone());
    *glb.stack.entry(name).or_insert(v.clone()) = v.clone();

    Ok(args[1].clone())
}



pub fn release(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let name = is_destination(&args[0], &glb.stack, "release")?;

    if glb.stack.get(&name).is_none() {
        return gerr!("Error: Variable [{name}] does not exist");
    }

    let value = glb.stack.remove(&name).unwrap();
    Ok(value)
}

pub fn reset(glb : &mut Globals) -> Result<Type, ERROR> {
    glb.stack = Stack::new();
    Ok(Type::VOID())
}
