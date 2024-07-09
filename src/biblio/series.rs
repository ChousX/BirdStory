use super::*;

#[derive(Component, Deref, DerefMut)]
pub struct SeriesWorks(Vec<Entity>);

#[derive(Bundle)]
pub struct SeriesBundle {
    pub series_works: SeriesWorks,
    pub title: Title,
    pub authors: Writers,
    pub narrators: Readers,
}


#[derive(Component)]
pub struct SeriesBook{
    pub id: Entity,
    pub book_number: u16,
}
