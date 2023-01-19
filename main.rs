// fibonacci sequence
use std::io;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    loop {
        println!("Please enter the number in the fibonacci sequence needed");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number).expect("error");
        let input = number.trim().parse::<i128>().unwrap();
        let mut a:i128 = 0;
        let mut b:i128 = 1;
        let mut c:i128 = 0;
        for _ in 0..input-1{ 
            
            c = (a + b)%10;
            a = b%10;
            b = c%10;
        }
        if input == 1{
            c = 1;
        }
        println!("{}",c);
        sleep(Duration::from_millis(500));
        println!("");
    }
}