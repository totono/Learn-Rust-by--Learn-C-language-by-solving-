use ex::act05::ex510::ans510;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex510() {
        let array = [1,-1,2,-2,3];
        assert_eq!(crate::ans510(array),[1,2,3]);
    }
}