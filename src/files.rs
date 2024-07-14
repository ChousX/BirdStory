use bevy::ecs::query::QueryData;
use nfo::{
    Nfo,
    Source,
    GeneralInformation,
    MediaInformation,
    Encoded
};
use std::path::{
    Path, 
    PathBuf
};
use std::fs;
use std::ffi::OsStr;
use std::collections::BinaryHeap as Heap;
use crate::prelude::*;

pub const DEFAULT_INPUT_FOLDER: &str = "/media/Void/pre-books";
pub const DEFAULT_STORAGE_FOLDER: &str = "/media/Void/books";

pub struct FilesPlugin;
impl Plugin for FilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StorageFolder>()
            .init_resource::<InputsFolders>()
            .add_event::<AddNewBooksEvent>()
            .add_event::<MoveBookEvent>()
            .add_event::<FixSingleFiles>()
            .add_event::<GetInfoEvent>()
            .add_systems(
                PreUpdate,
                add_dir_inputs.run_if(on_event::<AddNewBooksEvent>()),
            ).add_systems(
                Update,
                (
                     move_book_to_storage,
                     get_info_from_folder.run_if(on_event::<GetInfoEvent>()),
                     fix_single_files.run_if(on_event::<FixSingleFiles>()),
                )
            ).add_systems(
                Startup,
                init
            );
    }
}

fn init(
    mut add: EventWriter<AddNewBooksEvent>,
){
    add.send(AddNewBooksEvent);
}

#[derive(Resource, Deref, DerefMut)]
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

impl StorageFolder {
    pub fn path<'a>(&'a self) -> &'a Path{
        Path::new(&self.0)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct DataFolder(pub String);

#[derive(Component, Default)]
pub struct UnmovedData;

#[derive(Event, Default)]
pub struct AddNewBooksEvent;


struct FoldersInfo{
    pub folders_with_sub_folders: Vec<String>,
    pub folders_with_no_sub_folders: Vec<String>,
    pub loss_files: Vec<String>,
}

impl FoldersInfo {
    pub fn new(
        root_folder: &str,
    ) -> Result<Self, std::io::Error>{
    
        let root_path = Path::new(root_folder);
        let mut folder_iter = std::fs::read_dir(root_path)?;
        let mut queue = Heap::new();
    
        let mut loss_files = Vec::new();
        for path in folder_iter{
            let path = path?.path();
            let path_str = path.clone().into_os_string().into_string().expect("bad file path");
            if path.is_dir() {
                queue.push(path_str);
            } else {
                loss_files.push(path_str);
            }
        }

        let mut folders_with_no_sub_folders = Vec::new();
        let mut folders_with_sub_folders = Vec::new();

        while let Some(subject) = queue.pop() {
            let subject_path = Path::new(&subject);

            let mut seen_folder = false;
            let mut folder_iter = std::fs::read_dir(subject_path)?;
            for child in folder_iter {
                let child = child?;
                let child_path = child.path();
                if child_path.is_dir() {
                    seen_folder = true;
                    let child_str = child_path.into_os_string().into_string().expect("bad file path str");
                    queue.push(child_str);
                } 
            }

            if seen_folder {
                folders_with_sub_folders.push(subject);
            } else {
                folders_with_no_sub_folders.push(subject)
            }
        }
        Ok(Self {
            loss_files,
            folders_with_no_sub_folders,
            folders_with_sub_folders,
        })
    }
}


fn add_dir_inputs(
    mut commands: Commands,
    input_folders: Res<InputsFolders>,
    mut fix_solo: EventWriter<FixSingleFiles>,
    mut add_info: EventWriter<GetInfoEvent>,
) {
    for folder in input_folders.0.iter(){
        if let Ok(FoldersInfo { 
            folders_with_sub_folders,
            mut folders_with_no_sub_folders, 
            mut loss_files 
        }) = FoldersInfo::new(folder){

            if !loss_files.is_empty() {
                fix_solo.send(FixSingleFiles);
            }

            for book in loss_files.into_iter(){
                let id = commands.spawn((
                    DataFolder(book),
                    UnmovedData,
                    SingleFile,
                )).id();

            }

            for book in folders_with_no_sub_folders.into_iter(){
                let id = commands.spawn((
                    DataFolder(book),
                    UnmovedData,
                )).id();

                add_info.send(GetInfoEvent(id));
            }
        }
    }
}

#[derive(Component)]
pub struct SingleFile;

#[derive(Event)]
pub struct FixSingleFiles;

fn fix_single_files(
    mut loss_files: Query<(&mut DataFolder, Entity), With<SingleFile>>
    , mut commands: Commands,
){
    for (mut file, entity) in loss_files.iter_mut(){
        let file_path = Path::new(&file.0);
        let new_folder_path = {
            let file_name = file_path.file_stem().unwrap();
            let mut file_path = file_path.to_path_buf();
            file_path.pop();
            file_path.push(file_name);
            file_path
        };

        if let Err(err) = copy(&file_path, &new_folder_path){
            panic!("{}", err);
        }

        commands.entity(entity).remove::<SingleFile>();

    }
}

#[derive(Event )]
pub struct MoveBookEvent(pub Entity);

fn move_book_to_storage(
    mut file_dir: Query<(&mut DataFolder, &Title), With<UnmovedData>>,
    mut events: EventReader<MoveBookEvent>,
    mut commands: Commands,
    storage: Res<StorageFolder>,
) {
    let storage = storage.path();
    for MoveBookEvent(book_id) in events.read(){
        if let Ok(( mut book_current_dir, Title(title))) = file_dir.get_mut(*book_id){
            let to = {
                let mut temp = storage.to_path_buf();
                temp.push(title);
                temp
            };

            let from = Path::new(&book_current_dir.0);
            if from.is_file() {
                panic!("Somethings has gone wrong")
            }
            if let Err(err) = copy(&from, &to){
                //Lets see this err!
                panic!("{}", err);
            }
        //Folder has been moved
            //Updating folder Local
            book_current_dir.0 = to.into_os_string().into_string().expect("bad file str");
            
            //removing unmoved_data tag;
            commands.entity(*book_id).remove::<UnmovedData>();
        } else {
            //this should never happen
            panic!("missing book.... not sure how");
        }
    }
}

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Event)]
pub struct GetInfoEvent(pub Entity);

#[derive(Component, Deref, DerefMut)]
pub struct NfoData(pub Nfo);

fn get_info_from_folder(
    mut commands: Commands,
    mut events: EventReader<GetInfoEvent>, 
    mut folders: Query<&DataFolder, (Without<SingleFile>, With<UnmovedData>)>
) {
    for GetInfoEvent(entity) in  events.read(){
        if let Ok(DataFolder(folder_str)) = folders.get(*entity){
            let folder_path = Path::new(&folder_str);
            let mut folder_iter = fs::read_dir(&folder_path).expect("bad file str");
            for file in folder_iter{
                let file_path = file.expect("bad file").path();
                let file_str = file_path.clone().into_os_string().into_string().expect("bad string");
                if let Some(extension) = file_path.extension(){
                    let nfo_str = OsStr::new("nfo");
                    if extension == nfo_str {
                        if let Some(nfo_data) = Nfo::new(file_str){
                            let nfo_data = NfoData(nfo_data);
                            commands.entity(*entity).insert(nfo_data);
                        }
                    }
                }
            }
        }
    }
}

