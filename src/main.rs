use std::{fmt::format, process::Output};
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
    secs: u64
}

impl Timer {
    pub fn new(time: String) -> Timer{
        let times: Vec<u64> = time.split(":").map(|x| x.trim()).filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
        println!("Parsed: {:?}", times);
        let mut parsed_secs = 0;
        parsed_secs += times[0] * 3600;
        parsed_secs += times[1] * 60;
        parsed_secs += times[2];

        return Timer {
            secs: parsed_secs
        };
    }

    pub fn tick(&mut self) -> String {
        self.secs -= 1;
        return self.str();
    }

    pub fn str(&mut self) -> String {
        let hours = ((self.secs / 3600) as f64).floor();
        self.secs %= 3600;
        let min = ((self.secs / 60) as f64).floor();
        let sec = ((self.secs % 60) as f64).floor();
          
        return format!("{:>0zero$}:{:>0zero$}:{:>0zero$}", hours, min, sec, zero=2);
    }

    pub fn is_ended(&self) -> bool {
        return self.secs == 0;
    }
}

fn main() {

    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    //println!("{}", String::from_utf8_lossy(&output.stdout));
    
    let mut timer = Timer::new("00:01:00".to_string());

    while !timer.is_ended() {
        println!("{}", String::from_utf8_lossy(&output.stdout)); // console clear
        println!("{}", timer.tick());  
        sleep(time::Duration::from_secs(1));
    }
}
