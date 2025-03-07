use std::{
    ffi::OsStr,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use nfo::*;

use audiotags as AT;

use bevy::{prelude::*, utils::HashMap};

pub struct LibraryPlugin;
impl Plugin for LibraryPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn add_book(
    mut commmands: Commands,
    query: Query<(&PathToBook, Entity)>,
    mut authors: ResMut<Authors>,
    mut narators: ResMut<Narators>,
) {
    for (PathToBook(path), entity) in query.iter() {
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

        if title.is_none() || author.is_none() || narator.is_none() {
            if let Some(index) = files
                .iter()
                .position(|(extention, _file)| extention == "m4b")
            {
                let (_extention, m4b_path) = &files[index];
                let tag = AT::Tag::new()
                    .read_from_path(m4b_path)
                    .expect("Could not parse tag");
                let album = tag.album().expect("failed to get album");
                if title.is_none() {
                    title = Some(album.title.to_string());
                }
                if author.is_none() {
                    author = album.artist.map(|s| s.to_string());
                }
            }
        }

        if title.is_none() || author.is_none() {
            if let Some(index) = files
                .iter()
                .position(|(extention, _file)| extention == "mp3")
            {
                let (_extention, m4b_path) = &files[index];
                let tag = AT::Tag::new()
                    .read_from_path(m4b_path)
                    .expect("Could not parse tag");
                let album = tag.album().expect("failed to get album");
                if title.is_none() {
                    title = Some(album.title.to_string());
                }
                if author.is_none() {
                    author = album.artist.map(|s| s.to_string());
                }
            }
        }
        let mut commands = commmands.entity(entity);
        if let Some(title) = title {
            commands.insert(Title(title));
        }
        if let Some(author) = author {
            commands.insert(authors.get_author(&author));
        }
        if let Some(narator) = narator {
            commands.insert(narators.get_narator(&narator));
        }
    }
}

#[derive(Component)]
pub struct PathToBook(pub String);

#[derive(Component)]
pub struct Title(pub String);

#[derive(Resource)]
pub struct Series;

#[derive(Resource, Default)]
pub struct Authors {
    entries: Vec<String>,
    indices: HashMap<String, usize>,
}

impl Authors {
    pub fn get_author(&mut self, author: &str) -> Author {
        Author(self.get_or_insert(author))
    }

    fn get_or_insert(&mut self, author: &str) -> usize {
        if let Some(&index) = self.indices.get(author) {
            index
        } else {
            let index = self.entries.len();
            self.entries.push(author.to_string());
            self.indices.insert(author.to_string(), index);
            index
        }
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|s| s.as_str())
    }

    pub fn get_index(&self, author: &str) -> Option<usize> {
        self.indices.get(author).copied()
    }
}

#[derive(Component)]
pub struct Author(pub usize);

#[derive(Resource)]
pub struct Narators {
    entries: Vec<String>,
    indices: HashMap<String, usize>,
}

impl Narators {
    pub fn get_narator(&mut self, narator: &str) -> Narator {
        Narator(self.get_or_insert(narator))
    }

    fn get_or_insert(&mut self, narator: &str) -> usize {
        if let Some(&index) = self.indices.get(narator) {
            index
        } else {
            let index = self.entries.len();
            self.entries.push(narator.to_string());
            self.indices.insert(narator.to_string(), index);
            index
        }
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|s| s.as_str())
    }

    pub fn get_index(&self, author: &str) -> Option<usize> {
        self.indices.get(author).copied()
    }
}

#[derive(Component)]
pub struct Narator(pub usize);
