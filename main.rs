// use std::io;

fn main(){
    //  output
    let name = "abubakir";
    println!("привет {}!", name);
    // ----------------------------------------------------------------------------------//
    //input func your writing "use std::io;" starting file 
    println!("ismingizni kiriting ");
    let mut user = String::new();
    io::stdin()
        .read_line(&mut user)
        .expect("error man :(");

    println!("Здраствуйте мистер {}",user.trim());
    // ----------------------------------------------------------------------------------//
    // if else  оператор
    let age = 12;

    if age >= 16{
        println!("Siz Passport olishingi mumkin :)");
    }else{
        println!("siz xali 16 tiga tolmagansiz shuning uchun passport ololmaysiz :(");  
    }
    // ----------------------------------------------------------------------------------//
    //Loop цикл 
    let mut cycle = 0;

    loop{
        cycle += 1;
        if cycle > 10{
            println!("выход из цикла ");
            break;
        }
        println!("Счетчик:{}", cycle)
    }
    // ----------------------------------------------------------------------------------//
    // while operator он на оборот шитаеть вот 10 -> 0
    let mut number = 3;

    while number > 0{
        println!("{}", number);
        number -= 1;
    }
    println!("start")
    // ----------------------------------------------------------------------------------//
    // for оператор цикл 
    for i in 1..10{
        println!("{}",i);
    }
    // ----------------------------------------------------------------------------------//
    // Функсии и как сними рабоат
    let result = add(92,12);
    println!("Result {}", result);
    user_name_developer("Abubakir");

}
// creating func  
fn add(a:i32, b:i32)->i32{
    a + b
}
fn user_name_developer(name: &str){
    println!("Hello Mr {}",name)
}