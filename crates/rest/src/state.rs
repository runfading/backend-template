use sea_orm::DatabaseConnection;

/// Shared infrastructure used to construct each business router state.
#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub(crate) fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
