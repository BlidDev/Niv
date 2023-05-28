use crate::{structs::{Globals, Type, ERROR, Scope, QueryW}, make_wrapper, make_wrappers};

use super::{
    calculations::{*, op},
    variables::*,
    scopes::*,
    prints::{post, print, input, inputcast},
};


use unstringify::unstringify;
 
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
    ifcommand_w, ifcommand => ["args", "glb", "qr", "scp"],
    whilecommand_w, whilecommand => ["args", "glb", "qr", "scp"]
);




