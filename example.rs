#!/usr/bin/env -S cargo -Z script
---cargo
[dependencies]
clap = { version = "4.2", features = ["derive"] }
duct = "0.13"
duct_sh = "0.13"
---

use clap::Parser;
use duct::*;
use duct_sh::*;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, help = "The name of the person")]
    name: String,
}

fn main() {
    let args = Args::parse();

    let name = args.name.clone();
    // runtime string executed through sh shell
    let out = sh_dangerous(format!("echo mya name is {name}")).read().unwrap();
    println!("{}", out);

    // static string executed through sh shell
    let out = sh("echo this is static str").read().unwrap();
    println!("{}", out);

    let name = args.name.clone();
    // single command executed directly
    let out = cmd!("echo", format!("my name is {name}")).read().unwrap();
    println!("{}", out);
}