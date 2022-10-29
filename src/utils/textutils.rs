// remove multiple spaces in a string
// https://stackoverflow.com/a/71864249
pub fn trim_whitespace(s: &str) -> String {
    // second attempt: only allocate a string
    let mut result = String::with_capacity(s.len());
    s.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}

// remove quotes ", ' and ` from a string
pub fn remove_quotes(text: &str) -> String {
    let mut text = text.to_string();
    for quote in ['"', '\'', '`'] {
        text = text.replace(quote, "");
    }
    text
}

pub fn remove_special_characters(text: &str) -> String {
    let mut result = String::new();
    let mut list_of_special_characters = Vec::new();

    for j in ['𝐀', '𝐚', '𝐴', '𝑎', '𝑨', '𝒂', '𝒜', '𝒶', '𝘢'] {
        for i in 0..26 {
            list_of_special_characters.push((j as u32 + i, (97 + i) as u8 as char));
        }
    }

    for c in text.chars() {
        let mut found = false;

        for (special_character, character) in list_of_special_characters.iter() {
            if c as u32 == *special_character {
                result.push(*character);
                found = true;
                break;
            }
        }

        if !found {
            result.push(c);
        }
    }

    result
}

// Lister tous les fichiers d'un répertoire
pub fn list_files(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    let paths = std::fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        result.push(path.to_string());
    }

    result
}

fn get_list_replace() -> Vec<(String, String)> {
    let mut result = Vec::new();

    let path = list_files("dict/fr_FR");

    for file in path {
        if std::fs::metadata(&file).unwrap().is_file() {
            let file = std::fs::read_to_string(file).unwrap();
            let file = file.split('\n').collect::<Vec<&str>>();
            for line in file {
                let line = line.split('=').collect::<Vec<&str>>();

                if line.len() != 2 {
                    continue;
                }
                result.push((line[0].to_string(), line[1].to_string()));
            }
        }
    }

    result
}

pub fn replace(text: &str) -> String {
    // Si le répertoire existe
    if std::path::Path::new("dict/fr_FR").exists() {
        let mut text = text.to_string();

        for (replace, by) in get_list_replace() {
            text = text.replace(&replace, &by);
        }
    }

    text.to_string()
}

pub fn text_to_dict(text: &str) -> String {
    let mut text = text.to_string().to_lowercase();
    text = text.trim().to_string();
    text = trim_whitespace(&text);
    text = remove_quotes(&text);
    text = remove_special_characters(&text);

    text
}
