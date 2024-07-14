use StoryBird::{plugins::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            GUIPlugin, 
            ColorPalettePlugin, 
            FilesPlugin,
            BiblioPlugin,
         ))
        .run();
}
