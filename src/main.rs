use std::env;
use std::io;
use peng_shop_backend;

const APPLICATION_NAME:&str="Peng!Shop";
const MODULE_NAME:&str="TUI";
const VERSION:&str="0.1";

fn main(){
    let args:Vec<String> = env::args().collect();
    if args.len() == 1 || &args[1] == "interactive" || &args[1] == "int"{
        interactive();
    } else {
        handle_command(&args[1], args[2..args.len()].to_vec());
    }

}

fn interactive(){
    println!("Welcome to {APPLICATION_NAME} {MODULE_NAME} Version: {VERSION}");
    println!("Entering interactive mode:");
    interactive_query_command();    
}

fn interactive_query_command(){
    println!("What do you want to do?");
    let mut cmd:String = String::new();

    io::stdin()
    .read_line(&mut cmd)
    .expect("No command given");

    let split = cmd.split(" ");
    let collected:Vec<&str> = split.collect();
    let inputs:Vec<String> = collected.iter()
    .map(|&s|s.trim().into())
    .collect();

    handle_command(inputs[0].to_lowercase().as_str(), inputs[0..inputs.len()].to_vec())
}

fn handle_command(command:&str, packages:Vec<String>){
    match command {
        "install" | "in" | "i" => {
            peng_shop_backend::install(packages);
        },
        "uninstall" | "remove" | "rm" | "r"  => {
            peng_shop_backend::uninstall(packages);
        },
        "refresh" | "ref" | "U" | "update" => {
            peng_shop_backend::refresh();
        },
        "upgrade" => {
            peng_shop_backend::upgrade();
        },
        "dup" => {
            peng_shop_backend::distribution_upgrade();
        },
        "exit" | "q" | "e" => {
            println!("Exiting TUI");
            std::process::exit(0);
        }
        _ => {
            interactive_query_command();
        }
    }
}