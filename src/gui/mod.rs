mod selection_view;
mod info_view;


use info_view::*;
use selection_view::*;

use crate::prelude::*;

pub struct GUIPlugin;
impl Plugin for GUIPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, GUIRoot::run);
    }
}

#[derive(Component, Default)]
pub struct GUIRoot;

impl GUIRoot {
    pub fn run(mut commands: Commands){
        let _ = Self::default().build(&mut commands);
    }
}

impl GUI for GUIRoot{
    fn build(self, commands: &mut Commands) -> Entity {
        let root = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                Self
        )).id();
        let selection_view = SelectionView::default().build(commands);
        commands.entity(root).add_child(selection_view);
        let info_view = InfoView::default().build(commands);
        commands.entity(root).add_child(info_view);

        root
    }
}

pub trait GUI: Component{
    fn build(self, commands: &mut Commands) -> Entity;
}

