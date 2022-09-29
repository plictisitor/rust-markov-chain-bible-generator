// SORRY FOR UNNECESSARY COMMENTS, I USED GITHUB COPILOT TO WRITE THIS CODE
// I mead I only wrote the comments, the code was written by AI... I know I'm lazy...
// This is my first rust program ever... I'm pretty sure it's trash
// but the language is pretty :)

use std::collections::HashMap;
use std::fs;
// use rand::Rng;
use rand::seq::SliceRandom;

const FILENAME: &str = "bible.txt";
const GROUP_SIZE: usize = 5;
const TEXT_TO_CONTINUE: &str = "The meaning of life is ";

fn generate(text: String, map: &mut HashMap<String, Vec<String>>) -> String {
    // get the last three letters
    let last_n = &text[text.len() - GROUP_SIZE..text.len()];
    // choose a random from the hasmap at key last_n
    let next_group = map.get(last_n).unwrap();
    // choose a random from the vector
    let next = next_group.choose(&mut rand::thread_rng()).expect("bruh").to_string();
    // return the next
    return next.to_string();
}

fn main() {
    // define hasmap
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    // read text from bible.txt
    let mut text = fs::read_to_string(FILENAME).expect("Unable to read file");
    // lowercase all text
    text = text.to_lowercase();
    // remove everything except letters and spaces
    text = text.replace(|c: char| !c.is_alphabetic() && c != ' ', "");

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    // for every three chars in text
    for i in 0..text.len() - GROUP_SIZE {
        // add to inputs
        inputs.push(text[i..i + GROUP_SIZE].to_string());
        // add to outputs
        outputs.push(text[i + GROUP_SIZE..i + GROUP_SIZE + 1].to_string());
        // add to map at key inputs[i] with value outputs[i]
        map.entry(inputs[i].to_string())
            .or_insert(Vec::new())
            .push(outputs[i].to_string());
        if i % 80000 == 0 {
            // clear old progress
            print!("\x1B[2J\x1B[1;1Hgenerating... ");
            // print progress
            println!(
                "{}%, len: {}, uniqueness: {}",
                i as f32 / text.len() as f32 * 100.0,
                map.len(),
                map.len() as f32 / text.len() as f32 * 100.0
            );
        }
    }

    // clear progress
    print!("\x1B[2J\x1B[1;1H");
    // print text
    let mut output = String::from(TEXT_TO_CONTINUE);
    println!("{}", output);
    // generate text
    for _ in 0..4444 {
        // clear
        print!("\x1B[2J\x1B[1;1H");
        // add & print
        output.push_str(&generate(output.clone(), &mut map).to_string());
        println!("{}", output);
        // sleep for 0.1 seconds
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
    // add a dot. i love dots. they cute XD
    output.push_str(".");
    // clear and print
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", output);
    print!("\n");
}

// happy interpreting the output :D