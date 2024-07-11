use crate::prelude::*;
use bevy::render::render_graph::Node;
use bevy_simple_text_input::{
    TextInputBundle,
    TextInputInactive,
    TextInputPlugin,
    TextInputSubmitEvent,
    TextInputSystem,
    TextInputValue
};

pub struct GUIPlugin;
impl Plugin for GUIPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .add_event::<SearchEvent>()
            .add_systems(
                Startup, 
                (init_gui, init_camera)
            )
            .add_systems(
                Update,
                focus.before(TextInputSystem)
            ).add_systems(
                Update, 
                search_button_system.after(TextInputSystem)
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

#[derive(Component)]
pub struct SearchText;

#[derive(Component)]
pub struct SearchButton;

#[derive(Component)]
pub struct SelectionView;


fn init_gui( mut commands: Commands, colors: Res<ColorPalette>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
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
                flex_direction: FlexDirection::Column,
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
                flex_direction: FlexDirection::Column,
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
                    .with_placeholder( "Search", None)
                    .with_inactive(true),
                SearchText,
            )).id();

        let search_button =  commands.spawn((
                ButtonBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(10.0),
                        ..default()
                    },
                    background_color: colors.search_button_normal().into(),
                    border_color: colors.search_button_hovered().into(),
                    ..default()
                },
                SearchButton,
            )).id();

        commands.entity(searchbar).add_child(search_text);
        commands.entity(searchbar).add_child(search_button);
        commands.entity(brows_view).add_child(searchbar);
    }

    commands.entity(root).add_child(brows_view);
    commands.entity(root).add_child(info_view);
    
    {
        let selection_view = commands.spawn((NodeBundle{
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: colors.selection_view().into(),
            ..default()
            },
            SelectionView,
        )).id();
        
        commands.entity(brows_view).add_child(selection_view);
    }

    {
        let cover_art_viewer = commands.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(45.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                background_color: colors.cover_art_viewer().into(),
                ..default()
            },
            CoverArtViewer,
        )).id();

        {
            let padding0 = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(20.0),
                        ..default()
                    },
                    background_color: colors.cover_art_padding().into(),
                    ..default()
                
                },
            )).id();

            let padding1 = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(20.0),
                        ..default()
                    },
                    background_color: colors.cover_art_padding().into(),
                    ..default()
                
                },
            )).id();


            let art_view = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(60.0),
                        ..default()
                    },
                    ..default()
                
                },
            )).id();

            commands.entity(cover_art_viewer).add_child(padding0);
            commands.entity(cover_art_viewer).add_child(art_view);
            commands.entity(cover_art_viewer).add_child(padding1);
        }

        let more_info_viewer = commands.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(55.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                background_color: colors.more_info_viewer().into(),
                ..default()
            },
            MoreInfoViewer
        )).id();

        commands.entity(info_view).add_child(cover_art_viewer);
        commands.entity(info_view).add_child(more_info_viewer);
    }
}

fn init_camera(mut commands: Commands) { 
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

fn focus(
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<SearchButton>)>,
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

#[derive(Event)]
pub struct SearchEvent;

fn search_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    colors: Res<ColorPalette>,
    mut search: EventWriter<SearchEvent>
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.search_button_pressed().into();
                border_color.0 = colors.search_button_pressed().into();
                search.send(SearchEvent);
            }       
            Interaction::Hovered => {
                *color = colors.search_button_hovered().into();
                border_color.0 = colors.search_button_hovered().into();
            }
            Interaction::None => {
                *color = colors.search_button_normal().into();
                border_color.0 = colors.search_button_normal().into();
            }
        }
    }
}

#[derive(Component)]
pub struct SelectionSelected;

pub struct SelectionViewUpdateEvent;


fn selection_view_populator(

){

}

#[derive(Component)]
pub struct CoverArtViewer;

#[derive(Component)]
pub struct MoreInfoViewer;
