mod cover_art;
mod info_details_view;


use bevy::utils::info;

use super::*;

#[derive(Component, Default)]
pub struct InfoView;

impl GUI for InfoView{
    fn build(self, commands: &mut Commands) -> Entity {
        let info_view = commands.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0), // todo!(),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                self
        )).id();

        info_view
    }
}
