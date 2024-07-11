mod series;
mod person;
mod book;
mod author;
mod narrator;

pub use series::*;
pub use person::*;
pub use book::*;
pub use author::*;
pub use narrator::*;

use crate::prelude::*;
use bevy::utils::Duration;


pub struct BiblioPlugin;
impl Plugin for BiblioPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Series>()
            .add_systems(
                Update,
                (add_title, add_authors, add_narrators, add_series)
            );
    }
}

#[derive(Bundle)]
pub struct BookBundleAll{
    pub narrators: Readers,
    pub authors: Writers,
    pub title: Title,
    pub duration: BookDuration,
    pub data_folder: DataFolder,
    pub series: SeriesBook,
}

