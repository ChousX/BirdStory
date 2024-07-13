use super::*;
use std::collections::BTreeMap;

#[derive(Resource, Default)]
pub struct Series{
    data: BTreeMap<String, Entity>,
}

impl Series {
    pub fn get(&self, title: &str) -> Option<Entity> {
        self.data.get(title).copied()
    }

    pub fn add(&mut self, title: String, id: Entity) {
        let _ = self.data.insert(title, id).expect("Hmmm there was alrady a seires with this title; shits gone wild");
    }
}

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
impl SeriesBook {
    pub fn new(id: Entity, book_number: usize) -> Self{
        let book_number = book_number as u16;
        Self { id, book_number}
    }
}

pub fn add_series(
    mut commands: Commands,
    mut books: Query<(&DataFolder, Entity), (With<UnmovedData>, Without<SeriesBook>)>,
    mut series: ResMut<Series>,
    mut series_works: Query<&mut SeriesWorks>
){
    for (book_path, entity) in books.iter(){
        let title = {todo!()};
        if let Some(id) = series.get(title){
            if let Ok(mut works) = series_works.get_mut(id){
                let len = works.0.len(); 
                works.0.push(entity);
                commands.entity(entity).insert(SeriesBook::new(id, len));
            }
        } else {
            let series_works = SeriesWorks(vec![entity]);
            let id = commands.spawn(
                (
                    series_works,
                    Title(title.to_string()),
                
            )).id();
            series.add(title.to_string(), id);
            commands.entity(entity).insert(SeriesBook::new(id, 0));
        }
    }
}

