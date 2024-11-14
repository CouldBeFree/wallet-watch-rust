use mongodb::Database;

#[derive(Clone)]
pub struct AppState {
    pub(crate) db_conn: Database
}

impl AppState {
    pub fn new(db: Database) -> AppState {
        AppState {
            db_conn: db
        }
    }
}


