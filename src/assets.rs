use macroquad::{logging::error, text::{load_ttf_font, load_ttf_font_from_bytes, Font}, texture::Texture2D};



pub struct Assets {
    pub font_monogram: Font,
    pub start_cube: Texture2D,
}


impl Default for Assets {
    fn default() -> Self {
        let font = load_ttf_font_from_bytes( include_bytes!("..\\assets\\monogram.ttf") );
        if font.is_err() {
            error!("Unable to load monogram font!")
        }

        return Assets {
            font_monogram: font.unwrap(),
            start_cube: Texture2D::from_file_with_format( include_bytes!("..\\assets\\start.png"), None),
        }
    }
}