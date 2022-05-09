extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::motors::TachoMotor ;
// use std::thread;
// use std::time::Duration;
use std::{thread, time::Duration};
use std::env;
    
fn main() -> ev3dev_lang_rust::Ev3Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let motor = TachoMotor::find()?;

    motor.set_speed_sp(50.0 as i32)?;
    motor.set_position_sp(100 as i32)?;

    motor.run_to_rel_pos(None)?;

    // thread::sleep(Duration::from_millis(2000));
    
    motor.set_hold_pid_kd(1.0)?;
    motor.set_hold_pid_ki(1.0)?;
    // let p = motor.get_hold_pid_kd()?;
    // motor.

    let p = motor.get_speed_pid_kp()?;
    let i = motor.get_speed_pid_ki()?;

    println!("{:?} {:?}", p, i);

    // let i = motor.get_speed_pid_ki()?;
    // let d = motor.get_speed_pid_kd()?;
    
    // println!("{} {} {}", p, i, d);

    // let p = motor.get_hold_pid_kp()?;
    // let i = motor.get_hold_pid_ki()?;
    // let d = motor.get_hold_pid_kd()?;
    
    // println!("{} {} {}", p, i, d);


    Ok(())
}