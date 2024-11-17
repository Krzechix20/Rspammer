use std::io::{self, Write};
use enigo::*;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    print!("RSpammer v.1.0.0\n");
    loop {
        let mut enigo = Enigo::new();
        print!("Enter text to type: ");
        io::stdout().flush().unwrap();
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let text = text.trim();
        print!("Enter the number of times to type: ");
        io::stdout().flush().unwrap();
        let mut times = String::new();
        io::stdin().read_line(&mut times).unwrap();
        let times = times.trim().parse::<i32>().unwrap();
        print!("Enter the delay in seconds between each word (e.g. 1.5): ");
        io::stdout().flush().unwrap();
        let mut delay = String::new();
        io::stdin().read_line(&mut delay).unwrap();
        let delay = delay.trim().parse::<f64>().unwrap();
        print!("Starting in: ");
        for i in (1..=5).rev() {
            print!("{}...", i);
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }
        println!();
        for _ in 0..times {
            enigo.key_sequence(text);
            enigo.key_click(Key::Return);
            sleep(Duration::from_secs_f64(delay));
        }
        print!("\nWould you like to start again? (y/n): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        if choice.trim().to_lowercase() != "y" {
            println!("Goodbye!");
            break;
        }
        println!("\n--- Starting New Session ---\n");
    }
}