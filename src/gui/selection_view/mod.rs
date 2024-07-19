mod search_bar;
mod selections_view;

pub use search_bar::*;
pub use selections_view::*;

use super::*;

pub const SELECTION_VIEW_WIDTH: f32 = 30.0;


#[derive(Component, Default)]
pub struct SelectionView;
impl GUI for SelectionView{
    fn build(self, commands: &mut Commands) -> Entity {
        let selection_view_root = commands.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Percent(SELECTION_VIEW_WIDTH),
                        ..default()
                    },
                    ..default()
                },
                self,
        )).id();

        let search_bar = SearchBar::default().build(commands);
        commands.entity(selection_view_root).add_child(search_bar);
        let selections_view = SelectionsView::default().build(commands);
        commands.entity(selection_view_root).add_child(selections_view);

        selection_view_root
    }
}
