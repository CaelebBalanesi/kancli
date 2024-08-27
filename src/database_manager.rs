use rusqlite::{params, Connection, Result};

use crate::task_manager::{State, Task};

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let db = setup_database();
        DatabaseManager { connection: db }
    }

    pub fn new_task(&self, name: String, state: State) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("SELECT id FROM tasks ORDER BY id DESC LIMIT 1")?;

        let task_id: i32 = stmt.query_row([], |row| row.get(0)).unwrap_or(-1) + 1;

        self.connection.execute(
            "INSERT INTO tasks (id, name, state) VALUES (?1, ?2, ?3)",
            params![task_id, name, state.to_string().to_lowercase()],
        )?;
        Ok(())
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

    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, name, state FROM tasks")?;

        let tasks = stmt.query_map([], |row| {
            let state_str: String = row.get(2)?;
            let state = match state_str.as_str() {
                "inprogress" => State::InProgress,
                "backlog" => State::Backlog,
                "done" => State::Done,
                _ => State::Uncategorized,
            };

            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                state,
            })
        })?;

        Ok(tasks.collect::<Result<Vec<Task>>>()?)
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
