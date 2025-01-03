use rusqlite::{params, Connection, Result};
use std::{env, process};

#[derive(Debug)]
struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

struct TodoApp {
    connection: Connection,
}

impl TodoApp {
    fn new() -> Result<Self> {
        let connection = Connection::open("todos.db")?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS todo_item (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
                )",
            (),
        )?;

        Ok(Self { connection })
    }

    fn list(&self) -> Result<()> {
        let mut statement = self
            .connection
            .prepare("SELECT id, title, completed FROM todo_item")?;
        let todo_iter = statement.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?;

        println!("Your to-do list: ");
        for todo_item in todo_iter {
            let todo_item = todo_item.unwrap();
            let checkbox = if todo_item.completed { "[x]" } else { "[ ]" };
            println!("{} {} ({})", checkbox, todo_item.title, todo_item.id)
        }

        Ok(())
    }

    fn add(&self, title: &[String]) -> Result<()> {
        if title.is_empty() {
            eprintln!("'todo add' takes at least one argument.");
            process::exit(1);
        }

        self.connection.execute(
            "INSERT INTO todo_item (title, completed) VALUES (?1, ?2)",
            (title.join(" "), false),
        )?;

        println!("Added task: {}", title.join(" "));

        Ok(())
    }

    fn done(&self, ids: &[String]) -> Result<()> {
        if ids.is_empty() {
            eprintln!("'todo done' takes at least one argument.");
            process::exit(1);
        }

        for id in ids {
            let id = match id.parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Invalid task ID: {}", id);
                    continue;
                }
            };
            let rows_affected = self.connection.execute(
                "UPDATE todo_item SET completed = ?1 WHERE id = ?2",
                params![true, id],
            )?;

            println!("Marked {} task(s) as completed.", rows_affected);
        }

        Ok(())
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - edit [INDEX] [EDITED TASK/s]
        edits an existing task/s
        Example: todo edit 1 banana
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
";

fn help() {
    println!("{}", TODO_HELP);
}

fn main() -> Result<()> {
    // TODO better error handling
    let todo_app = TodoApp::new().expect("Couldn't create the to-do app instance.");
    // TODO use clap
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "list" => todo_app.list()?,
            "add" => todo_app.add(&args[2..])?,
            "done" => todo_app.done(&args[2..])?,
            "help" | _ => help(),

        }
    }

    Ok(())
}
