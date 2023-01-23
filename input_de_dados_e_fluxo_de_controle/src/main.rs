use std::io;

fn change_string_to_integer(input: & String)->i64{
   let value = input.trim().parse::<i64>().unwrap();
    value
}

fn main(){
    let mut number  = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut number).expect("Erro na leitura da variável");

    let mut number2 = String::new();
    println!("Digite um número: ");
    io::stdin().read_line(&mut number2).expect("Erro na leitura da variável");

    if change_string_to_integer(&number) > change_string_to_integer(&number2){
        println!("O número {} é maior que {}",number,number2);
    }else if change_string_to_integer(&number) == change_string_to_integer(&number2){
        println!("Os números são iguais");
    }else{
        println!("O número {} é menor que {}", number,number2);
    }
}
