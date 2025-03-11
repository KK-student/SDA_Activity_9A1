use std::char;

pub fn reverse_string(original: &String) -> String {
    let original_bytes = original.as_bytes();
    let length: usize = original.len();

    /*
    let mut result: String = String::with_capacity(length);

    {
        let mut i: usize = 0;
        while i < length {
            result.push(' ');

            i += 1;
        }
    }

    {
        let mut i: usize = 0;
        while i < length {
            let r_i = length - 1 - i;
            //result.insert(r_i, original_bytes[i] as char);
            //result.replace_range(r_i..r_i, &original_bytes[i].to_string()[0..0]);
            result.chars().nth(r_i).unwrap() = original_bytes[i];

            i += 1;
        }
    }

    return result;
    */

    let mut result_chars: Vec<u8> = Vec::with_capacity(length);

    {
        let mut i: usize = 0;
        while i < length {
            result_chars.push(' ' as u8);

            i += 1;
        }
    }

    {
        let mut i: usize = 0;
        while i < length {
            let r_i = length - 1 - i;
            result_chars[r_i] = original_bytes[i];

            i += 1;
        }
    }

    return String::from_utf8(result_chars).expect("Invalid UTF-8");
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test] 
    fn test1() {
        //Test if there is a reverse_string method that accepts 1 String parameter and returns an
        //empty string. 
        let original: String = "abcd".to_string(); //Could be any string. 
        let result: String = reverse_string(&original);
        assert_eq!(result, String::new());
    }

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
}

