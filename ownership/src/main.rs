fn say_hello(name: &String){
    println!("Hello {name}");
}

fn say_goodbye(name: String){
    println!("Goodbye {name}");
}

fn main() {
    let a: i32 = 1;// tipos Copy: float, boolean, char, integer
    let b = &a;
    println!("Hello, world! {a} {}",*b);
    // s é owner da string "Gabriel"
    let s = String::from("Gabriel");
    // name é owner do valor de s
    let name = s + " Luiz";
    
    //println!("{s}"); - não funciona porque o valor de s foi passado para a variável name

    say_hello(&name);
    
    // s é owner da string "Gabriel"
    let s = String::from("Gabriel");
    // name pegou emprestado do valor de s através de um ponteiro
    let name = &s;
    println!("{s}");
    let name = name.to_owned()+&" Luiz".to_string();
    say_goodbye(name);
}
