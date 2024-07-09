use super::*;

#[derive(Bundle)]
pub struct AuthorBundle {
    pub person: Person,
    pub works: AuthoredWorks
}

#[derive(Component, DeRef, DeRefMut)]
pub struct AuthoredWorks(Vec<Entity>);

