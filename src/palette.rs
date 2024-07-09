use crate::prelude::*;

pub struct ColorPalettePlugin;
impl Plugin for ColorPalettePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorPalette>();
    }
}

#[derive(Resource)]
pub struct ColorPalette(pub [Color; 4]);

impl Default for ColorPalette {
    fn default() -> Self {
        let colors = [
            Color::srgb_u8(83, 82, 173),
            Color::srgb_u8(173, 82, 126),
            Color::srgb_u8(172, 173, 82),
            Color::srgb_u8(82, 173, 129)
        ];
        Self(colors)
    }
}


