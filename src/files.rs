use crate::prelude::*;

pub struct FilesPlugin;
impl Plugin for FilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StorageFolder>()
            .init_resource::<InputsFolders>();
    }
}

#[derive(Resource, Default)]
pub struct InputsFolders(Vec<String>);

#[derive(Resource)]
pub struct StorageFolder(String);
impl Default for StorageFolder {
    fn default () -> Self{
        Self(".AudioBooks".into())
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DataFolder(String);

#[derive(Component, Default)]
pub struct UnmovedData;

#[derive(Event, Default)]
pub struct AddNewBooksEvent;


#[derive(Event)]
pub struct MigrateBooks(pub Entity);

fn build_config() -> String{
    String::new()
}

const DEFAULT_PATH: &str = "./.config"; 
