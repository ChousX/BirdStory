mod cover_art;
mod info_details_view;


use super::*;

#[derive(Component)]
pub struct InfoView;

impl GUI for InfoView{
    fn build(&self, commands: &mut Commands) -> Entity {
        todo!()
    }
}
