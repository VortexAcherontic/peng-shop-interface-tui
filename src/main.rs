use std::env;
use std::io;
use std::process;
use rand::Rng;
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

fn install(package:String){
    println!("Install not implemented yet!");
    peng_shop_backend::install(package);
}

fn uninstall(){ 
    println!("Uninstall not implemented yet!");
}

fn refresh(){
    println!("Refresh not implemented yet!");
}

fn interactive(){
    println!("Welcome to {APPLICATION_NAME} {MODULE_NAME} Version: {VERSION}");
    println!("You have not specified what to do.\nEntering interactive mode:");
    interactive_query_command();    
}

fn interactive_query_command(){
    println!("What do you want to do?");
    let mut cmd:String = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("No command given");
    
    println!("Running cmd '{}'", cmd.as_str().trim());
    match cmd.as_str().trim() {
        "install" => {
            install("nothing".to_string());
        },
        "exit" => {
            println!("Exiting TUI");
            std::process::exit(1);
        }
        _ => {
            interactive_query_command();
        }
    }
}
