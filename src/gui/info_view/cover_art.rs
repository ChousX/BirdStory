use super::*;

#[derive(Component)]
pub struct CoverArtView(pub Handle<Image>);
impl GUI for CoverArtView{
    fn build(self, commands: &mut Commands) -> Entity {
        let texture = self.0.clone();
        let root = commands.spawn((
            NodeBundle{
                style: Style{
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            self
        )).id();

        let padding0 = commands.spawn(
            NodeBundle{
                style: Style{
                    height: Val::Percent(100.0),
                    width: Val::Percent(30.0),
                    ..default()
                },
                ..default()
            }
        ).id();
        let padding1 = commands.spawn(
            NodeBundle{
                style: Style{
                    height: Val::Percent(100.0),
                    width: Val::Percent(30.0),
                    ..default()
                },
                ..default()
            }
        ).id();

        let image_holder = commands.spawn((
            NodeBundle{
                style: Style{
                    height: Val::Percent(100.0),
                    width: Val::Percent(40.0),
                    ..default()
                },
                ..default()
            }
        )).id();

        let image = commands.spawn(UiImage{
            color: bevy::color::palettes::css::BLACK.into(),
            texture,
            ..default()
        }).id();
        commands.entity(root).add_child(padding0);
        commands.entity(root).add_child(image_holder);
        commands.entity(root).add_child(padding1);
        commands.entity(image_holder).add_child(image);

        root
    }
}
