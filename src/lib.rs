/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test1() {
        let original: String = "abcd".to_string(); //Could be any string. 
        let result: String = reverse_string(original);
        assert_eq!(result, String::new());
    }
}

