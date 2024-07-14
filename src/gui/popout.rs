use crate::prelude::*;
use bevy::{render::camera::RenderTarget, window::WindowRef};

pub struct PopOutWindowPlugin;
impl Plugin for PopOutWindowPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnPopOutWindowEvent>()
            .add_systems(Update, spawn_pop_out_window.run_if(on_event::<SpawnPopOutWindowEvent>()));
    }
}

#[derive(Component)]
pub struct PopOutWindow;

#[derive(Component)]
pub struct PopOutWindowCamera(pub Entity);


#[derive(Event)]
pub struct SpawnPopOutWindowEvent(pub Entity);
fn spawn_pop_out_window(
    mut commands: Commands,
    mut events: EventReader<SpawnPopOutWindowEvent>,
    mut colors: Res<ColorPalette>,
){
    for event in events.read() {
        let ui = event.0;
        let window_id = commands.spawn((
            Window {
                title: "PopOut".into(),
                ..default()
            },
            PopOutWindow,
        )).id();
    
        let camera_id = commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    target: RenderTarget::Window(WindowRef::Entity(window_id)),
                    ..default()
                },
                ..default()
            },
            PopOutWindowCamera(window_id),
        )).id();

        let ui_root = commands
            .spawn((
                NodeBundle{
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(camera_id),
            )).id();
        commands.entity(ui_root).add_child(*ui);
    }
}
