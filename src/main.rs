mod cornucopia;
mod model;
mod repository;

use std::error::Error;

use cornucopia::queries::crud;
use model::TodoEntity;
use model::TodoRecord;
use postgres::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = get_client().unwrap();
    let rec = TodoRecord {
        title: "title".to_string(),
        description: "none".to_string(),
        done: false,
    };
    crud::incert().bind(&mut client, &rec.title, &rec.description, &rec.done)?;
    let result: Vec<TodoEntity> = crud::select_all()
        .bind(&mut client)
        .all()
        .unwrap()
        .iter()
        .map(TodoEntity::from)
        .collect();
    dbg!(result);
    Ok(())
}

use postgres::{Config, NoTls};

fn get_client() -> Result<postgres::Client, postgres::Error> {
    Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5444)
        .dbname("postgres")
        .connect(NoTls)
}
