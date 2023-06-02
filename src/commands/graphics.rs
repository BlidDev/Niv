use framework::{canvas::canvas::Canvas, sdl2::context::Context};
use sdl2::{render::TextureCreator, video::WindowContext, pixels::Color};
use std::{rc::Rc, borrow::{Borrow, BorrowMut}, sync::Mutex};

use crate::{structs::{Type, ERROR, GError}, gerr};

fn init_a(
    mut ctx : Rc<Option<Context>>,
    mut ctr : Rc<Option<TextureCreator<WindowContext>>>,
    mut cnv : Rc<Option<Canvas>>
) -> Result<Type, ERROR>{

    let mut ctx = ctx.borrow_mut();
    let mut ctr = ctr.borrow_mut();
    let mut cnv = cnv.borrow_mut();
    init(&mut ctx, &mut ctr, &mut cnv)
}

fn init<'a>(
    context: &mut Option<Context>,
    creator: &'a mut Option<TextureCreator<WindowContext>>,
    canvas: &mut Option<Canvas<'a>>,
) -> Result<Type, Box<dyn std::error::Error>> {

    *context = Some({
        let mut context = Context::new();
        context
            .with_window("Example", 848, 480, false)?
            .with_textures()?;
        context
    });

    let Some(ref mut ctx) = context else { panic!("") };
    let Some(ref mut cvn) = ctx.canvas else { panic!("") };
    cvn.set_draw_color(Color::RGB(150, 150, 150));
    cvn.clear();
    cvn.present();

    *creator = Some(cvn.texture_creator());
    
    *canvas = Some(Canvas::create_canvas(&creator.as_ref().unwrap(), 16, 9));

    let Some(ref mut cvn) = canvas else { panic!("") };
    cvn.set_clear_color(50, 50, 50, 255).clear().apply_pixels();
    cvn.set_pixel(0,0,0,0,0,0);
    cvn.apply_pixels();

    Ok(Type::VOID())
}
