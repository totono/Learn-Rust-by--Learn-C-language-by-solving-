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
        //println!("{:?}",array.reverse());

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

    const ELEMENTS:usize = 7;
    pub fn ans504() -> [u32;ELEMENTS]{
        let mut array = [0;ELEMENTS];

        for i in array.iter_mut(){
            match get_input().parse(){
                Ok(n) => {*i = n},
                Err(_e) => {println!("This is not a number!");
                std::process::exit(0)}
            };
        }

        array
    }
}