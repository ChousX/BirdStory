mod selection_view;
mod info_view;


use crate::prelude::*;

pub struct GUIPlugin;
impl Plugin for GUIPlugin{
    fn build(&self, app: &mut App) {
        
    }
}

/*
#[derive(Component)]
pub struct GUIRoot;
impl GUI for GUIRoot{
    fn build(&self, commands: &mut Commands) -> Entity {
    }
}
*/

pub trait GUI: Component{
    fn build(&self, commands: &mut Commands) -> Entity;
}

