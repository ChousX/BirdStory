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
        let files = if path.is_file() {
            let p = PathBuf::from(path);
            let extension = p
                .extension()
                .and_then(|ext| ext.to_str())
                .expect("failed to convert to str")
                .to_string();
            let mut out = HashMap::with_capacity(1);
            out.insert(extension, p);
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

        if let Some(nfo_path) = files.get("nfo") {
            let nfo = Nfo::new(nfo_path).expect("Could not parse nfo file");
            title = nfo.general.title;
            author = nfo.general.author;
            narator = nfo.general.read_by;
            files.remove("nfo");
        }
        if 
        if title.is_some()
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
