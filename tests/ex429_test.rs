use ex::act04::ex429::ans429;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex429() {
        assert_eq!(crate::ans429(3),
        "11111\n 222\n  3\n")
    }
    #[test]
    fn test_ex429_2() {
        assert_eq!(crate::ans429(5),
        "111111111\n 2222222\n  33333\n   444\n    5\n"
    )
    }

    #[test]
    #[ignore]
    fn test_ex429_3() {
        println!("{}",crate::ans429(12));
    }
}