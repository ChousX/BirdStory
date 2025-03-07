use std::{
    ffi::OsStr,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use nfo::*;

use audiotags as AT;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

pub struct LibraryPlugin;
impl Plugin for LibraryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateLibraryEvent>().add_systems(
            Update,
            (regester_books, add_book)
                .chain()
                .distributive_run_if(on_event::<UpdateLibraryEvent>),
        );
    }
}

#[derive(Event)]
pub struct UpdateLibraryEvent;

pub fn regester_books(
    mut commands: Commands,
    library_storage: Query<&LibraryStorage>,
    paths_to_books: Query<&PathToBook>,
) {
    let mut already_loaded = HashSet::default();
    for PathToBook(path) in paths_to_books.iter() {
        already_loaded.insert(PathBuf::from(path));
    }
    for LibraryStorage(root) in library_storage.iter() {
        let dir_iter = root
            .read_dir()
            .expect("Library Storage was not set to directory");

        for dir_entry in dir_iter {
            if let Ok(dir_entry) = dir_entry {
                let path = dir_entry.path();
                if !already_loaded.contains(&path) {
                    commands.spawn(PathToBook(path));
                }
            }
        }
    }
}

pub fn add_book(
    mut commmands: Commands,
    query: Query<(
        &PathToBook,
        Entity,
        Option<&Author>,
        Option<&Narator>,
        Option<&Title>,
    )>,
    mut authors: ResMut<Authors>,
    mut narators: ResMut<Narators>,
) {
    for (PathToBook(path), entity, author, narator, title) in query.iter() {
        if author.is_some() & narator.is_some() & title.is_some() {
            continue;
        }
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

        let mut title = title.map(|s| s.0.to_string());
        let mut author = author.map(|s| s.0.to_string());
        let mut narator = narator.map(|s| s.0.to_string());

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
pub struct LibraryStorage(pub PathBuf);

#[derive(Component)]
pub struct PathToBook(pub PathBuf);

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
