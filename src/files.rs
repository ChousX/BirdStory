use crate::prelude::*;

pub struct FilesPlugin;
impl Plugin for FilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, (
                read_or_init_config, 
                update_book_view_list
            ).chain()
        );
    }
}

#[derive(Resource)]
pub struct InputsFolders(Vec<String>);

#[derive(Resource)]
pub struct StorageFolder(String);

#[derive(Component, Default)]
pub struct UnmovedData;

fn read_or_init_config( mut commands: Commands, event: EventWriter<>){
    /*
    let config = match thing in std::fs::read_to_string(DEFAULT_PATH) {
        Ok(file_contence) => {
            file_contence
        },
        Err(_) => {
            build_config()
        }
    };
    */

    commands.insert_resource(InputsFolders(vec!["test_input".into()]));
    commands.insert_resource(StorageFolder(String::from("./AudioBooks")));
}

/// add_new_
fn update_book_view_list(){}

/// Try to encode everything form the inputs folders, if error add path to error list else deleat
/// input data
fn assimilation( mut commands: Commands, input_folders: Res<InputsFolders>, storage: Res<StorageFolder>){}


fn build_config() -> String{
    String::new()
}

const DEFAULT_PATH: &str = "./.config"; 
