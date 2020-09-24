use crate::font;
use crate::types;
use crate::util;

use types::Game;

impl Game {
    pub fn render_text(&mut self, text: &str, x: u32, y: u32) {
        for (i, c) in text.chars().enumerate() {
            self.render_character(c, x + i as u32 * 6, y);
        }
    }
    
    pub fn render_character(&mut self, character: char, x: u32, y: u32) {
        let font = util::uncompress(font::COMPRESSED_FONT[character as usize - '!' as usize].to_vec());
    
        for (i, pixel) in font.iter().enumerate() {
            if *pixel == 1 {
                self.screen[y as usize + i / 5][x as usize + i % 5] = 1;          
            }
        }
    }
}
