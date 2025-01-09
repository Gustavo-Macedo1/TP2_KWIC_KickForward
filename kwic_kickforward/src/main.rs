use std::collections::HashSet;
use std::fs;
use std::io;

// Criando apelidos (aliases) para as funções no estilo KickForward
type CircularShiftsFn = fn(Vec<(String, String)>, OrderShiftsFn);
type OrderShiftsFn = fn(Vec<(String, String)>, NoOpFn);
type NoOpFn = fn(Vec<(String, String)>, EmptyFn);
type EmptyFn = fn(Vec<(String, String)>);

// Função para ler o arquivo dado um caminho
pub fn read_input<F>(
    file_path: String,
    next_function: F,
) -> Result<(), io::Error>
where
    F: FnOnce(Vec<String>, HashSet<String>, CircularShiftsFn),
{
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();
    
    // Criando um conjunto de stop words (você pode adicionar mais palavras conforme necessário)
    let stop_words: HashSet<String> = vec!["a", "the", "is", "on", "at", "sat"]
        .into_iter()
        .map(String::from)
        .collect();
    
    next_function(lines, stop_words, generate_circular_shifts);
    Ok(())
}

// Função que, para cada título, gera as keywords
pub fn split_into_keywords(
    lines: Vec<String>,
    stop_words: HashSet<String>,
    next_function: CircularShiftsFn,
) {
    let mut keyword_pairs = Vec::new();

    for line in &lines {
        for word in line.split_whitespace() {
            let word_cleaned = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();

            if !stop_words.contains(&word_cleaned) && !word_cleaned.is_empty() {
                keyword_pairs.push((word_cleaned.clone(), line.clone()));
            }
        }
    }

    next_function(keyword_pairs, order_shifted_pairs);
}

pub fn generate_circular_shifts(
    pairs: Vec<(String, String)>,
    next_function: OrderShiftsFn,
) {
    let mut shifted_pairs = Vec::new();

    for (keyword, title) in pairs {
        let mut words: Vec<String> = title.split_whitespace().map(String::from).collect();

        while !words.is_empty() && words[0].to_lowercase() != keyword {
            let first = words.remove(0);
            words.push(first);
        }

        shifted_pairs.push((words.join(" "), title.clone()));
    }

    next_function(shifted_pairs, no_op);
}

pub fn order_shifted_pairs(
    shifted_pairs: Vec<(String, String)>,
    next_function: NoOpFn,
) {
    let mut ordered_pairs = shifted_pairs;
    ordered_pairs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    next_function(ordered_pairs, |_data| {});
}

pub fn no_op(data: Vec<(String, String)>, _func: EmptyFn) {
    for (shifted, title) in data {
        println!("{} : {}", shifted, title);
    }
}

fn main() {
    let file_path = r"./docs/texto.txt".to_string();
    if let Err(e) = read_input(file_path, split_into_keywords) {
        eprintln!("Erro ao ler o arquivo: {}", e);
    }
}