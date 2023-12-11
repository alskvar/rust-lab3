use std::ffi::CString;
use clap::Parser;
use std::path::PathBuf;

use ju_tcs_rust_23_25::{head, tail};
// use lib::ju-tcs-rust-23-25::{head, tail};



// extern "ju-tcs-rust-23-25"
#[derive(clap::Parser)]
enum Cmd{
    Head{n: usize, file: PathBuf},
    Tail{n: usize, file: PathBuf}
}


// fn head(_:u32, _:PathBuf){todo!();}
// fn tail(_: u32, _: PathBuf){todo!();}

fn main() {
    let cmd = Cmd::parse();
    let mut res: Vec<String>;
    match cmd{
        Cmd::Head{n, file} => {
            res = head(file.as_path(), n);
        }
        Cmd::Tail{n, file} => {
           res = tail(file.as_path(), n);
        }
    }
    for i in res{
        println!("{}", i);
    }
}
