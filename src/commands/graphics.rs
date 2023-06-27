use device_query::DeviceQuery;
use sfml::window::{Event, Style};
use crate::{structs::{Type, ERROR, GError, Globals}, gerr, canvas::{Canvas, CanvasBuilder}, sgerr, util::args_to_vars, commands::variables::set};



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

    sgerr!(
        (Type::I32(w), Type::I32(h),Type::I32(cw),Type::I32(ch), Type::STR(title)),
        (&args[0], &args[1], &args[2], &args[3], &args[4]), 
        "Error: Invalid args for [init]: {args:?}");

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

pub fn set_clear(
    args : Vec<Type>,
    glb  : &mut Globals,
    cnv : &mut Option<Canvas>,
) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2]),
        "Error: Invalid args for [init]: {args:?}"
    );
    cnv.set_clear(*r as u8, *g as u8, *b as u8);
    Ok(Type::VOID())
}

pub fn clear(cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
    };

    cnv.clear();

    Ok(Type::VOID())
}

pub fn apply_pixels(cnv : &mut Option<Canvas>) -> Result<Type,ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
    };
    cnv.apply();
    Ok(Type::VOID())
}

pub fn set_pixel(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
    };
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(x), Type::I32(y),
         Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2], &args[3], &args[4]),
        "Error: Invalid args for [init]: {args:?}"
    );

    //println!("x: {} y: {}", *x, *y);

    cnv.set_pixel((*x as u32, *y as u32), *r as u8, *g as u8, *b as u8);

    Ok(Type::VOID())
}


pub fn set_area(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
    };
    let args = args_to_vars(&args, &glb.stack)?;

    sgerr!(
        (Type::I32(x), Type::I32(y),
         Type::I32(w), Type::I32(h),
         Type::I32(r), Type::I32(g), Type::I32(b)),
        (&args[0], &args[1], &args[2], &args[3], &args[4], &args[5], &args[6]),
        "Error: Invalid args for [init]: {args:?}"
    );

    cnv.set_area((*x as u32, *y as u32), (*w as u32, *h as u32), *r as u8, *g as u8, *b as u8)?;

    Ok(Type::VOID())
}

pub fn get_pixel(args : Vec<Type>, glb  : &mut Globals, cnv : &mut Option<Canvas>) -> Result<Type, ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
    };

    let args = args_to_vars(&args, &glb.stack)?;
    sgerr!(
        (
            Type::I32(ref x), Type::I32(ref y), r, g, b, 
        ),
        (&args[0], &args[1], &args[2], &args[3], &args[4]),
        "Error: Invalid arguments given to [get_pixel]: {:?}", args
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

pub fn display(cnv : &mut Option<Canvas>) -> Result<Type,ERROR> {
    let Some(cnv) = cnv else {
        return gerr!("Error: Canvas used before being initilized")
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

