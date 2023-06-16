use device_query::Keycode;

use crate::{structs::{Type, ERROR, GError, Globals}, gerr, canvas::{Canvas, CanvasBuilder}, sgerr, util::args_to_vars, commands::variables::set};

pub fn key_pressed(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let args = args_to_vars(&args, &glb.stack)?;

    let key = {
        match &args[0] {

            Type::CHAR(ref c) => c.clone().to_string(),
            Type::STR(ref s) => s.clone(),
            _=> {return gerr!("Error: Wrong key argument in [key_pressed]: {:?}", args)}
        }
    };

    let code : Keycode =  key.clone().parse()?;

    if glb.keys.contains(&code) {
        return Ok(Type::BOOL(true));
    }

    Ok(Type::BOOL(false))
}
