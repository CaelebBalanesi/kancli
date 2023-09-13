use rusqlite::Connection;

struct Project {
    id: i32,
    name: String,
}

struct Task {
    id: i32,
    name: String,
    state: String,
    project_id: i32,
}

fn main() {
    let db = setup_database();
    let mut stmt = db.prepare("SELECT id, name FROM projects").unwrap();
    let proj_iter = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap()
        })
    }).unwrap();
    
    println!("Welcome to Kancli");
    println!("What project do you want to open?");
    for proj in proj_iter {
        let project = proj.unwrap();
        print!("[{:}] {}", project.id, project.name);
    }
    println!("[new] New project");
    let mut project = String::new();
    let _ = std::io::stdin().read_line(&mut project);
    if project.trim() == "new" {
        new_project(&db);
    } else {
        open_project(&db, project.trim());
    }
}

fn open_project(db: &Connection, project_id: &str) {
    let mut stmt = db.prepare("SELECT id, name, state FROM tasks WHERE projectId = ?1").unwrap();
    let task_iter = stmt.query_map([project_id], |row| {
        Ok(Task{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            state: row.get(2).unwrap(),
            project_id: project_id.parse::<i32>().unwrap()
        })
    }).unwrap();
    for task in task_iter {
        let task_data = task.unwrap();
        print!("[{}]({}) - {}", task_data.project_id, task_data.state, task_data.name);
    }
    println!("[new] New task");
    let mut task = String::new();
    let _ = std::io::stdin().read_line(&mut task);
    if task.trim() == "new" {
        new_task(db, project_id.parse::<i32>().unwrap());
    } else {
        edit_task(db, project_id.parse::<i32>().unwrap(), task.trim().parse::<i32>().unwrap());
    }
}

fn edit_task(db: &Connection, project_id: i32, task_id: i32) {
    let mut stmt = db.prepare("SELECT * FROM tasks WHERE projectId = ?1 AND id = ?2 LIMIT 1").unwrap();
    let task = stmt.query_row([project_id, task_id], |row| {
        Ok(Task{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            state: row.get(2).unwrap(),
            project_id: row.get(3).unwrap()
        })
    }).unwrap();
    println!("[0] delete\n[1] rename\n[2] change state\n[3] back");
    let mut choice = String::new();
    let _ = std::io::stdin().read_line(&mut choice).unwrap();
    if choice.trim() == "0" {
        db.execute("DELETE FROM tasks WHERE id = :1", [task_id]).unwrap();
    } else if choice.trim() == "1" {
        println!("The current name is {}. What do you want to change it to?", task.name);
        let mut name = String::new();
        let _ = std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        db.execute("UPDATE tasks SET name = ?1 WHERE id = ?2", [name, &task.id.to_string()]).unwrap();
    } else if choice.trim() == "2" {
        println!("The current state is {}. What do you want to change it to?", task.state);
        let mut state = String::new();
        let _ = std::io::stdin().read_line(&mut state).unwrap();
        let state = state.trim();
        db.execute("UPDATE tasks SET state = ?1 WHERE id = ?2", [state, &task.id.to_string()]).unwrap();
    } else {
        println!("tf???");
    }
}

fn new_task(db: &Connection, project_id: i32){
    let mut stmt = db.prepare("SELECT * FROM tasks ORDER BY id DESC LIMIT 1").unwrap();
    let task_id = stmt.query_row((), |row| {
        Ok(Task{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            state: row.get(2).unwrap(),
            project_id: project_id
        })
    }).unwrap_or(Task{
        id: -1,
        name: "blank".to_string(),
        state: String::from("Todo"),
        project_id: project_id

    }).id;
    println!("What is the name of the new task?");
    let mut name = String::new();
    let _ = std::io::stdin().read_line(&mut name);
    db.execute("INSERT INTO tasks (id, name, state, projectId) VALUES (?1, ?2, ?3, ?4)", (task_id + 1, name, String::from("Todo"), project_id)).unwrap();
    println!("Created the new task!");
}

fn new_project(db: &Connection){
    println!("What is the name of the new project?");
    let mut name = String::new();
    let _ = std::io::stdin().read_line(&mut name);
    let mut stmt = db.prepare("SELECT * FROM projects ORDER BY id DESC LIMIT 1").unwrap();
    let project = stmt.query_row((), |row| {
        Ok(Project{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap()
        })
    }).unwrap_or(Project{
        id: -1,
        name: "blank".to_string(),
    });
    db.execute("INSERT INTO projects (id, name) VALUES (?1, ?2)", (project.id + 1, name)).unwrap();
    println!("Created the new project!");
}

fn setup_database() -> Connection{
    let db = Connection::open("./kancli_data.db").unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS projects(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )", ()).unwrap();
    db.execute("CREATE TABLE IF NOT EXISTS tasks(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                state TEXT NOT NULL,
                projectId INTEGER NOT NULL
            )", ()).unwrap();
    db
}
