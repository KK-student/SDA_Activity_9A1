use std::char;

/*
* Description: Reverses a String, ignoring all punctuation characters ('.', ',', '?', ';', '!', ':', '\'', '(', ')', '[', ']', '"', '-', '_', '/', '@', '{', '}', '*'). 
* Parameters: 
*   original: The input String to reverse. 
*/
pub fn reverse_string(original: &String) -> String {
    let length: usize = original.len();

    let punctuation: Vec<char> = Vec::from(['.', ',', '?', ';', '!', ':', '\'', '(', ')', '[', ']', '"', '-', '_', '/', '@', '{', '}', '*']);

    let mut result_chars: Vec<u8> = Vec::from(original.as_bytes());

    {
        let mut i: usize = 0;
        while i < length / 2 {
            let r_i = length - 1 - i;
            let temp: char = result_chars[r_i] as char;

            let c: char = result_chars[i] as char; 
            if !(punctuation.contains(&temp) || punctuation.contains(&c)) {
            result_chars[r_i] = result_chars[i];
            result_chars[i] = temp as u8;
            }

            i += 1;
        }
    }

    return String::from_utf8(result_chars).expect("Invalid UTF-8");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        //Test if the reverse_string method returns a String with the same or greater capacity as the length of the given original String. 
        let original: String = "abcdefg".to_string();
        let result: String = reverse_string(&original);
        assert!(result.capacity() >= original.len());
    }

    #[test]
    fn test3() {
        //Test if the reverse_string function returns the reversed version of "abcde".
        let original: String = "abcde".to_string();
        let result: String = reverse_string(&original);
        assert_eq!(result, "edcba".to_string());
    }

    #[test]
    fn test4() {
        //Test if the reverse_string function returns the reversed version of "abcde.?" ignoring
        //the punctuation.
        let original: String = "abcde.?".to_string();
        let result: String = reverse_string(&original);
        assert_eq!(result, "abedc.?".to_string());
    }

    #[test]
    fn test5() {
        //Test if the reverse_string function returns the reversed version of "?.abcde" ignoring
        //the punctuation.
        let original: String = "?.abcde".to_string();
        let result: String = reverse_string(&original);
        assert_eq!(result, "?.cbade".to_string());
    }

    #[test]
    fn test6() {
        //Testing punctuation in random places. 
        let mut original: String = "ab.c?de".to_string();
        let mut result: String = reverse_string(&original);
        assert_eq!(result, "ed.c?ba".to_string());

        original = "abc.de".to_string();
        result = reverse_string(&original);
        assert_eq!(result, "edc.ba".to_string());

        original = "abc.d".to_string();
        result = reverse_string(&original);
        assert_eq!(result, "dbc.a".to_string());
    }
}

