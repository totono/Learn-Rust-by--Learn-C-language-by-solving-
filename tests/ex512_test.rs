//use ex::act05::ex512::ans512;

#[cfg(test)]
mod tests{
    use ex::act05::ex512::SubjectScore;
    
    #[test]
    fn test_ex512_sum() {
        let test1 = SubjectScore{math:25,language:25};
        assert_eq!(test1.sum(),50);
    }
    #[test]
    fn test_ex512_ave() {
        let test2 = SubjectScore{math:25,language:25};
        assert_eq!(test2.average(),25.0);
    }
}