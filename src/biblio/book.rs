use super::*;

#[derive(Component, Deref, DerefMut)]
pub struct Writers(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct Readers(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct BookDuration(pub Duration);

#[derive(Component, Deref, DerefMut)]
pub struct Title(pub String);

pub fn add_title(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Title>, With<UnmovedData>)>
){
    for (id, path) in books.iter() {
        let title = todo!();
        commands.entity(id).insert(Title(title));
    }
}


