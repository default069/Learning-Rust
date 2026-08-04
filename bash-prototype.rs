#[allow(unused_imports)]
use std::io;
use std::fs;
use std::path::{PathBuf};

fn main(){
    let significance = input("please writing command: ");
    
    if significance == "ls" {
        command_ls(&significance);
    }else{
        println!("Sorry, there's no such command ");
    }

}
fn input(prompt: &str)-> String{
    if !prompt.is_empty(){
        println!("{}",prompt);
    }
    let mut input_Command = String::new();
    io::stdin().read_line(&mut input_Command).expect("error");

    input_Command.trim().to_string()
}
//I'm starting to write the first code for a Bash prototype
fn command_ls(_ls_prompt: &str){
    let mut path_ls = PathBuf::new();
    path_ls.push("/");
// A function to view the elements in a file 
    match fs::read_dir(path_ls){
        Ok(entries) => {
            for entry in entries {
                if let Ok(item) = entry{
                    if let Some(name) = item.file_name().to_str(){
                        println!("{}", name)
                    }
                }
            }
        }
        Err(e) => {
                println!("Error reading the directory: {}", e);
            }
    }
}