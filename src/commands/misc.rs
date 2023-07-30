use rand::{thread_rng, Rng};

use crate::{structs::{Type, Globals, GError, ERROR, TypeIndex, Timer}, util::{args_to_vars, get_variable}, gerr, sgerr};

use std::{thread::sleep, time::{Duration, Instant}};

use super::{prints::format_command, variables::set};

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

pub fn err_msg(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let fmt = format_command(args, glb)?;
    
    let Type::STR(format) =  fmt else {
        return gerr!("Error: [format] returned an invalid value: [{:?}]", fmt)
    };

    gerr!("{}", format)
}

pub fn typeid(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let var = get_variable(&args[0], &glb.stack)?;

    Ok(Type::I32(var.dis() as i32))
}

pub fn return_cmd(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let arg = get_variable(&args[0], &glb.stack)?;

    if arg.dis() == TypeIndex::RETURN as usize{
        return gerr!("Error: cannot run [return] with RETURN type: [{:?}]", arg);
    }
    
    return Ok(Type::RETURN(Box::new(arg)));
}


// new_timer timer_elapsed timer_millis timer_reset timer_set_delay end_timer


pub fn new_timer(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let delay = get_variable(args.first().unwrap(), &glb.stack)?;

    sgerr!(
        Type::I32(delay),
        delay,
        "Error: invalid delay argument given in [new_timer]: [{:?}]", delay
    );
    
    if delay < 0 {
        return gerr!("Error: invalid delay given in [timer_set_delay]: [{}]", delay);
    }

    let id = glb.timers
        .insert(Timer::new(delay as u32));

    Ok(Type::I32(id as i32))
}

pub fn timer_millis(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let id = get_variable(args.first().unwrap(), &glb.stack)?;

    sgerr!(
        Type::I32(id),
        id,
        "Error: invalid id given in [timer_millis]: [{:?}]", id
    );

    let Ok(entry) = glb.timers.get(id as usize) else {
        return  gerr!("Error: timer with id [{}] does not exist", id);
    };


    let millis = entry.clock.elapsed().as_millis() as i32;

    Ok(Type::I32(millis))
}



pub fn timer_elapsed(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let id = get_variable(args.first().unwrap(), &glb.stack)?;

    sgerr!(
        Type::I32(id),
        id,
        "Error: invalid id given in [timer_elapsed]: [{:?}]", id
    );

    let Ok(entry) = glb.timers.get(id as usize) else {
        return  gerr!("Error: timer with id [{}] does not exist", id);
    };


    let millis = entry.clock.elapsed().as_millis();

    let elapsed = millis >= entry.delay as u128; 

    Ok(Type::BOOL(elapsed))
}


pub fn timer_reset(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let id = get_variable(args.first().unwrap(), &glb.stack)?;

    sgerr!(
        Type::I32(id),
        id,
        "Error: invalid id given in [timer_reset]: [{:?}]", id
    );

    let Ok(entry) = glb.timers.get_mut(id as usize) else {
        return  gerr!("Error: timer with id [{}] does not exist", id);
    };

    entry.clock = Instant::now();

    Ok(Type::VOID())
}


pub fn timer_set_delay(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(id), Type::I32(delay)),
        (&args[0], &args[1]),
        "Error: invalid arguments given in [timer_set_delay]: [{:?}]", args
    );

    let Ok(entry) = glb.timers.get_mut(*id as usize) else {
        return  gerr!("Error: timer with id [{}] does not exist", id);
    };

    if *delay < 0 {
        return gerr!("Error: invalid delay given in [timer_set_delay]: [{}]", delay);
    }


    entry.delay = *delay as u32;


    Ok(Type::VOID())
}


pub fn end_timer(args : Vec<Type>, glb : &mut Globals) -> Result<Type, ERROR> {

    let args = args_to_vars(&args, &glb.stack)?;

     let name = match &args[0] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid timer type name [{:?}] given to [end_timer]", args[0])
     };

     let Some(Type::I32(id)) = glb.stack.get(&name) else {
         return gerr!("Error: invalid variable given to [end_timer]: [{:?}]", &args[0])
     };

    glb.timers.remove(*id as usize)?;


    set(vec![Type::STR(name), Type::VOID()], glb)
}
