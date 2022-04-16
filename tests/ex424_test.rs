use ex::act04::ex424::ans424;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex424_lower_higher() {
        assert_eq!(crate::ans424(3,5),
        "*****\n\
         *****\n\
         *****\n")
    }
    #[test]
    fn test_ex424_higher_lower() {
        assert_eq!(crate::ans424(5,3),
        "*****\n\
         *****\n\
         *****\n")
    }
    #[test]
    fn test_ex424_lower_higher2() {
        assert_eq!(crate::ans424(2,5),
        "*****\n\
         *****\n")
    }
    #[test]
    fn test_ex424_lower_higher3() {
        assert_eq!(crate::ans424(5,2),
        "*****\n\
         *****\n")
    }
}