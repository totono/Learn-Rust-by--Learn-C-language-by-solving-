use ex::exercise::ex426::ans426;

#[cfg(test)]
mod tests{
    #[test]
    fn test_ex426() {
        assert_eq!(crate::ans426(33,5),
        "-+-+-\n\
         +-+-+\n\
         -+-+-\n\
         +-+-+\n\
         -+-+-\n\
         +-+-+\n\
         -+-\n")
        
    }
}