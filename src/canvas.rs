
use sfml::{window::{ContextSettings, Style, Event}, graphics::{RenderWindow, RenderTarget, Texture, Sprite, Transformable}, SfBox, system::{Vector2, Clock}};

use crate::{structs::{ERROR, GError}, gerr};

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
    pub clock : SfBox<Clock>
}

impl Canvas {
    pub fn set_pixel(&mut self, pos : Vec2, r : u8, g : u8, b : u8) {
        let pos = (pos.0.clamp(0, self.c_size.0 ),pos.1.clamp(0, self.c_size.1));
        let offest = (pos.1 * (self.c_size.0 * 4) + (pos.0 * 4)) as usize;
        self.pixels[offest + 0] = r;
        self.pixels[offest + 1] = g;
        self.pixels[offest + 2] = b;
        //self.pixels[offest + 2] = b;

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


        let mut s = Sprite::with_texture(&self.texture);

        s.set_scale((
                self.w_size.0 as f32 / s.local_bounds().width, 
                self.w_size.1 as f32 / s.local_bounds().height, 
        ));

        self.window.draw(&s);
        self.window.display();
        
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





        let mut can = Canvas {
            w_size : w_size.clone(),
            c_size : c_size.clone(),
            window : RenderWindow::new(
                (w_size.0, w_size.1),
                &self.title.unwrap_or("Canvas Window".to_string()),
                self.style.unwrap_or(Style::DEFAULT),
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
            clock : Clock::start()
        }; 

        let mut size = sfml::window::VideoMode::desktop_mode();
        size.width  -= w_size.0 /2;
        size.height -= w_size.1 /2;
        can.window.set_position(Vector2::new(size.width as i32/ 2, size.height as i32/ 2));
        
        unsafe { 
            can.texture.update_from_pixels(&can.pixels, c_size.0, c_size.1, 0, 0);
        }

        Ok(can)
    }
}
