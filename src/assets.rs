use macroquad::{audio::{load_sound_from_bytes, Sound}, logging::error, text::{load_ttf_font_from_bytes, Font}, texture::Texture2D};



pub struct Assets {
    pub font_monogram: Font,
    pub start_cube: Texture2D,
    pub maxhp: Texture2D,
    pub projectile: Texture2D,
    pub size: Texture2D,
    pub slowdmg: Texture2D,
    pub speed: Texture2D,
    pub shooter: Texture2D,
    pub tower: Texture2D,
    pub menu1: Texture2D,
    pub menu2: Texture2D,
    pub menu_song: Sound,
    pub play_song: Sound,
    pub menu_switch: Sound,
    pub shoot: Sound,
    pub hit: Sound,
    pub dead: Sound,
}


impl Assets {
    pub async fn default() -> Self {
        let font = load_ttf_font_from_bytes( include_bytes!("..\\assets\\monogram.ttf") );
        if font.is_err() {
            error!("Unable to load monogram font!")
        }

        return Assets {
            font_monogram: font.unwrap(),
            start_cube: Texture2D::from_file_with_format( include_bytes!("..\\assets\\start.png"), None),
            maxhp: Texture2D::from_file_with_format( include_bytes!("..\\assets\\maxhp.png"), None),
            projectile: Texture2D::from_file_with_format( include_bytes!("..\\assets\\projectile.png"), None),
            size: Texture2D::from_file_with_format( include_bytes!("..\\assets\\size.png"), None),
            slowdmg: Texture2D::from_file_with_format( include_bytes!("..\\assets\\slowdmg.png"), None),
            speed: Texture2D::from_file_with_format( include_bytes!("..\\assets\\speed.png"), None),
            tower: Texture2D::from_file_with_format( include_bytes!("..\\assets\\tower.png"), None),
            menu1: Texture2D::from_file_with_format( include_bytes!("..\\assets\\menu1.png"), None),
            menu2: Texture2D::from_file_with_format( include_bytes!("..\\assets\\menu2.png"), None),
            shooter: Texture2D::from_file_with_format( include_bytes!("..\\assets\\shooter.png"), None),
            play_song: load_sound_from_bytes( include_bytes!("..\\assets\\medium_boss.wav") ).await.unwrap(),
            menu_song: load_sound_from_bytes( include_bytes!("..\\assets\\little_slime.wav") ).await.unwrap(),
            menu_switch: load_sound_from_bytes( include_bytes!("..\\assets\\menu.wav") ).await.unwrap(),
            shoot: load_sound_from_bytes( include_bytes!("..\\assets\\shoot.wav") ).await.unwrap(),
            hit: load_sound_from_bytes( include_bytes!("..\\assets\\hit.wav") ).await.unwrap(),
            dead: load_sound_from_bytes( include_bytes!("..\\assets\\dead.wav") ).await.unwrap(),
        }
    }
}