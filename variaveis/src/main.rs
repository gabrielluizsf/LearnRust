const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    let name = "Gabriel";
    let github_url: &str = "https://github.com/gabrielluizsf";

    let year = 2022;
    let age = 21;
    print_info(age, year, name, github_url);
    {
        let age = 22;
        let year = 2023;
        print_info(age, year, name, github_url);
    }
    {
        let age = 23;
        let year = 2024;
        print_info(age, year, name, github_url);
    }
    let total_hours: u32 = 30;
    print_hours_in_seconds(total_hours)
}

fn print_hours_in_seconds(total_hours: u32){
    let hours_in_seconds = total_hours * SECONDS_IN_HOUR;
    println!("Hours in seconnds: {}", hours_in_seconds);
}

fn print_info(age: i32, year: i32, name: &str, github_url: &str) {
    println!("Year: {}", year);
    println!("Hello, {}!", name);
    println!("Github: {}", github_url);
    println!("Age: {}", age);
}
