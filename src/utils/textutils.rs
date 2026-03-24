//! Utilitaires de traitement de texte pour GSP
//!
//! Fournit des fonctions pour le nettoyage et le formatage du texte.

/// Parse les hashtags en séparant les mots CamelCase
///
/// # Exemples
/// ```
/// let text = String::from("#helloWorld");
/// let result = parse_hashtag(&text);
/// assert_eq!(result, "#hello World");
/// ```
pub fn parse_hashtag(string: &str) -> String {
    let mut result = String::new();

    string.split_whitespace().for_each(|w| {
        if w.starts_with('#') {
            for c in w.chars() {
                if c.is_uppercase() {
                    result.push(' ');
                }

                result.push(c);
            }
        } else {
            result.push_str(w);
        }

        result.push(' ');
    });

    trim_whitespace(&result)
}

/// Supprime les espaces multiples dans une chaîne
/// https://stackoverflow.com/a/71864249
/// # Exemples
/// ```
/// let text = String::from("  a  b  c  ");
/// let result = trim_whitespace(&text);
/// assert_eq!(result, "a b c");
/// ```
pub fn trim_whitespace(string: &str) -> String {
    let mut result = String::with_capacity(string.len());

    string.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });

    result
}

/// Supprime les caractères spéciaux et normalise le texte
///
/// # Exemples
/// ```
/// let text = String::from("𝐠𝐫𝐚𝐬 et 𝘪𝘵𝘢𝘭𝘪𝘤");
/// let result = remove_special_characters(&text);
/// assert_eq!(result, "gras et italic");
/// ```
pub fn remove_special_characters(string: &str) -> String {
    let mut list_of_special_characters = Vec::new();

    list_of_special_characters.append(&mut get_list_remove_quotes());
    list_of_special_characters.append(&mut get_list_replace_char());

    // Alphabet - caractères spéciaux vers alphabet normal
    for j in [
        '𝐀', '𝐚', '𝑨', '𝒂', '𝒜', '𝒶', '𝔸', '𝕒', '𝕬', '𝖆', '𝖠', '𝖺', '𝗔', '𝗮', '𝘈', '𝘢', '𝘼', '𝙖',
        '𝙰', '𝚊', '𝐴', '𝑎', '𝔄', '𝔞', '𝓐', '𝓪',
    ] {
        for i in 0..26 {
            if let (Some(from), Some(to)) = (
                std::char::from_u32(j as u32 + i),
                std::char::from_u32('a' as u32 + i),
            ) {
                list_of_special_characters.push([from, to]);
            }
        }
    }

    // Chiffres - caractères spéciaux vers chiffres normaux
    for j in ['𝟎', '𝟘', '𝟢', '𝟬', '𝟶', '𝟬', '𝟢'] {
        for i in 0..10 {
            if let (Some(from), Some(to)) = (
                std::char::from_u32(j as u32 + i),
                std::char::from_u32('0' as u32 + i),
            ) {
                list_of_special_characters.push([from, to]);
            }
        }
    }

    let mut text = string.to_string();
    for i in list_of_special_characters {
        text = text.replace(i[0], &i[1].to_string());
    }

    text
}

/// supprimer les caractères markdown
///
/// # Examples
/// ```
/// let text = String::from("**Hello World**");
/// let result = read_vars(text);
/// assert_eq!(result, " Hello World");
/// ```
pub fn remove_markdown(string: &str) -> String {
    let mut text = string.to_string();

    text = text.replace("**", ""); // bold
    text = text.replace("*", ""); // italic

    text
}

/// Lit et formatte les variables (snake_case, kebab-case, CamelCase)
///
/// # Exemples
/// ```
/// let text = String::from("HelloWorld");
/// let result = read_vars(&text);
/// assert_eq!(result, " Hello World");
/// ```
pub fn read_vars(string: &str) -> String {
    let mut text = string.to_string();

    // Remplace les _,- par des espaces (Snake_case et kebab-case)
    for i in ['_', '-'] {
        text = text.replace(i, " ");
    }

    // Convertit le CamelCase en texte normal
    text = parse_camel_case(&text);

    // Supprimer les caractères markdown
    text = remove_markdown(&text);

    text
}

/// Liste des caractères de quotes à remplacer par des espaces
fn get_list_remove_quotes() -> Vec<[char; 2]> {
    ['"', '\'', '`', '[', ']', '{', '}']
        .iter()
        .map(|x| [*x, ' '])
        .collect()
}

