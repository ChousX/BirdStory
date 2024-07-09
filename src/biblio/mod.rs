mod series;
mod person;
mod book;
mod author;
mod narrator;

use series::*;
use person::*;
use book::*;
use author::*;
use narrator::*;

use crate::prelude::*;
use bevy::utils::Duration;


pub struct BiblioPlugin;
impl Plugin for BiblioPlugin{
    fn build(&self, app: &mut App) {
        app

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

