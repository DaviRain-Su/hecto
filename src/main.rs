use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        // let c = b.unwrap() as char ;
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?}\r", b);
        }else {
            println!("{:?} ({})\r", b, c);
        }
        // println!("{}", c);
        if c == 'q' {
            break;
        }
    }
}
