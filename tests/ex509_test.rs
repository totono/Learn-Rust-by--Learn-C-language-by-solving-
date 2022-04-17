use ex::act05::ex509::ans509;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex509() {
        let array = [1,2,3,4,5];
        assert_eq!(crate::ans509(array),[5,4,3,2,1])
    }
}