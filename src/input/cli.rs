use std::io;


pub fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn input_number() {
        assert_eq!(get_input(),"5");
    }
    
    #[test]
    fn input_text() {
        assert_eq!(get_input(),"input_text");
    }
}