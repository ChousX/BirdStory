use super::*;

#[derive(Component, DeRef, DeRefMut)]
pub struct Writers(Vec<Entity>);

#[derive(Component, DeRef, DeRefMut)]
pub struct Readers(Vec<Entity>);

#[derive(Component, DeRef, DeRefMut)]
pub struct BookDuration(Duration);

#[derive(Component, DeRef, DeRefMut)]
pub struct Title(String);

pub fn add_title(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Title>, With<UnmovedData>)>
){
    for (id, path) in books.iter() {
        let title = todo!();
        commands.entity(id).insert(Title(title));
    }
}


