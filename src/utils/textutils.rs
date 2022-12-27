/// remove multiple spaces in a string
/// https://stackoverflow.com/a/71864249
/// # Examples
/// ```
/// use utils::textutils::trim_whitespace;
/// let s = "  a  b  c  ";
/// let s = trim_whitespace(s);
/// assert_eq!(s, "a b c");
/// ```
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

pub fn get_list_remove_quotes() -> Vec<[char; 2]> {
    return ['"', '\'', '`', '[', ']', '{', '}']
        .iter()
        .map(|x| [*x, ' '])
        .collect();
}

/// list de caractères spéciaux à remplacer
pub fn get_list_replace_char() -> Vec<[char; 2]> {
    return [
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
        ('в', 'b'),
        ('В', 'b'),
        ('г', 'r'),
        ('ғ', 'f'),
        ('е', 'e'),
        ('и', 'n'),
        ('к', 'k'),
        ('К', 'k'),
        ('л', 'n'),
        ('м', 'm'),
        ('М', 'm'),
        ('н', 'h'),
        ('Н', 'H'),
        ('о', 'o'),
        ('р', 'p'),
        ('Р', 'p'),
        ('с', 'c'),
        ('т', 't'),
        ('Т', 'T'),
        ('у', 'y'),
        ('х', 'x'),
    ]
    .iter()
    .map(|x| [x.0, x.1])
    .collect();
}

/// remove special characters
/// # Examples
/// ```
/// use utils::textutils::remove_special_characters;
/// let text = "𝐠𝐫𝐚𝐬 et 𝘪𝘵𝘢𝘭𝘪𝘤";
/// let text = remove_special_characters(text);
/// assert_eq!(text, "gras et italic");
/// ```
pub fn remove_special_characters(text: &str) -> String {
    let mut list_of_special_characters = Vec::new();

    list_of_special_characters.append(&mut get_list_remove_quotes());

    list_of_special_characters.append(&mut get_list_replace_char());

    // Alphabet
    for j in [
        '𝐀', '𝐚',
        '𝑨', '𝒂',
        '𝒜', '𝒶',
        '𝔸', '𝕒',
        '𝕬', '𝖆',
        '𝖠', '𝖺',
        '𝗔', '𝗮',
        '𝘈', '𝘢',
        '𝘼', '𝙖',
        '𝙰', '𝚊',
        '𝐴', '𝑎',
        '𝔄', '𝔞',
        '𝓐','𝓪',
    ] {
        for i in 0..26 {
            list_of_special_characters.push([
                std::char::from_u32(j as u32 + i).unwrap(),
                std::char::from_u32('a' as u32 + i).unwrap(),
            ]);
        }
    }

    // Chiffres
    for j in ['𝟎', '𝟘', '𝟢', '𝟬', '𝟶', '𝟬', '𝟢'] {
        for i in 0..10 {
            list_of_special_characters.push([
                std::char::from_u32(j as u32 + i).unwrap(),
                std::char::from_u32('0' as u32 + i).unwrap(),
            ]);
        }
    }

    let mut text = text.to_string();
    for i in list_of_special_characters {
        text = text.replace(i[0], &i[1].to_string());
    }

    text
}

/// Parsé les hashtag
/// # Examples
/// ```
/// use utils::textutils::parse_hashtag;
/// let text = "#HelloWorld";
/// let result = parse_hashtag(text);
/// assert_eq!(result, "#Hello World");
/// ```
pub fn parse_hashtag(text: &str) -> String {
    let mut result = String::new();
    let mut is_hashtag = false;

    for c in text.chars() {
        if c == '#' {
            is_hashtag = true;
            continue;
        }

        if is_hashtag {
            if c.is_uppercase() {
                result.push(' ');
            }
            is_hashtag = false;
        }

        result.push(c);
    }

    result
}

// Lister tous les fichiers d'un répertoire
fn list_files(path: &str) -> Vec<String> {
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
    let mut text = text.to_string();
    text = parse_hashtag(&text);
    text = text.trim().to_string();
    text = trim_whitespace(&text);
    text = remove_special_characters(&text);

    text
}
