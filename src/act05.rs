pub mod ex501 {
    pub fn ans501(x: u32) -> [u32; 5] {
        let mut array = [0; 5];

        for (i, val) in (x..=x + 4).zip(array.iter_mut()) {
            *val = i;
        }
        for i in array.iter() {
            println!("{}", *i);
        }

        println!("{:?}", array);
        array
    }
}
pub mod ex502 {
    pub fn ans502(x: u32) -> [u32; 5] {
        let mut array: [u32; 5] = [x; 5];
        for (i, val) in (x - 4..x).zip(array.iter_mut()).rev() {
            *val = i;
        }
        for i in array.iter().rev() {
            println!("{}", *i);
        }

        //array.reverse()は()を返すのでコンパイルエラー。なんで？
        array.reverse();
        array
    }
}

pub mod ex503 {
    pub fn ans503() -> [u32; 5] {
        let array = [5, 4, 3, 2, 1];

        for i in array.iter() {
            println!("{}", *i);
        }
        array
    }
}

pub mod ex504 {
    use crate::input::cli::get_input;
    const ELEMENTS_NUMBER: usize = 7;

    pub fn ans504() -> [u32; ELEMENTS_NUMBER] {
        let mut array: [u32; ELEMENTS_NUMBER] = [0; ELEMENTS_NUMBER];

        for i in array.iter_mut() {
            match get_input().parse() {
                Ok(n) => *i = n,
                Err(_e) => {
                    println!("This is not a number!");
                    std::process::exit(0)
                }
            };
        }

        println!("{:?}", array);
        array.reverse();
        println!("{:?}", array);
        array
    }
}

pub mod ex505 {

    use crate::input::cli::get_input;
    pub fn create_bunpu(scores: &[usize]) -> [usize; 11] {
        let mut bunpu: [usize; 11] = [0; 11];

        for i in 0..scores.len() {
            println!("{}番\n{}", i + 1, scores[i]);
            bunpu[scores[i] / 10] += 1;
        }
        bunpu
    }

    pub fn input_scores(how_many: usize) -> Vec<usize> {
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..how_many {
            println!("{}番", i + 1);
            vec.push(input_number());
        }
        vec
    }

    pub fn input_number() -> usize {
        loop {
            match get_input().parse() {
                Ok(n) => return n,
                Err(e) => {
                    println!("Invalid number: {}", e);
                    continue;
                }
            }
        }
    }
}

pub mod ex506 {
    use super::ex505::{create_bunpu, input_number, input_scores};

    pub fn ans506() {
        println!("How many people?");
        let how_many = input_number();
        let scores = input_scores(how_many);
        let bunpu = create_bunpu(&scores);

        let bunpu_max = match bunpu.iter().max() {
            Some(n) => *n,
            None => 0,
        };

        for i in (1..=bunpu_max).rev() {
            for j in &bunpu {
                if *j >= i {
                    print!(" * ");
                } else {
                    print!("   ");
                }
            }
            println!();
        }

        for _i in 0..33 {
            print!("-");
        }
        println!();
        for i in 0..=10 {
            print!(" {}", i * 10);
        }
        println!();
    }
}

pub mod ex506_struct {
    use super::ex505::{create_bunpu, input_number, input_scores};

    struct ScoreGraph {
        //scores : Vec<usize>,
        bunpu: [usize; 11],
    }

    impl ScoreGraph {
        fn new() -> Self {
            println!("Please enter how many people");
            let scores = input_scores(input_number());
            ScoreGraph {
                //scores:sc,
                bunpu: create_bunpu(&scores),
            }
        }

        fn display_graph_tate(&self) {
            for (i, item) in self.bunpu.iter().enumerate().take(10) {
                println!("{} ～ {}", i * 10, i * 10 + 9);
                for _j in 0..*item {
                    print!("*");
                }
                println!();
            }
            println!("     100");
            for _i in 0..self.bunpu[10] {
                print!("*");
            }
            println!();
        }

        fn display_graph_yoko(&self) {
            let bunpu_max = match self.bunpu.iter().max() {
                Some(n) => *n,
                None => 0,
            };

            for i in (1..=bunpu_max).rev() {
                for j in &self.bunpu {
                    if *j >= i {
                        print!(" * ");
                    } else {
                        print!("   ");
                    }
                }
                println!();
            }

            for _i in 0..33 {
                print!("-");
            }
            println!();
            for i in 0..=10 {
                print!(" {}", i * 10);
            }
            println!();
        }
    }

    pub fn ans506_struct() {
        let graph1 = ScoreGraph::new();
        let graph2 = ScoreGraph::new();

        println!("First graph");
        graph1.display_graph_yoko();
        println!("Second graph");
        graph2.display_graph_tate();
    }
}


pub mod ex509 {
    pub fn ans509(mut array : [i32;5]) -> [i32;5]{
        array.reverse();
        array
    }
}

pub mod ex510{
    pub fn ans510(array : [i32;5]) -> Vec<i32>{
       let ans:Vec<i32> = array.into_iter().filter(|x| *x > 0).collect();
       ans
    }
}