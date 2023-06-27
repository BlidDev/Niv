use crate::{structs::{Globals, Type, ERROR, Scope, QueryW}, make_wrapper, make_wrappers};
use crate::canvas::Canvas;

use super::{
    calculations::{*, op},
    variables::*,
    scopes::*,
    prints::{post, print, input, inputcast, format_command},
    graphics::*,
    misc::*,
    input::*,
    helper::*
};


use unstringify::unstringify;
 
make_wrappers!(
    set_w ,set => ["args", "glb"],
    release_w, release => ["args","glb"],
    reset_w, reset => ["glb"],
    cal_w, cal => ["args", "glb"],
    op_w, op => ["args", "glb"],
    print_w, print => ["args", "glb"],
    format_w, format_command => ["args", "glb"],
    post_w, post => ["glb"],
    input_w, input => ["args", "glb"],
    inputcast_w, inputcast => ["args", "glb"],
    ifcommand_w, ifcommand => ["args", "glb", "qr", "scp", "cnv"],
    singleif_w, single_if => ["args", "glb", "qr", "scp", "cnv"],
    whilecommand_w, whilecommand => ["args", "glb", "qr", "scp", "cnv"],
    init_w, init => ["args", "glb", "cnv"],
    set_clear_w, set_clear => ["args", "glb", "cnv"],
    clear_w, clear => ["cnv"],
    display_w, display => ["cnv"],
    apply_pixels_w, apply_pixels => ["cnv"],
    set_pixel_w, set_pixel => ["args", "glb", "cnv"],
    set_area_w, set_area => ["args", "glb", "cnv"],
    get_pixel_w, get_pixel => ["args", "glb", "cnv"],
    handle_input_w, handle_input => ["glb","cnv"],
    key_pressed_w, key_pressed => ["args", "glb"],

    sleep_w, sleep_command => ["args", "glb"],
    exit_w, exit => [],
    rng_w, rng => ["args", "glb"],











    ovid_w, ovid => ["cnv"],
    dorbell_w, dorbell => ["args", "glb", "qr", "scp", "cnv"],
    badduck_w, badduck => [],
    zayther_w, zayther => [],
    astro_w, astro => [],
    blid_w, blid => ["cnv"]
);




