pub mod ex501{
    pub fn ans501(x:u32) ->[u32;5]{
        let mut array = [0;5];

        for (i, val) in (x..=x+4).zip(array.iter_mut()) {
            *val = i;
        }
        for i in array.iter(){
            println!("{}",*i);
        }

        println!("{:?}",array);
        array
    }
}
pub mod ex502{
    pub fn ans502(x:u32) -> [u32;5]{
        let mut array:[u32;5] = [x;5];
        for (i,val) in (x-4..x).zip(array.iter_mut()).rev(){
            *val = i;
        }
        for i in array.iter().rev() {
            println!("{}",*i);
        }

        //array.reverse()は()を返すのでコンパイルエラー。なんで？
        array.reverse();
        array
    }
}

pub mod ex503{
    pub fn ans503()->[u32;5]{
        let array = [5,4,3,2,1];

        for i in array.iter() {
            println!("{}",*i);
        }
        array
    }
}

pub mod ex504{
    use crate::input::cli::get_input;
    const ELEMENTS_NUMBER:usize = 7;

    pub fn ans504() -> [u32;ELEMENTS_NUMBER]{
        let mut array:[u32;ELEMENTS_NUMBER] = [0;ELEMENTS_NUMBER];

        for i in array.iter_mut(){
            match get_input().parse(){
                Ok(n) => {*i = n},
                Err(_e) => {println!("This is not a number!");
                std::process::exit(0)}
            };
        }

        println!("{:?}",array);
        array.reverse();
        println!("{:?}",array);
        array
    }
}


//汚い　もうすこしやりようがあるはず
pub mod ex505{

    use crate::input::cli::get_input;
    pub fn ans505(scores : Vec<usize>) -> [usize;11]{

    let mut bunpu:[usize;11] = [0;11];

    for i in  0..scores.len() {
        println!("{}番\n{}",i + 1,scores[i]);
        bunpu[scores[i] / 10] += 1;
    }
    bunpu

//    for (i,item) in bunpu.iter().enumerate().take(10){
//        println!("{} ～ {}",i*10,i*10+9);
//            for _j in 0..*item{
//                print!("*");
//            }
//        println!();
//    }
//    println!("     100");
//    for _i in 0..bunpu[10]{
//        print!("*");
//    }
//        println!();

   }

   pub fn input_scores(how_many: usize) -> Vec<usize>{
        let mut vec:Vec<usize> = Vec::new();
        //let how_many = input_number();
        for i in 0..how_many{
            println!("{}番",i + 1);
            vec.push(input_number());
        }
        vec
   }

   pub fn input_number() -> usize{
       loop{
            match get_input().parse(){
                Ok(n) => return n,
                Err(e) => {println!("Invalid number: {}",e); continue}
            }
       }
    }
}
