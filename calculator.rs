use std::io;

fn main(){
    println!("Welcome Colculator");
    println!("Please choose operator who you useing");
    println!("1.'+' 2.'-' ");

    let result = input("Введите номер операции: ");
    
    if result == 1{
        println!("Введите первий число:");
        let a = input("");
        println!("Введите второй число:");
        let b = input("");
        let plus_result = plus(a,b);
        println!("Результат:{}",plus_result);
    }else if result == 2{
        println!("Введите первий число:");
        let a = input("");
        println!("Введите второй число:");
        let b = input("");
        let minus_result = minus(a,b);
        println!("Результат:{}",minus_result)
    }else{
        println!("Такой операции у нас нету");
    }


}
fn input(prompt: &str)-> i32{
    if !prompt.is_empty(){
        println!("{}",prompt)
    }

    let mut choose = String::new();
    io::stdin().read_line(&mut choose).expect("err");

    choose.trim().parse::<i32>().expect("Пожалуйста, введите корректное число!")
} 
fn plus(a:i32,b:i32) ->i32{
    a + b 
}
fn minus(a:i32,b:i32) ->i32{
    a - b  
}