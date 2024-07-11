use crate::prelude::*;

pub const DEFAULT_INPUT_FOLDER: &str = "/media/Void/pre-books";
pub const DEFAULT_STORAGE_FOLDER: &str = "/media/Void/books";

pub struct FilesPlugin;
impl Plugin for FilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StorageFolder>()
            .init_resource::<InputsFolders>()
            .add_systems(
                Update,
                add_dir_inputs.run_if(on_event::<AddNewBooksEvent>())
            ).add_systems(
                Update, 
                (
                    //I hope this works
                    check_folders.after(add_dir_inputs),
                    check_folders.befor(move_book_to_storage)
                )
            );
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct InputsFolders(Vec<String>);
impl Default for InputsFolders {
    fn default() -> Self {
       Self(vec![dbg!(DEFAULT_INPUT_FOLDER.into())])
    }
}

#[derive(Resource)]
pub struct StorageFolder(String);
impl Default for StorageFolder {
    fn default () -> Self{
        Self(dbg!(DEFAULT_STORAGE_FOLDER.into()))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DataFolder(pub String);

#[derive(Component, Default)]
pub struct UnmovedData;

#[derive(Event, Default)]
pub struct AddNewBooksEvent;

fn add_dir_inputs(
    mut commands: Commands,
    input_folders: Res<InputsFolders>,
) {
    for folder in *input_folders.iter(){
        let dir_path = Path::new(folder);
        if let Ok(dir_iter) = std::fs::read_dir(dir_path){
            for book in dir_iter {
                if let Ok(book) = book {
                    commands
                        .spawn((DataFolder(book.path().into()), UnmovedData));
                }
            }
        }
    }
}


#[derive(Event, Default)]
pub struct MoveBookEvent(pub Entity);

fn check_folders(
    mut commands: Commands,
    mut files: Query<(&DataFolder, Entity), With<UnmovedData>>,
) {
    let mut books = Vec::new();
    for (dir_path, entity) in files.iter() {
        if let Ok(file_iter) = std::fs::read_dir(dir_path) {
            for DataFolder(entry) in file_iter {
                if let Ok(path) = entry.path(){
                    if path.is_dir() {
                        books.push(path)
                    } else {}
                }
            }
        }
        if !books.is_empty() {
            commands.entity(entity).despawn();
            for new_book in books.drain(){
                commands.spawn((
                    DataFolder(new_book.into()),
                    UnmovedData,
                ));
            }
        }
    }
}

fn move_book_to_storage(
    mut file_dir: Query<&DataFolde, With<UnmovedData>>,
    mut events: EventReader<MoveBookEvent>,
    mut commands: Commands,
    storage_dir: Res<StorageFolder>,
) {
    for MoveBookEvent(book_id) in events.iter(){
        if let Ok(book_current_dir) = file_dir.get(book_id){
            //Do stuff to move book over to storage
            //After moved remove eg: 
            //  commands.entity(book_id).remove_component::<UnmovedData>();
            todo!()
        } else {
            panic!("missing book.... not sure how");
        }
    }
}
