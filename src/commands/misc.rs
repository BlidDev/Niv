use crate::{structs::{Type, Globals, GError, ERROR}, util::args_to_vars, gerr};

use std::{thread::sleep, time::Duration};

pub fn sleep_command(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR>{

    let args = args_to_vars(&args, &glb.stack)?;

    let Type::I32(ref delay) = &args[0] else {
        return gerr!("Error: Wrong argument given to [sleep]: {:?}", args[0]);
    };

    
    sleep(Duration::from_millis(*delay as u64));


    Ok(Type::VOID())
}
