mod command_handler;
mod database_manager;
mod task_manager;

use command_handler::command_handler_init;
use database_manager::DatabaseManager;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let database_manager = DatabaseManager::new();

    command_handler_init(args, database_manager);
}
