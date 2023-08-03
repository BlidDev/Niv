use crate::{structs::{Globals, Type, ERROR, Scope, QueryW, Roots}, make_wrapper, make_wrappers};
use crate::canvas::Canvas;

use super::{
    calculations::{*, op},
    variables::*,
    scopes::*,
    prints::{dbg, prt, post, print, input, inputcast, format_command},
    graphics::*,
    misc::*,
    input::*,
    helper::*, utype::{make_type, setf, getf},
    cst::cst,
    list::*,
    str::*,
    files::*,
    chain::chain
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
    inv_w, inv => ["args", "glb"],

    sin_w, sin => ["args", "glb"],
    cos_w, cos => ["args", "glb"],
    tan_w, tan => ["args", "glb"],
    asin_w, asin => ["args", "glb"],
    acos_w, acos => ["args", "glb"],
    atan_w, atan => ["args", "glb"],

    print_w, print => ["args", "glb"],
    dbg_w, dbg => ["args", "glb"],
    prt_w, prt => ["args", "glb"],
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
    get_area_w, get_area => ["args", "glb", "cnv"],
    get_pixel_w, get_pixel => ["args", "glb", "cnv"],
    get_millis_w, get_millis => ["cnv"],

    load_font_w, load_font => ["args", "glb", "cnv"],
    new_text_w,  new_text  => ["args", "glb", "cnv"],

    set_ttext_w,    set_ttext => ["args", "glb", "cnv"],
    set_tsize_w,    set_tsize => ["args", "glb", "cnv"],
    set_tpos_w,     set_tpos => ["args", "glb", "cnv"],
    set_tclr_w,     set_tclr => ["args", "glb", "cnv"],
    set_tvisible_w, set_tvisible => ["args", "glb", "cnv"],
    end_text_w,     end_text => ["args", "glb", "cnv"],


    handle_input_w, handle_input => ["glb","cnv"],
    key_pressed_w, key_pressed => ["args", "glb"],

    sleep_w, sleep_command => ["args", "glb"],
    exit_w, exit => [],
    rng_w, rng => ["args", "glb"],
    cst_w, cst => ["args", "roots","glb", "qr", "scp", "cnv"],
    err_msg_w, err_msg => ["args", "glb"],
    typeid_w,  typeid  => ["args", "glb"],

    new_timer_w, new_timer => ["args", "glb"],
    timer_elapsed_w, timer_elapsed => ["args", "glb"],
    timer_millis_w, timer_millis => ["args", "glb"],
    timer_reset_w, timer_reset => ["args", "glb"],
    timer_set_delay_w, timer_set_delay => ["args", "glb"],
    end_timer_w, end_timer => ["args", "glb"],

    chain_w, chain => ["args", "roots","glb", "qr", "scp", "cnv"],

    run_w, run => ["args", "roots", "glb", "qr", "cnv"],
    return_cmd_w, return_cmd => ["args", "glb"],


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
    s_words_w, s_words => ["args", "glb"],
    trim_w , trim  => ["args", "glb"],

    openfile_w, openfile => ["args", "glb"],
    readbuf_w,  readbuf  => ["args", "glb"],
    writef_w,   writef   => ["args", "glb"],
    closef_w,   closef   => ["args", "glb"],

    ovid_w, ovid => ["cnv"],
    dorbell_w, dorbell => ["args", "roots", "glb", "qr", "scp", "cnv"],
    badduck_w, badduck => [],
    zayther_w, zayther => [],
    astro_w, astro => [],
    blid_w, blid => ["cnv"]
);




