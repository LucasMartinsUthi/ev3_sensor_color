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
use std::cmp;

const TARGET: i32 = 0;
const KP: i32 = 2;
const VELOCIDADE: i32 = 50;

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
    let button = Ev3Button::new()?;

    //Gyro calibrate
    let gyro = GyroSensor::find()?;
    gyro.set_mode_gyro_cal()?;
    thread::sleep(Duration::from_millis(3000));
    gyro.set_mode_gyro_g_and_a()?;

    //Set motors
    let motor1 = LargeMotor::get(MotorPort::OutA)?;
    let motor2 = LargeMotor::get(MotorPort::OutD)?;

    motor1.reset()?;
    motor2.reset()?;

    motor1.run_direct()?;
    motor2.run_direct()?;

    while true {
        let angle = gyro.get_angle()?;

        let erro = (TARGET - angle) * KP;
        let speed = cmp::min(cmp::max(erro, -VELOCIDADE), VELOCIDADE) as i32;
        
        let mut velocidade_a = VELOCIDADE;
        let mut velocidade_b = VELOCIDADE;

        if speed < 0 {
            velocidade_a = VELOCIDADE - speed.abs();
        } else {
            velocidade_b = VELOCIDADE - speed.abs();
        }

        println!("A: {}, B: {}", velocidade_a, velocidade_b);

        println!("Speed: {}", speed);
        println!("Angle: {}", angle);

        motor1.set_duty_cycle_sp(velocidade_b)?;
        motor2.set_duty_cycle_sp(velocidade_a)?;

        button.process();
        if button.is_right() {
            break;
        }

        //debug
        println!("Speed: {}", speed);
        println!("Angle: {}", angle);

        thread::sleep(Duration::from_millis(100));
    }

    motor1.stop()?;
    motor2.stop()?;

    let a: u8 = 10;

    debug(&a)

    println("{}", a)
    
    Ok(())}

fn debug(a: u8) {
    println("{}", a)
}
