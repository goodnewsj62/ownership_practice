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

        to make the display format better just like i see in other linux display command like top
        for each column we get the maximum length of character and when displaying if the character is < than
        the max we will pad with space
    */

    let (title, desc, status) = ("title", "description", "status");

    let id_col_size = todos.len().to_string().len();
    let [col_1, col_2, col_3] = get_max_length(todos);
    let [col_1, col_2, col_3] = [
        col_1.max(title.len()),
        col_2.max(desc.len()),
        col_3.max(status.len()),
    ];

    println!(
        "{} {} {} {}",
        p_r("id".to_uppercase().as_ref(), id_col_size),
        p_r(title.to_uppercase().as_ref(), col_1),
        p_r(desc.to_uppercase().as_ref(), col_2),
        p_r(status.to_uppercase().as_ref(), col_3)
    );

    for (index, todo) in todos.iter().enumerate() {
        let description = if let Some(value) = &todo.description {
            value
        } else {
            ""
        };
        let status = format!("{:?}", todo.status);

        println!(
            "{} {} {} {}",
            p_r(index.to_string().as_ref(), id_col_size),
            p_r(todo.title.as_ref(), col_1),
            p_r(description, col_2),
            p_r(status.as_ref(), col_3)
        );
    }
}

fn get_max_length(todos: &[Todo]) -> [usize; 3] {
    let mut column_len: [usize; 3] = [0, 0, 8];

    todos.iter().for_each(|f| {
        let title_len = f.title.len();
        let description_len = f.description.as_ref().map_or(0, |d| d.len());

        column_len[0] = if title_len > column_len[0] {
            title_len
        } else {
            column_len[0]
        };

        column_len[1] = if description_len > column_len[1] {
            description_len
        } else {
            column_len[1]
        };
    });

    column_len
}

fn p_r(value: &str, width: usize) -> String {
    format!("{:width$}", value, width = width + 1)
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
