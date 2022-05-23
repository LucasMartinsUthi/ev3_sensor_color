extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::{GyroSensor, ColorSensor};
use ev3dev_lang_rust::motors::{TachoMotor, MotorPort, LargeMotor};
use std::thread;
use std::time::Duration;

// use ev3dev_lang_rust;
use std::env;
use std::sync::mpsc;

fn main() -> Ev3Result<()> {
    let button = Ev3Button::new()?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    while true {
        button.process();
        if button.is_right() {
            println!("{:?}", rx.recv().unwrap());    
        }

        if button.is_left() {
            break;
        }
    }

    Ok(())
}