/// Liste de caractères spéciaux à remplacer
fn get_list_replace_char() -> Vec<[char; 2]> {
    [
        ('ᴀ', 'a'),
        ('ʙ', 'b'),
        ('ᴄ', 'c'),
        ('ᴅ', 'd'),
        ('ᴇ', 'e'),
        ('ɢ', 'g'),
        ('ʜ', 'h'),
        ('ɪ', 'i'),
        ('ᴊ', 'j'),
        ('ᴋ', 'k'),
        ('ʟ', 'l'),
        ('ᴍ', 'm'),
        ('ɴ', 'n'),
        ('ᴏ', 'o'),
        ('ᴘ', 'p'),
        ('ʀ', 'r'),
        ('ᴛ', 't'),
        ('ᴜ', 'u'),
        ('ᴠ', 'v'),
        ('ᴡ', 'w'),
        ('ʏ', 'y'),
        ('ᴢ', 'z'),
        ('а', 'a'),
        ('А', 'A'),
        ('в', 'b'),
        ('В', 'b'),
        ('г', 'r'),
        ('ғ', 'f'),
        ('е', 'e'),
        ('Е', 'E'),
        ('ѕ', 's'),
        ('Ѕ', 'S'),
        ('и', 'n'),
        ('і', 'i'),
        ('І', 'I'),
        ('к', 'k'),
        ('К', 'k'),
        ('л', 'n'),
        ('м', 'm'),
        ('М', 'm'),
        ('н', 'h'),
        ('Н', 'H'),
        ('о', 'o'),
        ('О', 'O'),
        ('р', 'p'),
        ('Р', 'p'),
        ('с', 'c'),
        ('С', 'C'),
        ('т', 't'),
        ('Т', 'T'),
        ('у', 'y'),
        ('х', 'x'),
    ]
    .iter()
    .map(|x| [x.0, x.1])
    .collect()
}

/// Parse le CamelCase en séparant les mots
fn parse_camel_case(text: &str) -> String {
    let mut result = String::new();

    let mut is_uppercase = false;
    for c in text.chars() {
        if c.is_uppercase() {
            if !is_uppercase {
                result.push(' ');
            }
            is_uppercase = true;
        } else {
            is_uppercase = false;
        }

        result.push(c);
    }

    result
}

/// Remplace les termes selon le dictionnaire
pub fn replace(text: &str) -> String {
    // Si le répertoire existe
    if std::path::Path::new("dict/fr_FR").exists() {
        let mut text = text.to_string();

        for (replace, by) in get_list_replace() {
            text = text.replace(&replace, &by);
        }

        return text;
    }

    text.to_string()
}

/// Charge la liste des remplacements depuis le dictionnaire
fn get_list_replace() -> Vec<(String, String)> {
    let mut result = Vec::new();

    let path = match list_files("dict/fr_FR") {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Avertissement: impossible de lire le dictionnaire: {}", e);
            return result;
        }
    };

    for file in path {
        if let Ok(metadata) = std::fs::metadata(&file)
            && metadata.is_file()
            && let Ok(content) = std::fs::read_to_string(&file)
        {
            for line in content.split('\n') {
                let parts: Vec<&str> = line.split('=').collect();

                if parts.len() != 2 {
                    continue;
                }
                result.push((parts[0].to_string(), parts[1].to_string()));
            }
        }
    }

    result
}

/// Liste tous les fichiers d'un répertoire
fn list_files(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();

    let paths = std::fs::read_dir(path)?;

    for path in paths {
        let path = path?.path();
        if let Some(path_str) = path.to_str() {
            result.push(path_str.to_string());
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hashtag() {
        let text = String::from("#helloWorld");
        let result = parse_hashtag(&text);
        assert_eq!(result.as_str(), "#hello World");
    }

    #[test]
    fn test_parse_hashtag2() {
        let text = String::from("le #chat est #mignon");
        let result = parse_hashtag(&text);
        assert_eq!(result.as_str(), "le #chat est #mignon");
    }

    #[test]
    fn test_trim_whitespace() {
        let text = String::from("  a  b  c  ");
        let result = trim_whitespace(&text);
        assert_eq!(result.as_str(), "a b c");
    }

    #[test]
    fn test_remove_special_characters() {
        let text = String::from("𝐠𝐫𝐚𝐬 et 𝘪𝘵𝘢𝘭𝘪𝘤");
        let result = remove_special_characters(&text);
        assert_eq!(result.as_str(), "gras et italic");
    }

    #[test]
    fn test_replace_special_chars() {
        let text = String::from("С і І ѕ");
        let result = remove_special_characters(&text);
        assert_eq!(result, "C i I s");
    }

    #[test]
    fn test_read_vars() {
        let text = String::from("HelloWorld");
        let result = read_vars(&text);
        assert_eq!(result.as_str(), " Hello World");
    }

    #[test]
    fn test_replace_no_dict() {
        // Test when dict directory doesn't exist
        let text = replace("HelloWorld");
        assert_eq!(text, "HelloWorld");
    }
}
