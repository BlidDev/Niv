use sfml::{window::{ContextSettings, Style, Event}, graphics::{RenderWindow, RenderTarget, Texture, Sprite, Font, View, FloatRect}, SfBox, system::{Vector2, Clock}};

use crate::{structs::{ERROR, GError, Registry}, gerr, text::TextObj};

type Vec2 = (u32, u32);

pub fn product(v : &Vec2)  -> u32 {
    v.0 * v.1
}


pub struct Canvas {
    pub w_size : Vec2,
    pub c_size : Vec2,
    pub window : RenderWindow,
    pub texture : SfBox<Texture>,
    pub pixels : Vec<u8>,
    pub cleanup_buffer : Vec<u8>,
    pub clock : SfBox<Clock>,

    pub font : Option<SfBox<Font>>,
    pub texts : Registry<TextObj>,

    pub pixel_view : SfBox<View>,
    pub text_view  : SfBox<View>,
}

impl Canvas {
    pub fn set_pixel(&mut self, pos : Vec2, r : u8, g : u8, b : u8) {
        let pos = (pos.0.clamp(0, self.c_size.0 ),pos.1.clamp(0, self.c_size.1));
        let offest = (pos.1 * (self.c_size.0 * 4) + (pos.0 * 4)) as usize;
        self.pixels[offest + 0] = r;
        self.pixels[offest + 1] = g;
        self.pixels[offest + 2] = b;

    }

    pub fn set_area(&mut self, pos : Vec2, size : Vec2, r : u8, g : u8, b : u8) -> Result<(), ERROR>{
        if  pos.0 + size.0 > self.c_size.0 ||
            pos.1 + size.1 > self.c_size.1 {
                return gerr!("Error: area in [set_area] does not fit the canvas size [x: {}, y: {}, w: {}, h: {}]", pos.0, pos.1, size.0, size.1);
        }

        //println!("{:?} {:?}", pos, size);
        let slice = [r,g,b, 255].repeat(size.0 as usize);
        for y in 0..size.1 {
            let start = (((y + pos.1) * self.c_size.0 * 4) + (pos.0) * 4) as usize;
            let end = start + size.0 as usize * 4;
            self.pixels[start..end].copy_from_slice(&slice.clone())
        }


        Ok(())
    }

    pub fn get_area(&mut self, pos : Vec2, size : Vec2) -> Result<Vec<u8>, ERROR>{
        if  pos.0 + size.0 > self.c_size.0 ||
            pos.1 + size.1 > self.c_size.1 {
                return gerr!("Error: area in [get_area] does not fit the canvas size [x: {}, y: {}, w: {}, h: {}]", pos.0, pos.1, size.0, size.1);
        }

        let mut v = vec![];
        for y in 0..size.1 {
            for x in 0..size.0 {
                let index = (((pos.1 + y) * self.c_size.0 * 4) + (pos.0 + x) * 4) as usize;
                v.extend(self.pixels[index..index+4].to_vec());

            }
        }


        Ok(v)
    }

    pub fn apply(&mut self) {
        unsafe {
            self.texture.update_from_pixels(&self.pixels,
                self.c_size.0,
                self.c_size.1,
                0,0);
        }
    }

    pub fn set_clear(&mut self, r : u8, g : u8, b : u8) {
        self.cleanup_buffer = [r,g,b,255].repeat(self.pixels.len());
    }

    pub fn clear(&mut self) {
        self.pixels = self.cleanup_buffer.clone()
    }

    pub fn display(&mut self) {


        let s = Sprite::with_texture(&self.texture);

        self.window.set_view(&self.pixel_view);
        self.window.draw(&s);
        if let Some(font) = self.font.as_ref() {
            self.window.set_view(&self.text_view);
            for text in self.texts.map.iter()
            {
                if !text.1.is_visible { continue; }
                let t = text.1.to_ojb(font);
                self.window.draw(&t);
            }
        }
        
        self.window.display();
        
    }

    pub fn set_view(&mut self) {
        get_letterbox_view(&mut self.pixel_view, self.w_size.0, self.w_size.1);
    }

    pub fn input<T>(&mut self, mut fun : T) 
    where T : FnMut(Event){
        while let Some(event) = self.window.poll_event(){
            fun(event)
        }
    }
}

