use std::io::{BufRead, BufReader, Write};
use std::{fs::File, io::BufWriter};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)] // todo args to store your fav todo
struct TodoArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// add and save todo
    Add { item: String },

    /// list all todos
    List,

    /// remove id
    Remove {
        /// id of the todo item
        #[arg(long, short)]
        id: u32,
    },

    /// mark as done
    Done {
        /// id of the todo item
        #[arg(long, short)]
        id: u32,
    },

    /// mark task as undone
    Undone {
        /// id of the todo item
        #[arg(long, short)]
        id: u32,
    },

    /// matk archive a task
    Archive {
        /// id of the todo item
        #[arg(long, short)]
        id: u32,
    },
}

struct Todo {
    title: String,
    description: Option<String>,
    status: Status,
}

#[derive(Debug)]
enum Status {
    Archived,
    Active,
    Done,
}

impl Todo {
    fn from_values(title: String, description: Option<String>, status: Status) -> Self {
        Todo {
            title,
            description,
            status,
        }
    }
}

fn main() {
    let mut store: Vec<Todo> = read_from_file();
    let args = TodoArgs::parse();

    match &args.command {
        Commands::Add { item } => add_todo(&mut store, item),
        Commands::List => display_todos(&store),
        Commands::Done { id } => mark_as_done(&mut store, *id),
        Commands::Archive { id } => archived(&mut store, *id),
        Commands::Remove { id } => remove_todo(&mut store, *id),
        Commands::Undone { id } => mark_as_undone(&mut store, *id),
    }

    sync_to_file(&store);
}

fn display_todos(todos: &[Todo]) {
    /*
        [title | description | status]
        1.  name ,  description,  status
        2. name,  description, status

    */
    let header = ["id", "title", "description", "status"];

    println!("{:?}", header);

    for (index, todo) in todos.iter().enumerate() {
        let description = if let Some(value) = &todo.description {
            value
        } else {
            ""
        };

        println!("{index}. {} {} {:?}", todo.title, description, todo.status)
    }
}

fn add_todo(store: &mut Vec<Todo>, title: &str) {
    let data = Todo {
        title: title.to_owned(),
        description: None,
        status: Status::Active,
    };
    store.push(data);

    println!("todo added id {}", store.len() - 1)
}

fn remove_todo(store: &mut Vec<Todo>, id: u32) {
    if id as usize >= store.len() {
        panic!("Invalid Id")
    }

    store.remove(id as usize);

    println!("todo removed")
}

fn mark_as_done(store: &mut [Todo], id: u32) {
    if id as usize >= store.len() {
        panic!("Invalid Id")
    }

    if let Some(todo) = store.get_mut(id as usize) {
        todo.status = Status::Done;
    }
}

fn mark_as_undone(store: &mut [Todo], id: u32) {
    if id as usize >= store.len() {
        panic!("Invalid Id")
    }

    if let Some(todo) = store.get_mut(id as usize) {
        todo.status = Status::Active;
    }
}

fn archived(store: &mut [Todo], id: u32) {
    if id as usize >= store.len() {
        panic!("Invalid Id")
    }

    if let Some(todo) = store.get_mut(id as usize) {
        todo.status = Status::Archived;
    }
}

fn sync_to_file(store: &[Todo]) {
    let file = File::create("todo.csv").unwrap();
    let mut buf = BufWriter::new(file);

    for (index, value) in store.iter().enumerate() {
        let description = if let Some(v) = &value.description {
            v.to_owned()
        } else {
            "".to_owned()
        };

        let resp = format!("{index}|{}|{}|{:?}", value.title, description, value.status)
            .trim()
            .to_owned();

        writeln!(buf, "{resp}").unwrap();
    }
}

fn read_from_file() -> Vec<Todo> {
    let mut store: Vec<(Todo, u32)> = Vec::new();
    let file_result = File::open("todo.csv");

    let Ok(file) = file_result else {
        return Vec::new();
    };

    let buf = BufReader::new(file);
    for line in buf.lines() {
        if let Ok(value) = line {
            let values: Vec<&str> = value.split("|").collect();
            // ---/ /----

            match values.as_slice() {
                [id, title, description, status] => {
                    let id: u32 = id.parse().unwrap();
                    let description = if description.is_empty() {
                        None
                    } else {
                        Some((*description).to_string())
                    };
                    let title = (*title).to_owned();

                    let status = string_to_status(status);

                    store.push((Todo::from_values(title, description, status), id));
                }
                _ => {
                    panic!("Tempered or invalid file detected");
                }
            }
        } else {
            panic!("an error occurred");
        }
    }

    store.sort_by(|a, b| a.1.cmp(&b.1));

    store.into_iter().map(|a| a.0).collect()
}

fn string_to_status(status: &str) -> Status {
    match status {
        "Active" => Status::Active,
        "Archived" => Status::Archived,
        "Done" => Status::Done,
        _ => panic!("tampered file detected. Invalid status variant"),
    }
}

/*
    Simple CLI Todo Assignment

    Goal:
        Build a small command-line todo app in Rust.
        Keep it simple and use it to practice core Rust basics.

    What the app should do:
        Add a todo item
        List todo items in a table-like view
        Remove a todo item by id
        Mark a todo as done
        Mark a todo as undone
        Archive a todo item

    Suggested CLI commands:
        add
        list
        remove
        done
        undone
        archive

    What to show in the list:
        id
        todo text
        status marker (done or undone)
        archived marker if you decide to display archived items

    Main learning focus:
        clap for CLI argument parsing
        Vec for storing todo items
        struct for modeling a todo item
        String for todo text
        enum for todo status

    Good Rust topics to scan before building:
        ownership and borrowing [+]
        mutable references [+]
        Vec []
        struct [+]
        enum [+]
        String and &str  [+]
        impl blocks and methods [+]
        match [+]
        Option and Result [+]
        iterators []
        clap derive macros [+]
        basic formatting and printing [+]

    Keep the project small:
        Do not over-engineer it.
        The goal is practice, not building a full task manager.
*/
