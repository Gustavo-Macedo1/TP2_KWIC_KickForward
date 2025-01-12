use std::collections::HashSet;
use std::fs;
use std::io;

// Criando apelidos (aliases) para as funções no estilo KickForward
type CircularShiftsFn = fn(Vec<(String, String)>, OrderShiftsFn);
type OrderShiftsFn = fn(Vec<(String, String)>, FmtOutFn);
type FmtOutFn = fn(Vec<(String, String)>, WrOutFn);
type WrOutFn = fn(Vec<(String, String)>, NoOpFn);
type NoOpFn = fn(EmptyFn);
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

// Função que, para cada título, gera tuplas de pares chave-valor, sendo a chave uma keyword e o valor um título
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

// Permuta os títulos até que as keywords estejam no início da string
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

    next_function(shifted_pairs, format_output);
}

// Ordena os pares levando em consideração (e comparando) apenas o primeiro item da tupla
pub fn order_shifted_pairs(
    shifted_pairs: Vec<(String, String)>,
    next_function: FmtOutFn,
) {
    let mut ordered_pairs = shifted_pairs;

    ordered_pairs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    next_function(ordered_pairs, write_output);
}

// Formata os títulos para prepará-los para a saída na tela
pub fn format_output(
    ordered_pairs: Vec<(String, String)>,
    next_function:WrOutFn,
) {
    let mut formatted_output = Vec::new();

    for (kwic, title) in ordered_pairs {
        let mut formatted_title = String::from("(from: ");
        formatted_title = formatted_title + &title + ")";
        formatted_output.push((kwic.clone(), formatted_title));
    }

    next_function(formatted_output, no_op);
}

// Imprime na tela os pares ordenados e formatados
pub fn write_output(
    formatted_output: Vec<(String, String)>,
    next_function: NoOpFn,
) {
    for (kwic, title) in formatted_output {
        println!("{} : {}", kwic, title);
    }

    next_function(|_data| {});
}

// Última função do pipeline, encerra a cascata de funções
pub fn no_op(_func: EmptyFn) {
    return;
}

// Main usa o caminho de um arquivo para chamar read_input e iniciar a cascata de funções.
// Caso a leitura não seja bem sucedida, retorna erro
fn main() {
    let file_path = r"./docs/texto.txt".to_string();
    if let Err(e) = read_input(file_path, split_into_keywords) {
        eprintln!("Erro ao ler o arquivo: {}", e);
    }
}