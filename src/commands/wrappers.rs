use crate::{structs::{Globals, Type, ERROR, Scope, QueryW, Roots}, make_wrapper, make_wrappers};
use crate::canvas::Canvas;

use super::{
    calculations::{*, op},
    variables::*,
    scopes::*,
    prints::{dbg, post, print, input, inputcast, format_command},
    graphics::*,
    misc::*,
    input::*,
    helper::*, utype::{make_type, setf, getf},
    cst::cst,
    list::*,
    str::*
};


use unstringify::unstringify;
 
make_wrappers!(
    set_w ,set => ["args", "glb"],
    release_w, release => ["args","glb"],
    reset_w, reset => ["glb"],

    make_w, make_type => ["args","glb"],
    setf_w, setf  => ["args","glb"],
    getf_w, getf  => ["args","glb"],

    cal_w, cal => ["args", "glb"],
    op_w, op => ["args", "glb"],
    sqrt_w, sqrt => ["args", "glb"],
    abs_w, abs => ["args", "glb"],

    sin_w, sin => ["args", "glb"],
    cos_w, cos => ["args", "glb"],
    tan_w, tan => ["args", "glb"],
    asin_w, asin => ["args", "glb"],
    acos_w, acos => ["args", "glb"],
    atan_w, atan => ["args", "glb"],

    print_w, print => ["args", "glb"],
    dbg_w, dbg => ["args", "glb"],
    format_w, format_command => ["args", "glb"],
    post_w, post => ["glb"],
    input_w, input => ["args", "glb"],
    inputcast_w, inputcast => ["args", "roots","glb", "qr", "scp", "cnv"],

    ifcommand_w, ifcommand => ["args", "roots", "glb", "qr", "scp", "cnv"],
    singleif_w, single_if => ["args", "roots", "glb", "qr", "scp", "cnv"],
    whilecommand_w, whilecommand => ["args", "roots","glb", "qr", "scp", "cnv"],

    init_w, init => ["args", "glb", "cnv"],
    end_graphics_w, end_graphics => ["cnv"],
    set_clear_w, set_clear => ["args", "glb", "cnv"],
    clear_w, clear => ["cnv"],
    display_w, display => ["cnv"],
    apply_pixels_w, apply_pixels => ["cnv"],
    set_pixel_w, set_pixel => ["args", "glb", "cnv"],
    set_area_w, set_area => ["args", "glb", "cnv"],
    get_pixel_w, get_pixel => ["args", "glb", "cnv"],
    get_millis_w, get_millis => ["cnv"],

    handle_input_w, handle_input => ["glb","cnv"],
    key_pressed_w, key_pressed => ["args", "glb"],

    sleep_w, sleep_command => ["args", "glb"],
    exit_w, exit => [],
    rng_w, rng => ["args", "glb"],
    cst_w, cst => ["args", "glb"],


    run_w, run => ["args", "roots", "glb", "qr", "cnv"],


    gete_w , gete => ["args", "glb"],
    sete_w , sete => ["args", "glb"],

    list_clear_w, list_clear => ["args", "glb"],
    list_remove_w, list_remove => ["args", "glb"],
    list_len_w, list_len => ["args", "glb"],
    list_push_w, list_push => ["args", "glb"],
    list_pop_w, list_pop => ["args", "glb"],
    list_empty_w, list_empty => [],
    repeat_w, repeat => ["args", "glb"],
    repeatl_w, repeatl => ["args", "glb"],

    stolist_w, stolist => ["args", "glb"],
    ltostr_w, ltostr => ["args", "glb"],

    lines_w, lines => ["args", "glb"],
    words_w, words => ["args", "glb"],
    trim_w , trim  => ["args", "glb"],


    ovid_w, ovid => ["cnv"],
    dorbell_w, dorbell => ["args", "roots", "glb", "qr", "scp", "cnv"],
    badduck_w, badduck => [],
    zayther_w, zayther => [],
    astro_w, astro => [],
    blid_w, blid => ["cnv"]
);




