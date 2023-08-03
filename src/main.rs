use std::collections::{HashMap, HashSet};

mod util;
mod structs;
mod macros;
mod ops;
mod canvas;
mod user_type;
mod args;
mod text;
pub mod commands;

use args::Arguments;
use clap::Parser;
use device_query::DeviceState;
use structs::{Stack, Globals, QueryW, Registry};
use user_type::register_types;
use util::*;
use commands::wrappers::*;
use crate::structs::CommandQuery;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args =  Arguments::parse();
    let lines = args.args_to_lines()?;
    let lines = remove_comments_from_lines(&lines)?;
    
    let roots = find_root_scopes(&lines)?;



    let (roots, args_list) = {
        let mut v = HashMap::new();
        let mut an = HashMap::new();

        for (name, (start, end, args)) in roots.iter() {

            let temp = root_to_scope_tree(&lines, &(*start, *end))?;
            an.insert(name.clone(), args.clone());
            v.insert(name.clone(),temp);
        }
        (v, an)
    };
    
    let mut query = CommandQuery::new();

    register_commands(&mut query);


    let mut glb = Globals {
        stack : Stack::default(),
        curr : 0,
        s : " ",
        device_state : DeviceState::new(),
        keys : vec![],
        canvas_should_close : false,
        args_list,

        registered_types : HashMap::new(),

        input_files : HashSet::new(),
        output_files : HashMap::new(),

        timers : Registry::new()
        
    };

    let query = QueryW(query.clone());
    validate_root_scopes_names(&roots, &query)?;

    let mut cnv = None;

    let s = roots.get("MAIN").expect(&format!("Error: Root scope [{}] not found", "MAIN"));
    register_types(&lines, &roots, &s, &query, &mut glb, &mut cnv)?;




    traverse_root_scope("MAIN", &roots, &query, &mut glb,  &mut cnv)?;
 
 
    Ok(())
}


fn register_commands(query : &mut CommandQuery)
{
    commands![
        (*query),
        {
            set =>      (set_w, Some(2)),
            release =>  (release_w, Some(1)),
            reset =>    (reset_w, Some(0)),

            make =>     (make_w, None),
            setf =>     (setf_w, Some(3)),
            getf =>     (getf_w, Some(2)),

            cal =>      (cal_w,Some(3)),
            op =>       (op_w,Some(3)),
            inv =>      (inv_w, Some(1)),
            sqrt =>     (sqrt_w, Some(1)),
            abs  =>     (abs_w, Some(1)),

            sin =>      (sin_w, Some(1)),
            cos =>      (cos_w, Some(1)),
            tan =>      (tan_w, Some(1)),
            asin =>     (asin_w, Some(1)),
            acos =>     (acos_w, Some(1)),
            atan =>     (atan_w, Some(1)),

            post =>     (post_w,Some(0)),
            print =>    (print_w,None),
            dbg =>      (dbg_w, Some(1)),
            prt =>      (prt_w, Some(1)),
            format =>   (format_w,None),
            input =>    (input_w,None),
            inputcast =>(inputcast_w,None),

            if =>       (ifcommand_w,Some(3)),
            singleif=>       (singleif_w, None),
            while =>    (whilecommand_w,Some(1)),

            init => (init_w, Some(5)),
            end_graphics => (end_graphics_w, Some(0)),
            set_clear => (set_clear_w, Some(3)),
            clear => (clear_w, Some(0)),
            display => (display_w, Some(0)),
            apply => (apply_pixels_w, Some(0)),
            set_pixel => (set_pixel_w, Some(5)),
            set_area => (set_area_w, Some(7)),
            get_area => (get_area_w, Some(4)),
            get_pixel => (get_pixel_w, Some(5)),
            get_millis => (get_millis_w, Some(0)),

            load_font => (load_font_w, Some(1)),
            new_text => (new_text_w, Some(2)),

            set_ttext => (set_ttext_w, Some(2)),
            set_tsize =>(set_tsize_w, Some(2)),
            set_tpos => (set_tpos_w, Some(3)),
            set_tclr => (set_tclr_w, Some(4)),
            set_tvisible => (set_tvisible_w, Some(2)),
            end_text => (end_text_w, Some(1)),

            handle_input => (handle_input_w, Some(0)),
            key_pressed => (key_pressed_w, Some(1)),

            sleep => (sleep_w, Some(1)),
            rng => (rng_w, Some(2)),
            exit => (exit_w, Some(0)),
            cst  => (cst_w, Some(2)),
            err  => (err_msg_w, None),
            typeid => (typeid_w, Some(1)),

            new_timer => (new_timer_w, Some(1)),
            timer_elapsed => (timer_elapsed_w, Some(1)),
            timer_millis => (timer_millis_w, Some(1)),
            timer_reset => (timer_reset_w, Some(1)),
            timer_set_delay => (timer_set_delay_w, Some(2)),
            end_timer => (end_timer_w, Some(1)),

            chain => (chain_w, None),

            run => (run_w, None),
            return => (return_cmd_w, Some(1)),

            gete => (gete_w, Some(2)),
            sete => (sete_w, Some(3)),
            lclear => (list_clear_w, Some(1)),
            lremove => (list_remove_w, Some(2)),
            llen => (list_len_w, Some(1)),
            lpush => (list_push_w, Some(2)),
            lpop => (list_pop_w, Some(1)),
            lempty => (list_empty_w, Some(0)),
            repeat => (repeat_w, Some(2)),
            repeatl => (repeatl_w, Some(2)),

            stolist => (stolist_w, Some(1)),
            ltostr  => (ltostr_w, Some(1)),

            lines   => (lines_w, Some(1)),
            words   => (words_w, Some(1)),
            s_words   => (s_words_w, Some(1)),
            trim    => (trim_w , Some(1)),

            openfile => (openfile_w, Some(2)),
            readbuf  => (readbuf_w, Some(1)),
            writef   => (writef_w, Some(3)),
            closef   => (closef_w, Some(1)),

            ovid => (ovid_w, Some(0)),
            dorbell => (dorbell_w, Some(1)),
            badduck => (badduck_w, Some(0)),
            zayther => (zayther_w, Some(0)),
            astro   => (astro_w, Some(0)),
            blid    => (blid_w, Some(0))
        }
    ];
}

