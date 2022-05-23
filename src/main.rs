extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::{GyroSensor, ColorSensor};
use ev3dev_lang_rust::motors::{TachoMotor, MotorPort, LargeMotor};
use ev3dev_lang_rust::Port;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

// use ev3dev_lang_rust;
use std::env;
use std::sync::mpsc;
use std::cmp;

const MAX_SPEED: i32 = 1050;

const TURN_SPEED: i32 = 500;
const WALK_SPEED: i32 = 700;

const GYRO_ERROR_ROTATE: f32 = 2.0;

const WALK_DIST_THRESHOLD: u32 = 200;
const WALK_MIN_SPEED: u32 = 200;
const WALK_KP: f32 = 2.0;

const WHEEL_D: f32 = 3.2;
const ROBOT_D: f32 = 17.5;

const ODO: f32 = ROBOT_D/WHEEL_D;


fn main() -> Ev3Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    
    let button = Ev3Button::new()?;

    //Seting Motors
    let left_motor = LargeMotor::get(MotorPort::OutA).unwrap();
    let right_motor = LargeMotor::get(MotorPort::OutB).unwrap();

    left_motor.set_ramp_up_sp(1500)?;
    left_motor.set_ramp_down_sp(500)?;

    right_motor.set_ramp_up_sp(1500)?;
    right_motor.set_ramp_down_sp(500)?;

    left_motor.set_stop_action(LargeMotor::STOP_ACTION_HOLD)?;
    right_motor.set_stop_action(LargeMotor::STOP_ACTION_HOLD)?;

    //Gyro
    let gyro = GyroSensor::find()?;
    gyro.set_mode_gyro_g_and_a()?;

    while true {        
        walk(30.0,  &left_motor, &right_motor, &gyro)?;
        thread::sleep(Duration::from_secs(1));
        rotate(90.0, &left_motor, &right_motor, &gyro)?;
        thread::sleep(Duration::from_secs(1));
     
        button.process();
        if button.is_right() {
            break;
        }
    }


    Ok(())
}

fn rotate(target: f32, left_motor: &LargeMotor, right_motor: &LargeMotor, gyro: &GyroSensor) -> Ev3Result<()> {
    let degrees =  target * ODO;
    let degress = degrees as i32;

    let start_degree = gyro.get_angle()?;

    left_motor.set_speed_sp(TURN_SPEED)?;
    right_motor.set_speed_sp(TURN_SPEED)?;

    left_motor.run_to_rel_pos(Some(-degress))?;
    right_motor.run_to_rel_pos(Some(degress))?;

    left_motor.wait_while(LargeMotor::STATE_RUNNING, Some(Duration::from_secs(15)));

    // Correction
    let error = (start_degree + target as i32)  - gyro.get_angle()?;
    let error = error as f32 / GYRO_ERROR_ROTATE;

    println!("Error Rotate: {}", error);
    let degrees =  error * ODO;
    let degress = degrees as i32;

    left_motor.run_to_rel_pos(Some(-degress))?;
    right_motor.run_to_rel_pos(Some(degress))?;

    left_motor.wait_while(LargeMotor::STATE_RUNNING, Some(Duration::from_secs(10)));

    left_motor.stop()?;
    right_motor.stop()?;

    Ok(())
}

fn walk(distance: f32, left_motor: &LargeMotor, right_motor: &LargeMotor, gyro: &GyroSensor)  -> Ev3Result<()> {
    let wheel_circ = 2.0 * PI * (WHEEL_D / 2.0);
    let degrees = ((distance / wheel_circ) * 360.0) as i32;

    left_motor.set_speed_sp(WALK_SPEED)?;
    right_motor.set_speed_sp(WALK_SPEED)?;

    left_motor.run_to_rel_pos(Some(degrees))?;
    right_motor.run_to_rel_pos(Some(degrees))?;

    left_motor.wait_while(LargeMotor::STATE_RUNNING, Some(Duration::from_secs(15)));

    left_motor.stop()?;
    right_motor.stop()?;

    Ok(())
}
