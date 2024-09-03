use crate::database_manager::DatabaseManager;
use crate::graphics;
use crate::task_manager::{State, Task};

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
        version();
    } else if args[0] == "help" {
        help();
    } else if args[0] == "list" {
        list(args, db_manager);
    } else {
        println!("Unknown Arguments");
    }
}

fn add(args: Vec<String>, db_manager: DatabaseManager) {
    db_manager
        .new_task(args[0].clone(), State::Backlog)
        .unwrap();
}

fn delete(args: Vec<String>, db_manager: DatabaseManager) {
    db_manager
        .delete_task(args[0].parse::<i32>().unwrap())
        .unwrap();
}

fn update(args: Vec<String>, db_manager: DatabaseManager) {
    println!("To what state would you like to change to?\n[1] Backlog\n[2] In Progress\n[3] Done");
    let mut new_state_str = String::new();

    let _ = std::io::stdin().read_line(&mut new_state_str);

    println!("{}", new_state_str);

    let state: State = match new_state_str.trim() {
        "1" => State::Backlog,
        "2" => State::InProgress,
        "3" => State::Done,
        _ => State::Uncategorized,
    };

    db_manager
        .update_state(args[0].parse::<i32>().unwrap(), state)
        .unwrap();
}

fn list(args: Vec<String>, db_manager: DatabaseManager) {
    let tasks: Vec<Task> = db_manager.get_all_tasks().unwrap();
    graphics::draw_tasks(tasks);
}

fn help() {
    println!("add \"{{name of task}}\"\nupdate {{task id}}\ndelete {{task id}} \nlist\nversion");
}

fn version() {
    println!("v1.0.0");
}
