use macroquad::{logging::error, text::{load_ttf_font, load_ttf_font_from_bytes, Font}};



pub struct Assets {
    pub font_monogram: Font,
}


impl Default for Assets {
    fn default() -> Self {
        let font = load_ttf_font_from_bytes( include_bytes!("..\\assets\\monogram.ttf") );
        if font.is_err() {
            error!("Unable to load monogram font!")
        }

        return Assets {
            font_monogram: font.unwrap()
        }
    }
}