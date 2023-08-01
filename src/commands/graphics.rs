use device_query::DeviceQuery;
use sfml::{window::{Event, Style}, graphics::Font};
use crate::{structs::{Type, ERROR, GError, Globals}, gerr, canvas::{Canvas, CanvasBuilder}, sgerr, util::{args_to_vars, get_variable}, commands::variables::set, text::TextObj};



pub fn init(
    args : Vec<Type>,
    glb  : &mut Globals,
    cnv : &mut Option<Canvas>,
    ) -> Result<Type, ERROR> {

    /*let (
        Type::I32(w),
        Type::I32(h),
        Type::I32(cw),
        Type::I32(ch)) = (args[0], args[1], args[2], args[3]) else {
        return  gerr!("Error: Invalid args for [init]")
    };*/

    let args = args_to_vars(&args, &glb.stack)?;
    let title = match &args[4] {
        Type::STR(ref n) => n.clone(),
        Type::CHAR(ref c) => c.to_string().clone(),
        _ => return gerr!("Error: invalid window title in [init]: [{:?}]", args[4])
    };

    sgerr!(
        (Type::I32(w), Type::I32(h),Type::I32(cw),Type::I32(ch)),
        (&args[0], &args[1], &args[2], &args[3]), 
        "Error: invalid args for [init]: {args:?}");

    *cnv = Some({
        CanvasBuilder::new()
            .title(&title)
            .window_size((*w as u32 ,*h as u32))
            .canvas_size((*cw as u32,*ch as u32))
            .style(Style::CLOSE)
            .build()?
    });
    
    Ok(Type::VOID())
}

pub fn end_graphics(cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv_o) = cnv else {
        return gerr!("Error: trying to end non initilized canvas")
    };

    cnv_o.window.close();

    *cnv = None;

    Ok(Type::VOID())
}

pub fn set_clear(
    args : Vec<Type>,
    glb  : &mut Globals,
    cnv : &mut Option<Canvas>,
) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2]),
        "Error: invalid args for [init]: {args:?}"
    );

    if *r > u8::MAX as i32 || *r < 0
    || *g > u8::MAX as i32 || *g < 0
    || *b > u8::MAX as i32 || *b < 0 
    {
        return gerr!("Error: rgb values out of range in [set_clear]: [r: {}, g: {}, b: {}]",r,g,b);
    }
    cnv.set_clear(*r as u8, *g as u8, *b as u8);
    Ok(Type::VOID())
}

pub fn clear(cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    cnv.clear();

    Ok(Type::VOID())
}

pub fn apply_pixels(cnv : &mut Option<Canvas>) -> Result<Type,ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };
    cnv.apply();
    Ok(Type::VOID())
}

pub fn set_pixel(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(x), Type::I32(y),
         Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2], &args[3], &args[4]),
        "Error: invalid args for [set_pixel]: {args:?}"
    );

    if *r > u8::MAX as i32 || *r < 0
    || *g > u8::MAX as i32 || *g < 0
    || *b > u8::MAX as i32 || *b < 0 
    {
        return gerr!("Error: rgb values out of range in [set_pixel]: [r: {}, g: {}, b: {}]",r,g,b);
    }

    //println!("x: {} y: {}", *x, *y);

    cnv.set_pixel((*x as u32, *y as u32), *r as u8, *g as u8, *b as u8);

    Ok(Type::VOID())
}


pub fn set_area(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(x), Type::I32(y),
         Type::I32(w), Type::I32(h),
         Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2], &args[3], &args[4], &args[5], &args[6]),
        "Error: invalid args for [set_area]: {args:?}"
    );

    cnv.set_area((*x as u32, *y as u32), (*w as u32, *h as u32), *r as u8, *g as u8, *b as u8)?;


    //glb.ddd.s = Text::new("bruh", &glb.fnt, 25);

    Ok(Type::VOID())
}

pub fn get_pixel(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;
    sgerr!(
        (
            Type::I32(ref x), Type::I32(ref y), r, g, b, 
        ),
        (&args[0], &args[1], &args[2], &args[3], &args[4]),
        "Error: invalid arguments given to [get_pixel]: {:?}", args
    );

    if  *x >= cnv.c_size.0 as i32 || *x < 0 ||
        *y >= cnv.c_size.1 as i32 || *y < 0  {
        return gerr!("Error: [get_pixel] coords ([{}, {}]) are out of canvas area", *x, *y);
    }

    let offest = (*y as u32 * cnv.c_size.0 * 4 + (*x as u32 * 4)) as usize;

    let (pr,pg,pb) = (
        cnv.pixels[offest + 0],
        cnv.pixels[offest + 1],
        cnv.pixels[offest + 2]
    );

    set(vec![r.clone(),Type::I32(pr as i32)], glb)?;
    set(vec![g.clone(),Type::I32(pg as i32)], glb)?;
    set(vec![b.clone(),Type::I32(pb as i32)], glb)?;

    Ok(Type::VOID())
}


