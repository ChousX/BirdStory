use super::*;

#[derive(Bundle)]
pub struct AuthorBundle {
    pub person: Person,
    pub works: AuthoredWorks
}

#[derive(Component, DeRef, DeRefMut)]
pub struct AuthoredWorks(Vec<Entity>);

pub fn add_authors(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Writers>, With<UnmovedData>)>,
    authors: Query<(Entity, &mut AuthoredWorks, &Person)>,
){
    for (id, path) in books.iter() {
        let authors = {
            let book_authors = todo!();
        };
        commands.entity(id).insert(Writers(authors));
    }
}
