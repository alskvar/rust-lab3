use clap::Parser;
use std::path::PathBuf;

// use ju_tcs_rust_23_25::{head, tail};
#[derive(clap::Parser)]
enum Cmd{
    Head{n: u32, file: PathBuf},
    Tail{n: u32, file: PathBuf}
}

fn head(n: u32, file: PathBuf){todo!();}
fn tail(n: u32, file: PathBuf){todo!();}

fn main() {
    let cmd = Cmd::parse();
    match cmd{
        Cmd::Head{n, file} => {
            head(n, file);
        }
        Cmd::Tail{n, file} => {
            tail(n, file);
        }
    }
}
