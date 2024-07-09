mod series;
mod person;
mod book;
mod author;
mod narrator;

use crate::prelude::*;
use bevy::utils::Duration;


pub struct BiblioPlugin;
impl Plugin for BiblioPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<AddNewBooksEvent>()
            .add_systems(
                Update,
                init_books.run_if(on_event::<AddNewBooksEvent>()))
            .add_systems(
                Update, (
                    add_title, add_authors, add_narrators 
                ).distributive_run_if(
                    on_event::<AddNewBooksEvent>()
                ).after(init_books)
            )
    }
}

fn init_book(
    mut commands: Commands,
    input_folders: Res<InputsFolders>,
){
    for folder in input_folders.iter() {
        for data in std::fs::read_dir(folder).expect("could not read input_folder") {
            let data = data.unwrap();
            if data.path().is_dir(){
                commands
                    .spawn((
                        DataFolder(data.path().into()),
                        UnmovedData,
                    ));
            } else {
                //makefolder
                //move stray file into folder
                //spawn data_folder
            }
        }
    }
}

fn add_title(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Title>, With<UnmovedData>)>
){
    for (id, path) in books.iter() {
        let title = todo!();
        commands.entity(id).insert(Title(title));
    }
}

fn add_authors(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Writers>, With<UnmovedData>)>,
    authors: Query<(Entity, &mut AuthoredWorks, &Person)>,
){
    for (id, path) in books.iter() {
        let authors = {
            let book_authors = todo!();
        };
        commands.entity(id).insert(Writers(authors));
    }
}

fn add_narrators(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Readers>, With<UnmovedData>)>,
    narrators: Query(Entity, &mut AuthoredWorks, &Person)
){
     for (id, path) in books.iter() {
        let narrators = {
            let book_narrators = todo!();
        };
        commands.entity(id).insert(Readers(narrators));
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

#[derive(Event, Default)]
pub struct AddNewBooksEvent;


#[derive(Event, Default)]
pub struct MigrateBooks(pub Entity);

