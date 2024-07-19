use super::*;

#[derive(Component)]
pub struct SearchBar{
    size: Val,
}

impl Default for SearchBar {
    fn default() -> Self{
        Self{
            size: Val::Px(200.0),
        }
    }
}

impl GUI for SearchBar {
    fn build(self, commands: &mut Commands) -> Entity {
        let search_bar = commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: self.size.clone(),
                    ..default()
                },
                ..default()
            },
            self
        )).id();

        search_bar
    }
}

#[derive(Default, Component)]
pub struct SearchButton;

impl GUI for SearchButton {
    fn build(self, commands: &mut Commands) -> Entity {
        let button = commands.spawn((
            ButtonBundle {
                ..default()
            },
            self
        )).id();

        button
    }
}


