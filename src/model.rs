use crate::cornucopia::queries::crud::SelectAll;

#[derive(Debug, Clone)]
pub struct TodoRecord {    
    pub title: String,
    pub description: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct TodoEntity {
    pub id: i64,
    pub record: TodoRecord
}

impl From<&SelectAll> for TodoEntity {
    fn from(value: &SelectAll) -> Self {
        Self { id: value.id, record: TodoRecord{title: (value.title.clone()), description: (value.description.clone()), done: (value.done)} }
    }
}
