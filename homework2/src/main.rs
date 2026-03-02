// Victor Chairez 
// Sunday March 01, 2026


// ## Assignment 2: Word Frequency Counter

// ### Problem Statement

// Create a program that:
// 1. Takes a string of text as input
// 2. Splits the text into words (space as separator) // text.split_whitespace().collect();
// 3. Counts the frequency of each word
// 4. Returns the word with the highest frequency and its count


fn most_frequent_word(text: &str) -> (String, usize) // borrow without ownesrihp and return a tuple 
{
    let words: Vec<&str> = text.split_whitespace().collect(); // in a sentence, this divided on each space of that sentence. then collects into a list

    let mut max_word = String::new(); // max_word stores the highest total world
    let mut max_count = 0; // nothing at start and also mut as it CAN change 
    let mut i = 0; // so it will go through each word
    while i < words.len() // ^^^ also i had forgotten how to do this ... but i figured it out i think 
    { // inside this while above, we have nested conditionsals 
        let mut count = 0; // so if this new word matches the previous, it counts it and adds to the list 
        let mut j = 0; 
        while j < words.len() // this condition was a bit challenging for a second 
        {
            if words[i] == words[j] 
            {
                count += 1; 
            }
            j += 1; 
        }
        if count > max_count // now here if we had a max_count but there is a better one that could replace it..
        {
            max_count = count; // max_count becomes that better version 
            max_word = words[i].to_string(); // now its owned (it was &str before)
        }
        i += 1; // update ! 
    }
    (max_word, max_count) // so now we eturn the word that had the highest and the count 
}


fn main() 
{
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox"; 
    let (word, count) = most_frequent_word(text); 
    println!("Most fequent word: \"{}\" ({} times)", word, count); 
}
