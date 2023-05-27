use framework::{canvas::canvas::Canvas, sdl2::context::Context};
use sdl2::{render::TextureCreator, video::WindowContext, pixels::Color, event::Event};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut context: Option<Context> = None;
    let mut creator: Option<TextureCreator<WindowContext>> = None;

    let mut canvas: Option<Canvas> = None;

//    init(&mut context, &mut creator, &mut canvas)?;
//
//    let Some(ref mut ctx) = context else {panic!("")};
//    let mut event = ctx.sdl_context.event_pump()?;
//    let Some(ref mut ccvn) = ctx.canvas else {panic!("")};
//    let Some(ref mut cvn) = canvas else {panic!("")};
//
//    cvn.set_clear_color(50, 50, 50, 255).clear().apply_pixels();
//    cvn.set_pixel(0,0,0,0,0,0);
//    cvn.apply_pixels();
//
//    'game_loop : loop{
//        ccvn.clear();
//        for e in event.poll_iter() {
//            match e {
//                Event::Quit { .. } => break 'game_loop,
//                _=>{}
//            }
//        } 
//
//        cvn.display(ccvn);
//
//        ccvn.present();
//        
//    }

    Ok(())
}

fn init<'a>(
    context: &mut Option<Context>,
    creator: &'a mut Option<TextureCreator<WindowContext>>,
    canvas: &mut Option<Canvas<'a>>,
) -> Result<(), Box<dyn std::error::Error>> {

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

    Ok(())
}

