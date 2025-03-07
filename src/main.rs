mod library;

use bevy::prelude::*;
use library::LibraryPlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(LibraryPlugin);

    let _ = app.run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Library,
    Player,
}
