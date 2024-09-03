use crate::task_manager::{State, Task};
use rusqlite::InterruptHandle;
use term_size::dimensions;

pub fn draw_tasks(tasks: Vec<Task>) {
    let (x, y) = dimensions().unwrap();
    let third = ((x - 3) as f32 / 3.).floor() as usize;
    let top = "─".repeat(third);
    println!("┌{}┬{}┬{}┐", top, top, top);
    println!(
        "│Backlog{}│In Progress{}│Done{}│",
        " ".repeat(third - 7),
        " ".repeat(third - 11),
        " ".repeat(third - 4)
    );

    let mut backlog: Vec<Task> = vec![];
    let mut inprogress: Vec<Task> = vec![];
    let mut done: Vec<Task> = vec![];
    let mut uncategorized: Vec<Task> = vec![];

    for task in tasks {
        match task.state {
            State::Backlog => {
                backlog.push(task);
            }
            State::InProgress => {
                inprogress.push(task);
            }
            State::Done => {
                done.push(task);
            }
            State::Uncategorized => {
                uncategorized.push(task);
            }
        }
    }

    let mut max_tasks = 0;

    if backlog.len() > max_tasks {
        max_tasks = backlog.len();
    } else if inprogress.len() > max_tasks {
        max_tasks = inprogress.len();
    } else if done.len() > max_tasks {
        max_tasks = done.len();
    } else if uncategorized.len() > max_tasks {
        max_tasks = uncategorized.len();
    }

    for i in 0..max_tasks {
        print!("│");
        if backlog.len() > i {
            let task_string = format!("[{}] {}", backlog[i].id, backlog[i].name);
            print!("{}{}", task_string, " ".repeat(third - task_string.len()));
        } else {
            print!("{}", " ".repeat(third));
        }
        print!("│");
        if inprogress.len() > i {
            let task_string = format!("[{}] {}", inprogress[i].id, inprogress[i].name);
            print!("{}{}", task_string, " ".repeat(third - task_string.len()));
        } else {
            print!("{}", " ".repeat(third));
        }
        print!("│");
        if done.len() > i {
            let task_string = format!("[{}] {}", done[i].id, done[i].name);
            print!("{}{}", task_string, " ".repeat(third - task_string.len()));
        } else {
            print!("{}", " ".repeat(third));
        }
        println!("│");
    }
    println!("└{}┴{}┴{}┘", top, top, top);
}
