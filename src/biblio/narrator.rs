use super::*;

#[derive(Bundle)]
pub struct NarratorBundle {
    pub person: Person,
    pub works: NarratedWorkes,
}

#[derive(Component, DeRef, DeRefMut)]
pub struct NarratedWorkes(Vec<Entity>);

pub fn add_narrators(
    mut commands: Commands, 
    books: Query<(Entity, &DataFolder), (Without<Readers>, With<UnmovedData>)>,
    narrators: Query(Entity, &mut AuthoredWorks, &Person)
){
     for (id, path) in books.iter() {
        let narrators = {
            let book_narrators = todo!();
        };
        commands.entity(id).insert(Readers(narrators));
    }   
}


