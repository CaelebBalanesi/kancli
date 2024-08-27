use std::fmt;

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub state: State,
}

#[derive(Debug)]
pub enum State {
    Backlog,
    InProgress,
    Done,
    Uncategorized,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
