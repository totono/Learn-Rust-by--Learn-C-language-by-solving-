pub mod ex424 {

    pub fn ans424(x: u32, y: u32) -> String {
        let mut ans = String::new();
        let high;
        let low;

        if x > y {
            high = x;
            low = y;
        } else {
            high = y;
            low = x;
        }

        for _r in 1..=low {
            for _c in 1..=high {
                ans.push('*');
            }
            ans.push('\n');
        }
        ans
    }
}

pub mod ex425 {
    pub fn ans425(x: u32, y: u32) -> String {
        let mut ans = String::new();

        let row = x / y;
        let rem = x % y;

        for _a in 1..=row {
            for _r in 1..=y {
                ans.push('*');
            }
            ans.push('\n')
        }
        for _r in 1..=rem {
            ans.push('*');
        }
        ans.push('\n');
        ans
    }
}

pub mod ex426 {
    pub fn ans426(x: u32, y: u32) -> String {
        let mut ans = String::new();

        let height = x / y;
        let rem = x % y;
        let wid = y / 2;
        let odd = y % 2;

        for _h in 1..=height / 2 {
            for _w in 1..=wid {
                ans.push_str("-+");
            }
            if odd == 1 {
                ans.push('-');
            }
            ans.push('\n');
            for _w in 1..=wid {
                ans.push_str("+-");
            }
            if odd == 1 {
                ans.push('+');
            }
            ans.push('\n');

            //if (h + w) % 2 == 1 {
            //    ans.push('+');
            //} else {
            //    ans.push('-');
            //}
        }

        for r in 1..=rem {
            if (height + r) % 2 == 0 {
                ans.push('+');
            } else {
                ans.push('-');
            }
        }
        ans.push('\n');
        ans
    }
}

pub mod ex427 {
    pub fn ans427(x: u32) -> String {
        let mut ans = String::new();

        ans.push_str("*\n");
        if x == 1 {
            ans
        } else {
            for h in 2..=x {
                for _w in 1..=h / 2 {
                    ans.push_str("**");
                }
                if h % 2 == 1 {
                    ans.push('*');
                }
                ans.push('\n');
            }
            ans
        }
    }
}

pub mod ex428 {
    pub fn ans428(x: u32) -> String {
        let mut ans = String::new();

        if x == 1 {
            ans.push('*');
        } else {
            for h in 1..=x {
                for _w in 1..=x - h {
                    ans.push(' ');
                }
                for _w in 1..=h * 2 - 1 {
                    ans.push('*');
                }
                ans.push('\n');
            }
        }
        println!("{}", ans);
        ans
    }
}

pub mod ex429 {
    pub fn ans429(x: u32) -> String {
        let mut ans = String::new();

        for h in (1..=x).rev() {
            for _w in 1..=x - h{
                ans.push(' ');
            }
            for _w in 1..=h * 2 - 1 {
                ans.push_str(&((x - h + 1) % 10).to_string());
            }
            ans.push('\n');
        }
        ans
    }
}

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

   pub fn input_scores() -> Vec<usize>{
        let mut vec:Vec<usize> = Vec::new();
        let how_many = input_number();
        for i in 0..how_many{
            println!("{}番",i);
            vec.push(input_number());
        }
        vec
   }

   fn input_number() -> usize{
       loop{
            match get_input().parse(){
                Ok(n) => return n,
                Err(e) => {println!("Invalid number: {}",e); continue}
            }
       }
    }
}