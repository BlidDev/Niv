use std::{path::Path, fs::{File, OpenOptions}, io::Write};

use crate::{structs::{Type, Globals, GError, ERROR}, util::{args_to_vars, get_variable}, gerr, sgerr};
// V - openfile write readbuf close


pub fn openfile(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;


    sgerr!(
        (Type::STR(path), Type::CHAR(option)),
        (&args[0], &args[1]),
        "Error: invalid arguments given to [openfile]: [{:?}]", args
    );

    match option {
        'r' => {
            if glb.input_files.get(path).is_some() {return Ok(Type::VOID())}
            else if glb.output_files.get(path).is_some() {
                return gerr!("Error: the path [{}] is already open as an output file", path)
            }

            let path_o = Path::new(path);
            if !path_o.exists() || !path_o.extension().is_some() {
                return gerr!("Error: could not open path: [{}]", path);
            }

            glb.input_files.insert(path.clone());
        },
        'w' => {
            if glb.output_files.get(path).is_some() {return Ok(Type::VOID())}
            else if glb.input_files.get(path).is_some() {
                return gerr!("Error: the path [{}] is already open as an input file", path)
            }
            let path_o = Path::new(path);
            if path_o.exists() {
                let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)?;
                glb.output_files.insert(path.clone(), file);
                return Ok(Type::VOID())
            }

            glb.output_files.insert(path.clone(), File::create(path)?);
        },
        _ => return gerr!("Error: invalid option given to [openfile]: [{:?}]", option)
    }


    Ok(Type::VOID())
}


pub fn readbuf(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let arg = get_variable(&args[0], &glb.stack)?;

    sgerr!(
        Type::STR(path),
        arg,
        "Error: invalid path given to [readbuf]: [{:?}]", arg
    );

    let Some(entry) = glb.input_files.get(&path) else {
        if glb.output_files.get(&path).is_some() {
            return gerr!("Error: file path [{:?}] is opened as an output file, not an input one", path);
        }
        return gerr!("Error: file path [{:?}] hasn't been open yet", path);
    };

    let buffer = std::fs::read_to_string(entry)?;


    Ok(Type::STR(buffer))
}

pub fn writef(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::STR(path), Type::BOOL(flag)),
        (&args[0], &args[2]),
        "Error: invalid arguments given in [writef]: [{:?}]", args
    );

    let s = args[1].to_string()?;

    let Some(entry) = glb.output_files.get_mut(path) else {
        if glb.input_files.get(path).is_some() {
            return gerr!("Error: file path [{:?}] is opened as an input file, not an output one", path);
        }
        return gerr!("Error: file path [{:?}] hasn't been open yet", path);
    };

    if *flag {
        entry.set_len(0)?;
    }

    write!(entry, "{}", s)?;


    Ok(Type::VOID())
}

pub fn closef(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR>{

    let arg = get_variable(&args[0], &glb.stack)?;

    sgerr!(
        Type::STR(path),
        arg,
        "Error: invalid path given to [closef]: [{:?}]", arg
    );

    if !glb.input_files.remove(&path) && glb.output_files.remove(&path).is_none() {
        return gerr!("Error: file [{path}] has not been opened yet");
    }

    Ok(Type::VOID())
}
