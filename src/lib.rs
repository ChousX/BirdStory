mod gui;
mod biblio;
mod files;
mod palette;


pub mod prelude {
  pub use bevy::prelude::*;
  pub use crate::files::{
      InputsFolders,
      StorageFolder,
      DataFolder,
      UnmovedData
  };
  pub use crate::biblio::{
    Title,
  };
  pub use crate::palette::ColorPalette;
}

pub mod plugins {
    pub use crate::gui::GUIPlugin;
    pub use crate::files::FilesPlugin;
    pub use crate::palette::ColorPalettePlugin;
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
