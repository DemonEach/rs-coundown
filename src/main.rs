use std::{io::{self, Write}, process::Output, time::Instant};
use std::thread::sleep;
use std::time;
use std::process::Command;
use std::env;

/// ConsoleCleaner 
struct ConsoleCleaner {
    cleaner: Output
}

impl ConsoleCleaner {
    pub fn new(&mut self) {
        //TODO: add cleanup based of OS
        //println!("{}", env::consts::OS);
        self.cleaner = Command::new("clear").output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    }
}

struct Timer {
    timer: Instant
}

fn main() {

    sleep(time::Duration::from_secs(1));

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
