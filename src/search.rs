use crate::prelude::*;
use std::cmp::{
    Ordering,
    Ord,
    PartialOrd,
};
use std::collections::BinaryHeap;

pub struct SearchPlugin;
impl Plugin for SearchPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SearchResults>()
            .add_systems(
                Update,
                search_by_book_title
                //.run_if(on_event::<SearchEvent>())
            )
        ;
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SearchResults(Vec<Entity>);

fn search_by_book_title(
    mut book_titles: Query<(&Title, Entity)>,
    mut search_results: ResMut<SearchResults>,
){
    let mut priority_queue = BinaryHeap::new();
    for (Title(title), entity) in book_titles.iter() {
        priority_queue.push(SortEntity::new(title, entity))
    }
    let sorted: Vec<Entity> = priority_queue.drain().map(|sort_entity|{ sort_entity.id }).collect();
    search_results.0 = sorted;
}

pub struct SortEntity<T> {
    pub key: T,
    pub id: Entity
}

impl<T> SortEntity<T> {
    pub fn new(key: T, id: Entity) -> Self {
        Self{
            key,
            id
        }
    }
}


impl<T: PartialEq> PartialEq for SortEntity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T: PartialOrd> PartialOrd for SortEntity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        self.key.partial_cmp(&other.key) 
    }
}

impl<T: Ord> Ord for SortEntity<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<T: Eq> Eq for SortEntity<T> {}
