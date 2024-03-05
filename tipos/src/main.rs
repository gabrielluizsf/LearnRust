fn main() {
    // Inteiros
    // bits | signed | unsigned
    // 8    | i8     | u8
    // 16   | i16    | u16
    // 32   | i32    | u32
    // 64   | i64    | u64
    // 128  | i128   | u128
    // arch | isize  | usize
    let number: u64 = 23; 
    {
        let big_number = 500_374_934;
        let hexadecimal = 0xff;
        let octal = 0o77;
        let binário = 0b1111_0000;
        let byte = b'A';
        println!("{} {} {} {} {}", big_number, hexadecimal, octal, binário, byte);
        let number:i64 = -40;
        println!("Número negativo: {}",number);
    }
    // Pontos fluantes
    // bits | 
    // 32   | f32    
    // 64   | f64
    let float_number: f64 = 20.50;
    // Booleanos
    // true | false
    let boolean: bool = true;
    // Caractere
    // bytes |
    //   4   | char
    let emoji = '👍';
    println!("Caractere: {}\nNúmero positivo: {}\nPonto flutuante: {}\nBooleano:{}",emoji,number,float_number,boolean);
    //tupla
    let numbers: (i32, i32, f32) = (42, 30, 50.5);
    println!("[print no modo debug] tupla: {:?}", numbers);
    print_elements(numbers.0, numbers.1, numbers.2);
    let (a, b, c) = numbers;
    print_elements(a,b,c);
    // Matrizes == Array
    let numbers: [i32;3] = [20, 10,30];
    print_elements(numbers[0], numbers[1], 50.5);
    // slicing
    println!("{:?}", &numbers[..2]);
}

fn print_elements(e1: i32, e2: i32, e3: f32){
    println!("Elemento 1: {} Elemento 2: {} Elemento 3: {}", e1, e2, e3);
}
