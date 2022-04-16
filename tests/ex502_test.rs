use ex::ex05::ex502::ans502;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex502_1() {
        let array = [5,4,3,2,1];
        assert_eq!(crate::ans502(5),array);
    }
    #[test]
    fn test_ex502_2(){
        let array = [8,7,6,5,4];
        assert_eq!(crate::ans502(8),array);
    }
}