mod library;

use bevy::prelude::*;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).init_state::<AppState>();

    let _ = app.run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Library,
    Player,
}
