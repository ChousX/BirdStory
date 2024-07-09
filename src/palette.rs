use crate::prelude::*;

pub struct ColorPalettePlugin;
impl Plugin for ColorPalettePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorPalette>()
    }
}

#[derive(Resource)]
pub struct ColorPalette(pub [Color; 4]);

impl Default for ColorPalette {
    fn default() -> Self {
        let colors = [
            Color::srgb(83.0, 82.0, 173.0),
            Color::srgb(173.0, 82.0, 126.0),
            Color::srgb(172.0, 173.0, 82.0),
            Color::srgb(82.0, 173.0, 129.0)
        ];
        Self(colors)
    }
}
