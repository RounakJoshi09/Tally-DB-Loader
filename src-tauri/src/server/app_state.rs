use crate::server::database::Database;
use crate::server::tally::Tally;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref TALLY: Mutex<Tally> = Mutex::new(Tally::new());
    pub static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

pub fn get_database() -> std::sync::MutexGuard<'static, Database> {
    DATABASE.lock().expect("Failed to lock DATABASE mutex")
}
pub fn get_tally() -> std::sync::MutexGuard<'static, Tally> {
    TALLY.lock().expect("Failed to lock TALLY mutex")
}
