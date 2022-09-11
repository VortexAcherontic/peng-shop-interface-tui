use std::env;
use std::io;
use peng_shop_backend;

const APPLICATION_NAME:&str="Peng!Shop";
const MODULE_NAME:&str="TUI";
const VERSION:&str="0.1";

fn main(){
    let args:Vec<String> = env::args().collect();
    //dbg!(&args);
    if args.len() == 1 || &args[1] == "interactive" || &args[1] == "int" {
        interactive();
    } else {
        println!("Blub");
    }

}

fn refresh(){
    println!("Refresh not implemented yet!");
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

    match inputs[0].as_str() {
        "install" | "in" | "i" => {
            peng_shop_backend::install(inputs[0..inputs.len()].to_vec());
        },
        "uninstall" | "rm" | "r" | "u" | "un" => {
            peng_shop_backend::uninstall(inputs[0..inputs.len()].to_vec());
        },
        "refresh" | "ref" => {
            peng_shop_backend::upgrade();
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
