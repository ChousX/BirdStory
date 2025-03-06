use std::{
    ffi::OsStr,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use nfo::*;

use bevy::{prelude::*, utils::HashMap};

pub struct LibraryPlugin;
impl Plugin for LibraryPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn add_book(mut commmands: Commands, query: Query<&PathToBook>) {
    for PathToBook(path) in query.iter() {
        let path = Path::new(&path);
        let mut files = if path.is_file() {
            let p = PathBuf::from(path);
            let extension = p
                .extension()
                .and_then(|ext| ext.to_str())
                .expect("failed to convert to str")
                .to_string();
            let mut out = Vec::with_capacity(1);
            out.push((extension, p));
            out
        } else {
            if let Ok(files) = path.read_dir() {
                files
                    .filter_map(|p| {
                        if let Ok(p) = p {
                            let p = p.path();
                            if let Some(extension) = p
                                .extension()
                                .and_then(|ext| ext.to_str())
                                .map(|s| s.to_string())
                            {
                                Some((extension, p))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                continue;
            }
        };

        let mut title = None;
        let mut author = None;
        let mut narator = None;

        fn is_more_work(
            title: Option<String>,
            author: Option<String>,
            narator: Option<String>,
        ) -> bool {
            title.is_none() || author.is_none() || narator.is_none()
        }

        if let Some(index) = files
            .iter()
            .position(|(extention, _file)| extention == "nfo")
        {
            let (_extention, nfo_path) = &files[index];
            let nfo = Nfo::new(nfo_path).expect("Could not parse nfo file");
            title = nfo.general.title;
            author = nfo.general.author;
            narator = nfo.general.read_by;
        }

        if is_more_work(title, author, narator) {}
    }
}

#[derive(Component)]
pub struct PathToBook(pub String);

#[derive(Component)]
pub struct Title(pub String);

#[derive(Resource)]
pub struct Series;

#[derive(Resource)]
pub struct Author;

#[derive(Resource)]
pub struct Narator;
