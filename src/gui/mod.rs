use crate::prelude::*;
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSubmitEvent, TextInputSystem, TextInputInactive};

pub struct GUIPlugin;
impl Plugin for GUIPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .add_systems(
                Startup, 
                (init_gui, init_camera)
            )
            .add_systems(
                Update,
                focus.before(TextInputSystem)
            );
    }
}


#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SortingBy {
    Author,
    Narator,
    Seires,
    #[default]
    Book,
}

#[derive(Resource)]
pub struct GUIRoot(Entity);

#[derive(Resource)]
pub struct BrowsView(Entity);

#[derive(Resource)]
pub struct InfoView(Entity);

fn init_gui( mut commands: Commands, colors: Res<ColorPalette>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }).id();
    
    let brows_view = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(33.25),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            background_color: colors.brows_view().into(),
            ..default()
        }).id();

    let info_view = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(66.5),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: colors.info_view().into(),
            ..default()
        }).id();

    {
        let searchbar = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceBetween,
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: colors.search_bar_border().into(),
                    background_color: colors.search_bar_background().into(),
                    ..default()
                }, 
                
            )).id();

        let search_text = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(90.0),
                        ..default()
                    },
                    ..default()
                },
                TextInputBundle::default().with_text_style(TextStyle {
                    font_size: 40.,
                    color: colors.search_text(),
                    ..default()
                })
                    .with_placeholder( "Click Me", None)
                    .with_inactive(true),
            )).id();

        let search_button =  commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(10.0),
                        ..default()
                    },
                    background_color: colors.search_bar_background().into(),
                    border_color: colors.border_inactive().into(),
                    ..default()
                },
            )).id();

        commands.entity(searchbar).add_child(search_text);
        commands.entity(searchbar).add_child(search_button);
        commands.entity(brows_view).add_child(searchbar);
    }

    commands.entity(root).add_child(brows_view);
    commands.entity(root).add_child(info_view);

    commands.insert_resource(BrowsView(brows_view));
    commands.insert_resource(InfoView(info_view));
}

fn init_camera(mut commands: Commands) { 
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
    colors: Res<ColorPalette>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = colors.border_active().into();
                } else {
                    inactive.0 = true;
                    *border_color = colors.border_inactive().into();
                }
            }
        }
    }
}
