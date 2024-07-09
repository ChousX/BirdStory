use crate::prelude::*;

pub struct GUIPlugin;
impl Plugin for GUIPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_gui, init_camera));
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

fn init_gui( mut commands: Commands) {
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
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            background_color: Color::hsl(344.0, 100.0, 50.0).into(),
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
            background_color: Color::hsl(205.0, 100.0, 50.0).into(),
            ..default()
        }).id();

    let searchbar = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }).id();
    commands.entity(root).add_child(brows_view);
    commands.entity(root).add_child(info_view);

    commands.insert_resource(BrowsView(brows_view));
    commands.insert_resource(InfoView(info_view));
}

fn init_camera(mut commands: Commands) { 
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}
