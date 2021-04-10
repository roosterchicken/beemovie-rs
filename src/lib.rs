mod stuff;
mod script;
pub use script::script;
pub fn sentence(num: i32) -> String {
    let script = script();
    let mut split: Vec<&str> = script.split('.').collect();
    split.retain(|&str| str != ".");
    split.retain(|&str| str != " .");
    split.retain(|&str| str != " . ");
    split.retain(|&str| str != ". ");
    split.retain(|&str| !str.is_empty());
    split.retain(|&str| str != " ");
    let mut vector = Vec::<String>::new();
    for item in &split {
        vector.push(item.to_string());
    }
    stuff::multiply_sentences(num, vector)
}

pub fn word(num: i32) -> String {
    let mut script = script();
    script = script.replace(&['.', '?', '!', '?', ','][..], "");
    let mut split: Vec<&str> = script.split(' ').collect();
    split.retain(|&str| !str.is_empty());
    let mut vector = Vec::<String>::new();
    for item in &split {
        vector.push(item.to_string());
    }
    stuff::multiply_words(num, vector)
}

pub fn paragraph(num: i32) -> String {
    let script = script();
    let mut split: Vec<&str> = script.split('.').collect();
    split.retain(|&str| str != ".");
    split.retain(|&str| str != " .");
    split.retain(|&str| str != " . ");
    split.retain(|&str| str != ". ");
    split.retain(|&str| !str.is_empty());
    split.retain(|&str| str != " ");
    let mut vector = Vec::<String>::new();
    for item in &split {
        vector.push(item.to_string());
    }
    stuff::paragraph(num, vector)
}

pub fn version() -> String {
    "0.2.3".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        sentence(1);
        script();
        word(1);
        paragraph(1);
    }
}
