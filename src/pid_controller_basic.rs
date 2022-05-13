extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::ColorSensor;
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
    #[clap(short, long)]
    dir: i32,
    speed: i32,
    pos: i32,
    p: f32,
    i: f32,
    d: f32
}
    
fn main() -> Ev3Result<()> {
    // TODO
    // Colocar na maior velocidade possível
    // Girar 90 Graus
    // tunar o PID

    //usar clap para setar kp ki kd
    env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();

    let arg_p = args.p;
    let arg_i = args.i;
    let arg_d = args.d;

    let motor = LargeMotor::get(MotorPort::OutA)?;
    let motor_default = LargeMotor::get(MotorPort::OutB)?;

    motor.reset()?;
    motor_default.reset()?;

    println!("Position: {}", motor.get_position()?);
    println!("Position Default: {}", motor_default.get_position()?);
    // println!("Travel count Final: {}", motor.get_full_travel_count()?);
    let dir = match args.dir {
        1 => { -1 },
        _ => { 1 }
    };

    println!("Dir: {}", dir);
    
    motor.set_speed_sp(args.speed)?;
    motor.set_position_sp(args.pos * dir)?;
    motor.set_stop_action(TachoMotor::STOP_ACTION_HOLD)?;

    motor.set_hold_pid_kp(arg_p)?;
    motor.set_hold_pid_ki(arg_i)?;
    motor.set_hold_pid_kd(arg_d)?;

    motor.set_speed_pid_kp(arg_p)?;
    motor.set_speed_pid_ki(arg_i)?;
    motor.set_speed_pid_kd(arg_d)?;

    motor_default.set_speed_sp(args.speed)?;
    motor_default.set_position_sp(args.pos * dir)?;
    motor_default.set_stop_action(TachoMotor::STOP_ACTION_HOLD)?;

    motor.run_to_rel_pos(None)?;
    motor_default.run_to_rel_pos(None)?;

    motor.wait_while(TachoMotor::STATE_RUNNING, Some(Duration::from_secs(3)));

    println!("#### SPEED ####");
    let p = motor.get_speed_pid_kp()?;
    let i = motor.get_speed_pid_ki()?;
    let d = motor.get_speed_pid_kd()?;
    println!("p: {:?} i: {:?} d: {:?}", p, i, d);
    
    let p = motor_default.get_speed_pid_kp()?;
    let i = motor_default.get_speed_pid_ki()?;
    let d = motor_default.get_speed_pid_kd()?;
    println!("Default p: {:?} i: {:?} d: {:?}", p, i, d);

    println!("#### HOLD ####");
    let p = motor.get_hold_pid_kp()?;
    let i = motor.get_hold_pid_ki()?;
    let d = motor.get_hold_pid_kd()?;
    println!("p: {:?} i: {:?} d: {:?}", p, i, d);
    
    let p = motor_default.get_hold_pid_kp()?;
    let i = motor_default.get_hold_pid_ki()?;
    let d = motor_default.get_hold_pid_kd()?;
    println!("Default p: {:?} i: {:?} d: {:?}", p, i, d);

    //wait 2 seconds
    
    thread::sleep(Duration::from_secs(2));
    println!("--------------------------------------");

    println!("Position Final: {}", motor.get_position()?);
    println!("Position Default Final: {}", motor_default.get_position()?);
    // println!("Travel count Final: {}", motor.get_full_travel_count()?);

    motor.stop()?;
    Ok(())
}

