use ex::ex04::ex427::ans427;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex427() {
        println!("{}",crate::ans427(5));
        assert_eq!(crate::ans427(5),
        "*\n\
         **\n\
         ***\n\
         ****\n\
         *****\n")
    }
}