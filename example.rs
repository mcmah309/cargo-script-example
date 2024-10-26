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
    let name = args.name;

    // runtime string executed through sh shell
    let out = sh_dangerous(format!("echo 1 my name is {name}")).read().unwrap();
    println!("{}", out);
    // runtime string executed through sh shell, to stdout
    let out = sh_dangerous(format!("echo 2 my name is {name}")).run().unwrap();

    // static string executed through sh shell
    let out = sh("echo 3 this is static str").read().unwrap();
    println!("{}", out);

    // single command executed directly
    let out = cmd!("echo", format!("4 my name is {name}")).read().unwrap();
    println!("{}", out);

    // Native single command
    let _child = std::process::Command::new("echo")
        .args(["5 Native"])
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .unwrap();
        // .wait().await;
}