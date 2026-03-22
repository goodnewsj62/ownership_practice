use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, prelude::*},
};
fn main() {
    let env_args: Vec<String> = env::args().collect();
    let (word, path) = parse_env_args(&env_args);

    let found_lines = extract_lines(path, word);

    display_lines(&found_lines);
}
/*
    given a word and a text file find  lines that has that word and print them out to the console
    use the cli to process this


    the goal
    - use only rust libraries/crates for now
    then you can go ahead and try clap and other libraries

    topics
        - file
        - data structure
        - rust cli

    approach
    iteratively read file  line by line
    divide up words by spaces
    strip punctuations as those are not words
    go through the words and match word
    if there is a match print the line
*/

fn parse_env_args(args: &[String]) -> (&str, &str) {
    (&args[1], &args[2])
}

fn extract_lines(path: &str, word: &str) -> HashMap<usize, String> {
    let mut found_lines = HashMap::new();
    let file = File::open(path).expect("expected a file ");
    let buffer = io::BufReader::new(file);

    for (i, line) in buffer.lines().filter_map(Result::ok).enumerate() {
        // value here
        let found = line.contains(word);
        if found {
            found_lines.insert(i, line);
        }
    }

    found_lines
}

fn display_lines(store: &HashMap<usize, String>) {
    for (k, v) in store.iter() {
        println!("{k}// {}", v);
    }
}

// next steps
/*
    use the crate clap to parse cli argument
    take a look at sting and what can be done with String
    scan through the cli example in the book
*/
