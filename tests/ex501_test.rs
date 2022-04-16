use ex::ex05::ex501::ans501;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex501_1() {
        let array = [1,2,3,4,5];
        assert_eq!(crate::ans501(1),array);
}
    #[test]
    fn test_ex501_2(){
        let array = [3,4,5,6,7];
        assert_eq!(crate::ans501(3),array);
    }
}