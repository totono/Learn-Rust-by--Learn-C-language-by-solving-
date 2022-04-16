use ex::ex05::ex505::ans505;

#[cfg(test)]
mod tests {
    //    use ex::exercise::ex505::input_scores;

    use ex::ex05::ex505::{input_number, input_scores};

    #[test]
    fn test_ex505() {
        let scores = vec![10, 10, 20, 20, 50];
        let bunpu: [usize; 11] = [0, 2, 2, 0, 0, 1, 0, 0, 0, 0, 0];
        assert_eq!(crate::ans505(scores), bunpu);
    }

    #[test]
    fn test_ex505_2() {
        println!("How many people?");
        let how_many = input_number();
        let scores = input_scores(how_many);
        let bunpu = crate::ans505(scores);

        for (i, item) in bunpu.iter().enumerate().take(10) {
            println!("{} ～ {}", i * 10, i * 10 + 9);
            for _j in 0..*item {
                print!("*");
            }
            println!();
        }
        println!("     100");
        for _i in 0..bunpu[10] {
            print!("*");
        }
        println!();
    }
}
