extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::{GyroSensor, ColorSensor};
use ev3dev_lang_rust::motors::{TachoMotor, MotorPort, LargeMotor};
// use std::thread;
// use std::time::Duration;
use std::{thread, time::Duration};

// use ev3dev_lang_rust;
use std::env;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // #[clap(short, long)]
    // dir: i32,
    // speed: i32,
    // pos: i32,
    // p: f32,
    // i: f32,
    // d: f32
}
    
fn main() -> Ev3Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();

    let gyro = GyroSensor::find()?;

    gyro.set_mode_gyro_cal()?;
    thread::sleep(Duration::from_millis(3000));
    gyro.set_mode_gyro_g_and_a()?;

    while true {
        let angle = gyro.get_angle()?;
        println!("Angle: {}", angle);

        thread::sleep(Duration::from_millis(100));
    }


    
    Ok(())
}

