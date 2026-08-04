use std::io;
use std::env;
use std::fs;
use std path::{Path,PathBuf}

fn main(){
    let significance = input("please writing command: ");

    if significance == "ls"{

    }else if significance = "cd"{

    } else if significance = "mkdir"{

    }else if significance = "touch"{

    }else{
        println!("Sorry, there's no such command ")
    }
}

fn input(promt ,&str){
    if !prompt.is_empty(){
        println!("{}",prompt)
    }
    let mut input_Command = String::new();
    io::stdin().read_line(&mut input_Command).expect("error")

    inputCommand.trim().parse::<str>.expect("please write correct command")
}

fn commad_ls(ls_promt){

}
fn commad_ls(cd_promt){
    
}
fn commad_ls(mkdir_promt){
    
}
fn commad_ls(touch_promt){
    
}