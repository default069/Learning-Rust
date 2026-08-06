#[allow(unused_imports)]
#[warn(unused_variables)]
use std::io;
use std::fs;
use std::env;
use std::path::{PathBuf};

fn main(){
loop{
    let significance = input("please writing command: ");

    let cleaned_input = significance.trim();

    if cleaned_input.is_empty(){
        continue;
    }

    let mut parts = cleaned_input.split_whitespace();
    let command = parts.next().unwrap_or("");
    let argument = parts.next().unwrap_or("");

    match command {
        "ls" => {
            command_ls("argument");
        }
        "cd" => {
            command_cd("argument");
        }
        "exit" => {
            println!("stop terminal");
            break;
        }
        _=>{
            println!("Sorry, there's no such command")
        }

    }
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
    path_ls.push("/home/user");
// A function to view the elements in a file 
    match env::current_dir(){
        Ok(path_ls)=>{
            match fs::read_dir(path_ls){
                Ok(entries)=>{
                    for entry in entries{
                        if let Ok(item) = entry{
                            if let Some(name) = item.file_name().to_str(){
                                println!("{}", name);
                            }
                        }
                    }
                }
                Err(e) => println!("error reading the directory:{}", e),
            }
        }
         Err(e) => println!("error getting the directory:{}",e),
    }
}
fn command_cd(_cd_prompt: &str){
// A function to enter the elements in a file
    let mut path_cd = PathBuf::new();
    path_cd.push(&_cd_prompt);
    if !path_cd.exists() {
        // Создаем папку и обрабатываем возможную ошибку при создании
        if let Err(e) = fs::create_dir_all(&path_cd) {
            println!("Не удалось создать директорию: {}", e);
            return;
        }
    }
    match env::set_current_dir(&path_cd){
        Ok(_)=>{
            println!("Successfully changed working {}!",path_cd.display());
        }
        Err(e)=>{
            println!("cd error:{}",e)
        }
    }
}