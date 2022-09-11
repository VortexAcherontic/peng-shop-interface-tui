use std::env;
use std::io;
use std::process;
use rand::Rng;

const APPLICATION_NAME:&str="Peng!Shop";
const MODULE_NAME:&str="TUI";
const VERSION:&str="0.1";

fn main(){
    let args:Vec<String> = env::args().collect();
    //dbg!(&args);
    interactive();

}

fn install(){
    println!("Install not implemented yet!");
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
    
        println!("Running cmd {}", cmd.as_str());
        match cmd.as_str() {
            "install" => {
                install();
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