#[derive(Debug, Default)]
pub struct CanvasBuilder {
    settings : Option<ContextSettings>,
    w_size   : Option<Vec2>,
    c_size   : Option<Vec2>,
    title    : Option<String>,
    style    : Option<Style>
}

impl CanvasBuilder {
    pub fn new() -> CanvasBuilder {
        CanvasBuilder::default()
    }

    #[allow(dead_code)]
    pub fn settings(mut self, settings : ContextSettings) -> CanvasBuilder {
        self.settings = Some(settings);
        self
    }
    pub fn window_size(mut self,size : Vec2) -> CanvasBuilder {
        self.w_size = Some(size);
        self
    }

    pub fn canvas_size(mut self,size : Vec2) -> CanvasBuilder {
        self.c_size = Some(size);
        self
    }

    pub fn title(mut self, title : &str) -> CanvasBuilder {
        self.title = Some(title.to_owned());
        self
    }

    pub fn style(mut self, style : Style) -> CanvasBuilder {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> Result<Canvas, ERROR> {
        let w_size = self.w_size.unwrap_or((848,480).into());
        let c_size = self.c_size.unwrap_or((16, 9).into());

        let settings = self.settings.unwrap_or(
            ContextSettings {
                antialiasing_level : 0,
                
                ..Default::default()
            }
        );

        let size = (
            c_size.0 as f32,
            c_size.1 as f32
        );

        let center = (
            c_size.0 as f32 / 2.0,
            c_size.1 as f32 / 2.0,
        );
        let pixel_view = View::new(center.into(), size.into());

        let size = (
            w_size.0 as f32,
            w_size.1 as f32
        );

        let center = (
            w_size.0 as f32 / 2.0,
            w_size.1 as f32 / 2.0,
        );
        let text_view = View::new(center.into(), size.into());

        let mut can = Canvas {
            w_size : w_size.clone(),
            c_size : c_size.clone(),
            window : RenderWindow::new(
                (w_size.0, w_size.1),
                &self.title.unwrap_or("Canvas Window".to_string()),
                self.style.unwrap_or(Style::DEFAULT | Style::RESIZE),
                &settings
            ),
            pixels : [0,0,0,255].repeat(product(&c_size) as usize).to_vec(),
            cleanup_buffer : [0,0,0,255].repeat(product(&c_size) as usize).to_vec(),
            texture : {
                let Some(mut tex) = Texture::new() else {
                    return gerr!("Error: Could not construct texture")
                };
                let true = tex.create(c_size.0, c_size.1) else {
                    return gerr!("Error: Could not create texture")
                };

                tex
            }
            ,
            clock : Clock::start(),
            
            font : None,
            texts : Registry::new(),
            pixel_view,
            text_view
        }; 

        can.window.set_vertical_sync_enabled(true);
        let mut size = sfml::window::VideoMode::desktop_mode();
        size.width  -= w_size.0 /2;
        size.height -= w_size.1 /2;
        can.window.set_position(Vector2::new(size.width as i32/ 2, size.height as i32/ 2));
        can.set_view();
        
        unsafe { 
            can.texture.update_from_pixels(&can.pixels, c_size.0, c_size.1, 0, 0);
        }

        Ok(can)
    }
}


// converted from https://github.com/SFML/SFML/wiki/Source%3A-Letterbox-effect-using-a-view 
fn get_letterbox_view(view : &mut SfBox<View>, w_w : u32, w_h : u32) {

    // Compares the aspect ratio of the window to the aspect ratio of the view,
    // and sets the view's viewport accordingly in order to archieve a letterbox effect.
    // A new view (with a new viewport set) is returned.

    let window_ratio = w_w as f32 / w_h as f32;
    let view_ratio = view.size().x / view.size().y;
    let mut size_x = 1.0;
    let mut size_y = 1.0;
    let mut pos_x = 0.0;
    let mut pos_y = 0.0;

    let mut horizontal_spacing = true;
    if window_ratio < view_ratio {
        horizontal_spacing = false;
    }

    // If horizontalSpacing is true, the black bars will appear on the left and right side.
    // Otherwise, the black bars will appear on the top and bottom.

    if horizontal_spacing {
        size_x = view_ratio / window_ratio;
        pos_x = (1.0 - size_x) / 2.0;
    }
    else {
        size_y = window_ratio / view_ratio;
        pos_y = (1.0 - size_y) / 2.0;
    }

    view.set_viewport( FloatRect::new(pos_x, pos_y, size_x, size_y) );

}
