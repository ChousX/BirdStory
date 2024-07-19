use super::*;

#[derive(Component, Default)]
pub struct SelectionsView;

impl GUI for SelectionsView {
    fn build(self, commands: &mut Commands) -> Entity {
        let selections_view = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(69.5),
                        ..default()
                    },
                    ..default()
                },
                self
        )).id();
        selections_view
    }
}

#[derive(Component, Default)]
pub struct Selection;

//This one should not use the default GUI trait
impl Selection{
    fn build(self, commands: &mut Commands) -> Entity {
        let selection = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),//todo!
                        width: Val::Percent(100.0), //todo!
                        ..default()
                    },
                    ..default()
                },
                self
        )).id();

        selection
    }
}
