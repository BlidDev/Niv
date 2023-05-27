use crate::structs::{ERROR, Type};

use framework::{canvas::canvas::Canvas, sdl2::context::Context};
use sdl2::{render::TextureCreator, video::WindowContext, pixels::Color, event::Event};

pub fn init<'a>(
    context: &mut Option<Context>,
    creator: &'a mut Option<TextureCreator<WindowContext>>,
    canvas: &mut Option<Canvas<'a>>,
) -> Result<Type, ERROR> {



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
