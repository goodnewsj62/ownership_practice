use std::collections::HashMap;

fn main() {
    let text =
        "you will be astonished of how he handled the situation. However, my baby is the best";
    println!("{:?}", word_finder(text));
}

fn word_finder(text: &str) -> HashMap<String, usize> {
    let mut word_bank: HashMap<String, usize> = HashMap::new();
    let word_bytes = text.as_bytes();
    let mut last_index: usize = 0;

    for (i, &value) in word_bytes.iter().enumerate() {
        if value == b' ' {
            let word = &text[last_index..i];
            let count = increment_count(&word_bank, strip_punctuations(word));
            word_bank.insert(strip_punctuations(word).to_owned(), count);
            last_index = i + 1;
        }
    }

    let last_word = &text[last_index..];
    let count = increment_count(&word_bank, strip_punctuations(last_word));
    word_bank.insert(strip_punctuations(last_word).to_owned(), count);

    word_bank
}

fn strip_punctuations(word: &str) -> &str {
    let last_char = word.chars().last();

    match last_char {
        Some('.') | Some(',') | Some('?') | Some('!') => &word[0..word.len() - 1],

        Some(_) | None => word,
    }
}

fn increment_count(store: &HashMap<String, usize>, word: &str) -> usize {
    if let Some(&value) = store.get(word) {
        value + 1
    } else {
        1
    }
}

// ,?!.

/*
    words are separated by spaces
    we would not want punctuations to be in our word
step 1: strip out the punctuations
step 2: write a word finder
step3: each word found should be stored and counted
step5: display the words and their frequency in the most trivial way

to strip out punctuations the most straight forward way is to go through every character and filter by
if the character is a valid string or a punctuation (only push to our string if it is not a punctuation)
O(N)

another way  to do this is to check the last character after a word: This is where punctuations generally live
before a space and then we try to ignore it and only store the valid words

2
we must first trim the string of leading or lagging white spaces

The word finder will move through each character (assuming ASCII) till we find a space
sore the word from index stored to current index - 1 (before the space)  then we will use function
store to store and count the word
O(n)

the last operation will be to get the remaining word after the last space found

3
To store we will have a hashmap where we will store the value ig the hashmap does'nt have the value already
or increment by one if it already does have the value
*/
