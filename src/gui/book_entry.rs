use super::*;

#[derive(Default)]
pub struct BookEntryBuilder{
    title: Option<String>,
}

///Add Section 
impl BookEntryBuilder {
    pub fn add_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

///Build Section
impl BookEntryBuilder {
    pub fn build(self, commands: &mut Commands) -> Entity{
        let root = commands.spawn((
                NodeBundle{
                    style: Style {
                        height: Val::Px(100.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                },
        )).id();

        let image = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Px(100.0),
                        ..default()
                    },
                    background_color: bevy::color::palettes::css::BLACK.into(),
                    ..default()
                },
        )).id();

        let info_block = commands.spawn((
            NodeBundle{
                style: Style{
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        )).id();

        if let Some(title_str) = self.title{
            let title = commands.spawn((
                TextBundle::from_section(title_str, TextStyle{
                    ..default()
                }),
            )).id();
            commands.entity(info_block).add_child(title);
        }

        commands.entity(root).add_child(image);
        commands.entity(root).add_child(info_block);

        root
    }
}

#[derive(Event)]
pub struct AddGUIBookEvent(pub Entity);

pub fn add_gui_book(
    mut events: EventReader<AddGUIBookEvent>,
    mut titles: Query<&Title>,
    mut root: Query<Entity, With<_>>,
    mut commands: Commands,
){
    for AddGUIBookEvent(book_entity) in events.read(){
        let mut book_builder = BookEntryBuilder::default();

        if let Ok(Title(title)) = titles.get(book_entity){
            book_builder = book_builder.add_title(title);
        }

        let built = book_builder.build(&mut commands);
        let root_entity = root.single();
        commands.entity(root_entity).add_child(built);
    }
}
