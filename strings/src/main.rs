use std::io;

fn main() {
    //matriz de caracteres
    let name = ['G', 'a', 'b', 'r', 'i', 'e', 'l'];
    //caractere
    let l0 = name[0];
    println!(
        "Caracteres: Hello, {l0}{}{}{}{}{}{}",
        name[1], name[2], name[3], name[4], name[5], name[6]
    );
    let s: String = String::from_iter(name);
    println!("String from array: {s}");
    // Static memory
    //
    let name: &str = "Gabriel"; // str == static string &str == string reference(pointer)
    println!("String Pointer: {name}");
    // Heap Memory
    // undefinied chars - dynamic string
    let mut s = String::new();
    s.push_str(name);
    let s = String::from(name);
    let void_string = String::new();
    println!("Heap String: {s}");
    hello(void_string.clone());
    sum(void_string.clone())
}

fn sum(mut s: String){
    let banner = "Digite os números que serão somados \
                - devem ser separados por virgula    
                exemplo: 1,2,3,47,32,90
    ";
    println!("{banner}");
    io::stdin()
        .read_line(&mut s)
        .expect("Erro ao ler input do usuário");
    let nums : Vec<i32> = s.split(",")
        .map(|c| c.trim().parse().expect("Erro ao converter para i32"))        
        .collect();
    let result: i32 = nums.iter().sum();
    
    println!("Resultado da soma: {result}");
}

fn hello(mut s: String) {
    println!("Digite seu nome");
    io::stdin()
        .read_line(&mut s)
        .expect("Erro ao ler input do usuário");
    println!("Hello {s}");    
    println!("{}", "-".repeat(50));
    println!("Quantidade de letras {}", s.trim().chars().count())
}
