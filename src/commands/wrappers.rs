use crate::{structs::{Globals, Type, ERROR, GError, Stack, Pass}, util::is_destination, gerr};

use super::{
    calculations::{*, op},
    variables::*,
    scopes::*,
    prints::{post, print, input, inputcast},
};

pub fn set_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    set(args, pass.0)
}
pub fn release_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    release(args, pass.0)
}

pub fn reset_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    reset(pass.0)
}



pub fn cal_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    cal(args, pass.0)
}
pub fn op_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    op(args, pass.0)
}


pub fn print_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    print(args, pass.0)
}
pub fn post_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    post(pass.0)
}

pub fn input_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    input(args, pass.0)
}

pub fn inputcast_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    inputcast(args, pass.0)
}

pub fn ifcommand_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    ifcommand(args, pass.0, pass.2, pass.1)
}

pub fn whilecommand_w(args : Vec<Type>, pass : Pass) -> Result<Type, ERROR> {
    whilecommand(args, pass.0, pass.2, pass.1)
}
