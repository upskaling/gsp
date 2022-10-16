// remove multiple spaces in a string
pub fn remove_multiple_spaces(text: &str) -> String {
    let mut result = String::new();
    let mut last_char = ' ';
    for c in text.chars() {
        if c != ' ' || last_char != ' ' {
            result.push(c);
        }
        last_char = c;
    }
    result
}

// remove quotes ", ' and ` from a string
pub fn remove_quotes(text: &str) -> String {
    let mut result = String::new();
    let list_of_quotes = ['"', '\'', '`'];
    for c in text.chars() {
        if !list_of_quotes.contains(&c) {
            result.push(c);
        }
    }
    result
}

pub fn remove_special_characters(text: &str) -> String {
    let mut result = String::new();
    let mut list_of_special_characters = Vec::new();

    let l = ['𝐀', '𝐚', '𝐴', '𝑎', '𝑨', '𝒂', '𝒜', '𝒶', '𝘢'];

    for j in 0..l.len() {
        for i in 0..26 {
            let lower_case_letter = 97 + i;

            list_of_special_characters.push((l[j] as u32 + i, lower_case_letter as u8 as char));
        }
    }

    for c in text.chars() {
        let mut found = false;
        for i in 0..list_of_special_characters.len() {
            if c as u32 == list_of_special_characters[i].0 {
                result.push(list_of_special_characters[i].1);
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

//

pub fn text_to_dict(text: &str) -> String {
    let mut result = remove_quotes(text);
    result = remove_multiple_spaces(&result);
    result = remove_special_characters(&result);

    result
}
