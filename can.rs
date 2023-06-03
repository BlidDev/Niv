use sfml::{window::{ContextSettings, Style, Event, Key}, graphics::{RenderWindow, RenderTarget, Color, Texture, Sprite}};


fn main() {
    let context_settings = ContextSettings {
        antialiasing_level : 0,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        (848, 480),
        "Canvas Test",
        Style::CLOSE,
        &context_settings
    );


    let mut vec = [20, 20 , 20, 255].repeat(50 * 50);

    let mut tex = Texture::new().unwrap();
    _ = tex.create(50, 50);



    unsafe {tex.update_from_pixels(&vec, 50, 50, 0, 0)};


    'game : loop {
        window.clear(Color::rgb(45, 84, 200));
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code : Key::Escape,.. } =>
                    break 'game,
                _ => {}
                
            }
        }
        window.draw(&Sprite::with_texture(&tex));
        

        window.display();
    }
}

fn randomize(v : &mut Vec<u8>) {

}
