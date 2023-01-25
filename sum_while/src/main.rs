use std::io;

fn change_string_to_integer(input : & String)->i64{
    let number = input.trim().parse::<i64>().unwrap();
    number
}

fn main() { 
    let mut sum = 0;
    println!("Digite os números que vao ser ser somados:");
    let mut userinput = String::new();
        io::stdin().read_line(&mut userinput).expect("Erro ao ler o input do usuário");

    let mut i64value   = change_string_to_integer(&userinput);
        while i64value != 0{
            let value = i64value %10;
            sum = sum + value;
            i64value = i64value/10;
    }
    println!("O resultado da soma é {}",sum);
}
