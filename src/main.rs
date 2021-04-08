use std::io::{self, Write};
use std::thread::sleep;
use std::time;
use std::process::Command;

fn main() {
    
    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    println!("{}", String::from_utf8_lossy(&output.stdout));
    for i in 1..10 {
        println!("I can clear the screen! {}", i);
        sleep(time::Duration::from_secs(1));
        
      
        println!("{}", String::from_utf8_lossy(&output.stdout));
      
        println!("...see");
        sleep(time::Duration::from_secs(1));
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
