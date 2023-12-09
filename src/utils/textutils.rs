#[derive(Debug, Clone)]
pub struct TextUtils(String);

impl TextUtils {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    /// Parsé les hashtag
    /// # Examples
    /// ```
    /// use utils::textutils::parse_hashtag;
    /// let text = "#HelloWorld";
    /// let result = parse_hashtag(text);
    /// assert_eq!(result, "# Hello World");
    ///
    pub fn parse_hashtag(&mut self) {
        let mut result = String::new();

        let mut is_uppercase = false;
        for c in self.0.chars() {
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

        self.0 = result;
    }

    /// remove multiple spaces in a string
    /// https://stackoverflow.com/a/71864249
    /// # Examples
    /// ```
    /// use utils::textutils::trim_whitespace;
    /// let s = "  a  b  c  ";
    /// let s = trim_whitespace(s);
    /// assert_eq!(s, "a b c");
    /// ```
    pub fn trim_whitespace(&mut self) {
        let mut result = String::with_capacity(self.0.len());

        self.0.split_whitespace().for_each(|w| {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(w);
        });

        self.0 = result;
    }

    /// remove special characters
    /// # Examples
    /// ```
    /// use utils::textutils::remove_special_characters;
    /// let text = "𝐠𝐫𝐚𝐬 et 𝘪𝘵𝘢𝘭𝘪𝘤";
    /// let text = remove_special_characters(text);
    /// assert_eq!(text, "gras et italic");
    /// ```
    pub fn remove_special_characters(&mut self) {
        let mut list_of_special_characters = Vec::new();

        list_of_special_characters.append(&mut Self::get_list_remove_quotes());

        list_of_special_characters.append(&mut Self::get_list_replace_char());

        // Alphabet
        for j in [
            '𝐀', '𝐚', '𝑨', '𝒂', '𝒜', '𝒶', '𝔸', '𝕒', '𝕬', '𝖆', '𝖠', '𝖺', '𝗔', '𝗮', '𝘈', '𝘢', '𝘼',
            '𝙖', '𝙰', '𝚊', '𝐴', '𝑎', '𝔄', '𝔞', '𝓐', '𝓪',
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

        let mut text = self.0.clone();
        for i in list_of_special_characters {
            text = text.replace(i[0], &i[1].to_string());
        }

        self.0 = text;
    }

    /// Read variables
    /// # Examples
    /// ```
    /// use utils::textutils::read_vars;
    /// let text = "HelloWorld";
    /// let result = read_vars(text);
    /// assert_eq!(result, " Hello World");
    /// ```
    pub fn read_vars(&mut self) {
        let mut text = self.0.to_string();

        // on récupère les _,- par des espaces
        // Snake_case et kebab-case
        for i in ['_', '-'] {
            text = text.replace(i, " ");
        }

        // on converti le texte example: "HelloWorld" en "Hello World"
        // CamelCase
        text = Self::parse_camel_case(&text);

        self.0 = text;
    }

    fn get_list_remove_quotes() -> Vec<[char; 2]> {
        return ['"', '\'', '`', '[', ']', '{', '}']
            .iter()
            .map(|x| [*x, ' '])
            .collect();
    }

    /// list de caractères spéciaux à remplacer
    fn get_list_replace_char() -> Vec<[char; 2]> {
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

    pub fn replace(text: &str) -> String {
        // Si le répertoire existe
        if std::path::Path::new("dict/fr_FR").exists() {
            let mut text = text.to_string();

            for (replace, by) in Self::get_list_replace() {
                text = text.replace(&replace, &by);
            }
        }

        text.to_string()
    }

    fn get_list_replace() -> Vec<(String, String)> {
        let mut result = Vec::new();

        let path = Self::list_files("dict/fr_FR");

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hashtag() {
        let mut text = TextUtils::new(String::from("#HelloWorld"));
        text.parse_hashtag();
        assert_eq!(text.as_str(), "# Hello World");
    }

    #[test]
    fn test_trim_whitespace() {
        let mut text = TextUtils::new(String::from("  a  b  c  "));
        text.trim_whitespace();
        assert_eq!(text.as_str(), "a b c");
    }

    #[test]
    fn test_remove_special_characters() {
        let mut text = TextUtils::new(String::from("𝐠𝐫𝐚𝐬 et 𝘪𝘵𝘢𝘭𝘪𝘤"));
        text.remove_special_characters();
        assert_eq!(text.as_str(), "gras et italic");
    }

    #[test]
    fn test_read_vars() {
        let mut text = TextUtils::new(String::from("HelloWorld"));
        text.read_vars();
        assert_eq!(text.as_str(), " Hello World");
    }

    #[test]
    fn test_replace() {
        let text = TextUtils::replace("HelloWorld");
        assert_eq!(text, "HelloWorld");
    }
}
