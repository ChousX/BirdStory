use crate::prelude::*;

pub struct ColorPalettePlugin;
impl Plugin for ColorPalettePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorPalette>();
    }
}

#[derive(Resource)]
pub struct ColorPalette(pub [Color; 10]);

impl Default for ColorPalette {
    fn default() -> Self {
        use bevy::color::palettes::css::*;
        let colors = [
            CADET_BLUE.into(),
            DARK_CYAN.into(),
            DARK_SLATE_GREY.into(),
            BLACK.into(),
            Color::srgb_u8(255, 255, 255),
            Color::srgb_u8(83, 82, 173),
            Color::srgb_u8(83, 82, 163),
            Color::srgb_u8(82, 173, 129),
            Color::srgb_u8(173, 82, 126),
            Color::srgb_u8(122, 173, 82),
        ];
        Self(colors)
    }
}

impl ColorPalette{
    pub fn brows_view(&self) -> Color{
        self.0[2].clone()
    }

    pub fn info_view(&self) -> Color{
        self.0[2].clone()
    }

    pub fn text_active(&self) -> Color{
        self.0[4].clone()
    }

    pub fn text_inactive(&self) -> Color{
        self.0[1].clone()
    }


    pub fn border_active(&self) -> Color{
        self.0[0].clone()
    }

    pub fn border_inactive(&self) -> Color{
        self.0[1].clone()
    }

    pub fn search_text(&self) -> Color{
        self.0[4].clone()
    }

    pub fn search_bar_border(&self) -> Color{
        self.0[2].clone()
    }

    pub fn search_bar_background(&self) -> Color{
        self.0[1].clone()
    }

    pub fn search_button_normal(&self) -> Color{
        self.0[2].clone()
    }

    pub fn search_button_hovered(&self) -> Color {
        self.0[3].clone()
    }

    pub fn search_button_pressed(&self) -> Color {
        self.0[4].clone()
    }
}


