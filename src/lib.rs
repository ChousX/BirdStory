mod gui;
mod biblio;
mod files;


pub mod prelude {
  pub use bevy::prelude::*;
  pub use crate::files::{InputsFolders, StorageFolder};
  pub use crate::biblio::{
    Title,
    DataFolder,
  };
}

pub mod plugins {
    pub use crate::gui::GUIPlugin;
    pub use crate::files::FilesPlugin;
}

mod app {
  use crate::prelude::*;
  #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
  pub enum AppState {
    #[default]
    Entry,
  }

  pub const APP_NAME: &str = "StoryBird";
}
