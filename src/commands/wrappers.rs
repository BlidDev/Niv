use std::rc::Rc;

use crate::{structs::{Globals, Type, ERROR, Scope, QueryW}, make_wrapper, make_wrappers};

use super::{
    calculations::{*, op},
    variables::*,
    scopes::*,
    prints::{post, print, input, inputcast},
    graphics::*
};


use unstringify::unstringify;
use framework::{canvas::canvas::Canvas, sdl2::context::Context};
use sdl2::{render::TextureCreator, video::WindowContext, pixels::Color, event::Event};
 
make_wrappers!(
    set_w ,set => ["args", "glb"],
    release_w, release => ["args","glb"],
    reset_w, reset => ["glb"],
    cal_w, cal => ["args", "glb"],
    op_w, op => ["args", "glb"],
    print_w, print => ["args", "glb"],
    post_w, post => ["glb"],
    input_w, input => ["args", "glb"],
    inputcast_w, inputcast => ["args", "glb"],
    ifcommand_w, ifcommand => ["args", "glb", "qr", "scp", "ctx", "ctr", "cnv"],
    whilecommand_w, whilecommand => ["args", "glb", "qr", "scp", "ctx", "ctr", "cnv"],
    init_w, init => ["ctx", "ctr", "cnv"]
);





