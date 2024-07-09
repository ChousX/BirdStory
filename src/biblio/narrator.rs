use super::*;

#[derive(Bundle)]
pub struct NarratorBundle {
    pub person: Person,
    pub works: NarratedWorkes,
}

#[derive(Component, DeRef, DeRefMut)]
pub struct NarratedWorkes(Vec<Entity>);


