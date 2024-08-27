use rusqlite::{params, Connection, Result};

use crate::task_manager::Task;

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let db = setup_database();
        DatabaseManager { connection: db }
    }

    pub fn new_task(&self, name: String, state: String) {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM tasks ORDER BY id DESC LIMIT 1")
            .unwrap();
        let task_id =
            stmt.query_row((), |row| {
                Ok(Task {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    state: row.get(2).unwrap(),
                })
            })
            .unwrap_or(Task {
                id: -1,
                name: "blank".to_string(),
                state: String::from("Todo"),
            })
            .id + 1;

        self.connection
            .execute(
                "INSERT INTO tasks (id, name, state) VALUES (?1, ?2, ?3)",
                params![task_id, name, state],
            )
            .unwrap();
    }

    pub fn delete_task(&self, task_id: i32) -> Result<usize> {
        self.connection
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_id])
    }

    pub fn update_state(&self, task_id: i32, new_state: String) -> Result<usize> {
        self.connection.execute(
            "UPDATE tasks SET state = ?1 WHERE id = ?2",
            params![new_state, task_id],
        )
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        let mut stmt = self.connection.prepare("SELECT * FROM tasks").unwrap();

        let tasks = stmt
            .query_map((), |row| {
                Ok(Task {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    state: row.get(2).unwrap(),
                })
            })
            .unwrap();

        let task_vec: Vec<Task> = tasks.map(|x| x.unwrap()).collect();
        task_vec
    }
}

fn setup_database() -> Connection {
    let db = Connection::open("./kancli_data.db").unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                state TEXT NOT NULL
            )",
        (),
    )
    .unwrap();
    db
}
