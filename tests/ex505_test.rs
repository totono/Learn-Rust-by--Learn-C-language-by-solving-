use ex::exercise::ex505::ans505;

#[cfg(test)]
mod tests{
//    use ex::exercise::ex505::input_scores;

    #[test]
    fn test_ex505() {
       // let scores = input_scores();
       let scores = vec![10,10,20,20,50];
       let bunpu:[usize;11] = [0,2,2,0,0,1,0,0,0,0,0];
       assert_eq!(crate::ans505(scores),bunpu);

       
    }

    #[test]
    fn test_ex505_2(){

    }
}