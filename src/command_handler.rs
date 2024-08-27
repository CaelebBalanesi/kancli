use crate::database_manager::DatabaseManager;
use crate::task_manager::Task;

pub fn command_handler_init(mut args: Vec<String>, db_manager: DatabaseManager) {
    if args[0] == "add" {
        args.remove(0);
        add(args, db_manager);
    } else if args[0] == "update" {
        args.remove(0);
        update(args, db_manager);
    } else if args[0] == "delete" {
        args.remove(0);
        delete(args, db_manager);
    } else if args[0] == "version" {
        println!("v1.0.0");
    } else if args[0] == "help" {
        help();
    } else if args[0] == "list" {
        list(args, db_manager);
    } else {
        println!("Unknown Arguments");
    }
}

fn add(args: Vec<String>, db_manager: DatabaseManager) {
    db_manager.new_task(args[0].clone(), args[1].clone());
}

fn delete(args: Vec<String>, db_manager: DatabaseManager) {
    db_manager
        .delete_task(args[0].parse::<i32>().unwrap())
        .unwrap();
}

fn update(args: Vec<String>, db_manager: DatabaseManager) {
    db_manager
        .update_state(args[0].parse::<i32>().unwrap(), args[1].clone())
        .unwrap();
}

fn list(args: Vec<String>, db_manager: DatabaseManager) {
    let tasks: Vec<Task> = db_manager.get_all_tasks();
    for task in tasks {
        println!("{:?}", task);
    }
}

fn help() {
    println!("add - \nupdate - \ndelete - \nlist - \nversion - ");
}
