mod gui;
mod biblio;
mod files;
mod palette;
mod search;


pub mod prelude {
  pub use bevy::prelude::*;
  pub use crate::files::{
      InputsFolders,
      StorageFolder,
      DataFolder,
      UnmovedData,
      NfoData,
  };
  pub use crate::biblio::Title;
  pub use crate::palette::ColorPalette;
  pub use crate::gui::{
        SearchEvent,
  };
  pub use crate::search::SearchResults;
}

pub mod plugins {
    pub use crate::gui::GUIPlugin;
    pub use crate::files::FilesPlugin;
    pub use crate::palette::ColorPalettePlugin;
    pub use crate::search::SearchPlugin;
    pub use crate::biblio::BiblioPlugin;
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
