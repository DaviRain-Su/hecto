use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        // let b = b.unwrap();
        // let c = b as char;
        // if c.is_control() {
        //     println!("{:?}\r", b);
        // }else {
        //     println!("{:?} ({})\r", b, c);
        // }
        // if b == to_ctl_byte('q') {
        //     break;
        // }
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?}\r", b);
                }else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctl_byte('q') {
                    break;
                }
            },
            Err(err) => die(err),
        }
    }
}

fn to_ctl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!(e);
}
