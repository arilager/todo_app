// use std::env::VarError;
// use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::{env, io, process};

pub struct TodoApp {
    pub todo_list: Vec<String>,
    pub file_path: String,
    pub backup_path: String,
    pub is_backup: bool,
}

impl TodoApp {
    pub fn new() -> Result<Self, String> {
        let file_path: String = env::var("TODO_PATH").unwrap_or_else(|_| {
            let home = env::var("HOME").unwrap();
            format!("{}/.todo", &home)
        });

        let backup_path: String =
            env::var("TODO_BAK_PATH").unwrap_or_else(|_| String::from("/tmp/todo.bak"));

        let is_backup = env::var("TODO_BACKUP").is_ok();

        let todo_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&file_path)
            .expect("Couldn't open the to-do file");

        let mut buf_reader = BufReader::new(&todo_file);
        let mut file_content = String::new();
        buf_reader.read_to_string(&mut file_content).unwrap();
        // TODO to Vec<TodoItem>
        let todo_list = file_content.lines().map(str::to_string).collect();

        Ok(Self {
            todo_list,
            file_path,
            backup_path,
            is_backup,
        })
    }

    pub fn list(&self) {
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout);
        let mut data = String::new();

        for (number, task) in self.todo_list.iter().enumerate() {
            // Ensure the task string has at least 5 characters (e.g., [ ] + task content)
            if task.len() > 4 {
                // let symbol = &task[..4];
                let task = &task[4..];
                data = format!("{} {}\n", number, task);
            }
            writer
                .write_all(data.as_bytes())
                .expect("Failed to write to stdout.")
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("'todo add' takes at least one argument.");
            process::exit(1);
        }

        let todo_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .expect("Couldn't open the to-do file.");

        let mut writer = BufWriter::new(todo_file);

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            let line = format!("[ ] {}\n", arg.trim());
            writer.write_all(line.as_bytes()).expect("Failed to write data.");
        }

    }
}

// #[derive(Debug)]
// struct TodoItem {
//     id: u64,
//     title: String,
//     completed: bool,
// }

// struct TodoList {
//     items: Vec<TodoItem>,
// }
//
// impl TodoList {
//     fn new() -> Self {
//         Self { items: Vec::new() }
//     }
//
//     fn add_item(&mut self, title: String) {
//         let id = self.items.len() as u64 + 1;
//         let new_item = TodoItem {
//             id,
//             title: title.clone(),
//             completed: false,
//         };
//         self.items.push(new_item);
//         println!("New item added: {}", title);
//     }
//
//     fn list_items(&self) {
//         if self.items.is_empty() {
//             println!("Your to-do list is empty.");
//         } else {
//             println!("Your to-do list:");
//             for item in &self.items {
//                 let checkbox = if item.completed { "[x]" } else { "[ ]" };
//                 println!("{} {} ({})", checkbox, item.title, item.id);
//             }
//         }
//     }
//
//     fn complete_item(&mut self, id: u64) {
//         if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
//             item.completed = true;
//             println!("Completed: {}", item.title);
//         } else {
//             println!("No item with ID {} found.", id);
//         }
//     }
// }
//
// fn old_loop() {
//     let mut todo_list = TodoList::new();
//
//     loop {
//         println!("1. Add Item");
//         println!("2. List Items");
//         println!("3. Complete Item");
//         println!("4. Exit");
//
//         // String to store user input
//         let mut choice = String::new();
//         // Read user input until a newline character is encountered, append to String 'choice'
//         io::stdin()
//             .read_line(&mut choice)
//             .expect("Failed to read line");
//         // Parse input string to an u32
//         let choice: u32 = match choice.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//
//         match choice {
//             1 => {
//                 println!("Enter the title of the new item:");
//                 let mut title = String::new();
//                 io::stdin()
//                     .read_line(&mut title)
//                     .expect("Failed to read line");
//                 todo_list.add_item(title.trim().to_string());
//             }
//             2 => {
//                 todo_list.list_items();
//             }
//             3 => {
//                 println!("Enter the ID of the completed item:");
//                 let mut id = String::new();
//                 io::stdin().read_line(&mut id).expect("Failed to read line");
//                 let id: u64 = match id.trim().parse() {
//                     Ok(num) => num,
//                     Err(_) => continue,
//                 };
//                 todo_list.complete_item(id);
//             }
//             4 => {
//                 println!("Exiting the program.");
//                 break;
//             }
//             _ => {
//                 println!("Invalid choice. Please enter a number between 1 and 4.");
//             }
//         }
//     }
// }

fn main() {
    let todo_app = TodoApp::new().expect("Couldn't create the to-do app instance.");
    // TODO use clap
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "list" => todo_app.list(),
            "add" => todo_app.add(&args[2..]),
            _ => {}
        }
    }
}
