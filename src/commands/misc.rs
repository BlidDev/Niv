use rand::{thread_rng, Rng};

use crate::{structs::{Type, Globals, GError, ERROR}, util::{args_to_vars, get_variable}, gerr, sgerr};

use std::{thread::sleep, time::Duration};

pub fn sleep_command(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR>{

    let args = args_to_vars(&args, &glb.stack)?;

    let Type::I32(ref delay) = &args[0] else {
        return gerr!("Error: Wrong argument given to [sleep]: {:?}", args[0]);
    };

    
    sleep(Duration::from_millis(*delay as u64));


    Ok(Type::VOID())
}

pub fn exit() -> ! {
    std::process::exit(0)
}

pub fn rng(args : Vec<Type>,glb : &mut Globals) -> Result<Type, ERROR>{

    let args = (get_variable(&args[0], &glb.stack)?, get_variable(&args[1], &glb.stack)?);
    sgerr!(
        (Type::I32(start), Type::I32(end)),
        args,
        "Error: wrong args in [rng]: [{:?}]", args
    );

    Ok(Type::I32(thread_rng().gen_range(start..end)))
}


