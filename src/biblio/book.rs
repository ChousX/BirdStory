use super::*;

#[derive(Component, DeRef, DeRefMut)]
pub struct Writers(Vec<Entity>);

#[derive(Component, DeRef, DeRefMut)]
pub struct Readers(Vec<Entity>);

#[derive(Component, DeRef, DeRefMut)]
pub struct BookDuration(Duration);

#[derive(Component, DeRef, DeRefMut)]
pub struct DataFolder(String);

#[derive(Component, DeRef, DeRefMut)]
pub struct Title(String);


