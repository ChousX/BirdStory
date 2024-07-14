use super::*;

#[derive(Component, Deref, DerefMut)]
pub struct Writers(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct Readers(pub Vec<Entity>);

#[derive(Component, Deref, DerefMut)]
pub struct BookDuration(pub Duration);

#[derive(Component, Deref, DerefMut)]
pub struct Title(pub String);

pub fn add_using_nfo(
    mut commands: Commands, 
    books: Query<(Entity, &NfoData), (Without<Title>, With<UnmovedData>)>
){
    for (id, data) in books.iter() {
        let data = &data.0;
        let general = &data.general;
        let mut entity_commands = commands.entity(id);

        if let Some(title) = &general.title {
            entity_commands.insert(Title(title.clone()));
        }

        if let Some(author) = &general.author {
            //entity_commands.insert.insert()
        }

        if let Some(narrator) = &general.read_by {

        }

    }
}

pub fn show_titles(q: Query<&Title>){ 
    for q in q.iter() {
        dbg!(&q.0);
    }
}

