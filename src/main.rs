use StoryBird::{plugins::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: APP_NAME.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
        ))
        .add_plugins((
            GUIPlugin, 
            ColorPalettePlugin, 
            FilesPlugin,
            BiblioPlugin,
         ))
        .run();
}
