use sfml::{graphics::{Text, Font, Color, Transformable}, SfBox, system::Vector2f};

#[derive(Debug, Clone)]
pub struct TextObj {
    pub msg : String,
    pub is_visible : bool, 
    pub color : (u8, u8, u8),
    pub pos : (u32, u32),
    pub scale : u32
}

impl TextObj {
    pub fn new(msg : String,is_visible : bool, scale : u32) -> Self {
        TextObj { msg, is_visible, color: (0, 0, 0), pos: (0,0), scale}
    }

    pub fn to_ojb<'a>(&'a self, font : &'a SfBox<Font>) -> Text {

        let mut t = Text::new(&self.msg, font, self.scale);
        let (r,g,b) = self.color;
        let pos = Vector2f::new(self.pos.0 as f32, self.pos.1 as f32);
        t.set_fill_color(Color::rgb(r, g, b));
        t.set_position(pos);

        t
    }
}