pub fn get_area(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(x), Type::I32(y),
         Type::I32(w), Type::I32(h)),
        (&args[0], &args[1], &args[2], &args[3]),
        "Error: invalid args for [get_area]: {args:?}"
    );

    let v = cnv.get_area((*x as u32, *y as u32), (*w as u32, *h as u32))?;

    let v : Vec<Type> = v.iter().map(|e| Type::I32(*e as i32)).collect();

    Ok(Type::LIST(v))
}

pub fn display(cnv : &mut Option<Canvas>) -> Result<Type,ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };
    cnv.display();
    Ok(Type::VOID())
}


pub fn handle_input(glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    if let Some(cnv_o) = cnv {
        if glb.canvas_should_close {
            return Ok(Type::BOOL(true));
        }

        'events: while let Some(event) =  cnv_o.window.poll_event(){

            match event {
                Event::Closed =>
                {cnv_o.window.close(); glb.canvas_should_close = true; break 'events;},
                Event::Resized { width, height } => {
                    cnv_o.w_size = (width, height);
                    cnv_o.set_view();
                }
                _ => {}

            }
        }

        if glb.canvas_should_close {
            *cnv = None;
            return  Ok(Type::BOOL(true));
        }
    };


    glb.keys = glb.device_state.get_keys();
    

    Ok(Type::BOOL(false))
}

pub fn get_millis( cnv : &mut Option<Canvas> ) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    Ok(Type::I32(cnv.clock.elapsed_time().as_milliseconds()))
}

pub fn load_font( args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> 
{
    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };
    let path = get_variable(args.first().unwrap(), &glb.stack)?;

    sgerr!(
        Type::STR(path),
        path,
        "Error: invalid path given in [load_font]: [{:?}]", path
    );

    match Font::from_file(&path) {
        Some(f) => cnv.font = Some(f),
        None => return gerr!("Error: could not load font from path: [{}]", path),
    }

    Ok(Type::VOID())
}

pub fn new_text(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
            (Type::STR(msg), Type::I32(scale)),
            (&args[0], &args[1]),
            "Error: wrongs args given to [new_text]: {:?}", args
    );

    let Some(_) = &cnv.font else {
        return gerr!("Error: trying to create text before loading a font");
    };


    let id =
        cnv.texts
        .insert(TextObj::new(msg.clone(), true, *scale as u32));

    Ok(Type::I32(id as i32))
}


pub fn set_ttext(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;


    sgerr!(
        (Type::I32(id), Type::STR(msg)),
        (&args[0], &args[1]),
        "Error: invalid arguments given to [sett_text]: {:?}", args
    );

    let Ok(entry) = cnv.texts.get_mut(*id as usize) else {
        return  gerr!("Error: text with id [{}] does not exist", *id);
    };

    entry.msg = msg.clone();

    Ok(Type::VOID())
}


pub fn set_tpos(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;


    sgerr!(
        (Type::I32(id), Type::I32(x), Type::I32(y)),
        (&args[0], &args[1], &args[2]),
        "Error: invalid arguments given to [sett_pos]: {:?}", args
    );

    let Ok(entry) = cnv.texts.get_mut(*id as usize) else {
        return  gerr!("Error: text with id [{}] does not exist", *id);
    };

    entry.pos = (*x as u32, *y as u32);

    Ok(Type::VOID())
}

pub fn set_tclr(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;


    sgerr!(
        (Type::I32(id), Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2], &args[3]),
        "Error: invalid arguments given to [sett_clr]: {:?}", args
    );

    let Ok(entry) = cnv.texts.get_mut(*id as usize) else {
        return  gerr!("Error: text with id [{}] does not exist", *id);
    };

    entry.color = (*r as u8, *g as u8, *b as u8);

    Ok(Type::VOID())
}

pub fn set_tsize(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;


    sgerr!(
        (Type::I32(id), Type::I32(size)),
        (&args[0], &args[1]),
        "Error: invalid arguments given to [sett_size]: {:?}", args
    );

    let Ok(entry) = cnv.texts.get_mut(*id as usize) else {
        return  gerr!("Error: text with id [{}] does not exist", *id);
    };

    entry.scale = *size as u32;

    Ok(Type::VOID())
}


pub fn set_tvisible(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;


    sgerr!(
        (Type::I32(id), Type::BOOL(flag)),
        (&args[0], &args[1]),
        "Error: invalid arguments given to [sett_visible]: {:?}", args
    );

    let Ok(entry) = cnv.texts.get_mut(*id as usize) else {
        return  gerr!("Error: text with id [{}] does not exist", *id);
    };

    entry.is_visible = *flag;

    Ok(Type::VOID())
}


pub fn end_text(args : Vec<Type>, glb : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {

    let Some(cnv) = cnv else {
        return gerr!("Error: canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;

     let name = match &args[0] {

        Type::CHAR(ref c) => c.clone().to_string(),
        Type::STR(ref s) => s.clone(),
         _ => return gerr!("Error: invalid text type name [{:?}] given to [end_text]", args[0])
     };

     let Some(Type::I32(id)) = glb.stack.get(&name) else {
         return gerr!("Error: invalid variable given to [end_text]: [{:?}]", &args[0])
     };

    cnv.texts.remove(*id as usize)?;


    set(vec![Type::STR(name), Type::VOID()], glb)
}
