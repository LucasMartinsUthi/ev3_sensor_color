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
const WALK_SPEED: i32 = 500;

const WALK_DIST_THRESHOLD: u32 = 200;
const WALK_MIN_SPEED: u32 = 200;
const WALK_KP: f32 = 2.0;

const WHEEL_D: f32 = 2.9;
const ROBOT_D: f32 = 17.5;

const ODO: f32 = ROBOT_D/WHEEL_D;


fn main() -> Ev3Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    
    let button = Ev3Button::new()?;

    //Seting Motors
    let left_motor = LargeMotor::get(MotorPort::OutA).unwrap();
    let right_motor = LargeMotor::get(MotorPort::OutB).unwrap();

    left_motor.set_ramp_up_sp(2000)?;
    left_motor.set_ramp_down_sp(1000)?;

    right_motor.set_ramp_up_sp(2000)?;
    right_motor.set_ramp_down_sp(1000)?;

    // left_motor.reset()?;
    // right_motor.reset()?;

    // println!("{}", left_motor.get_max_speed()?);

    left_motor.set_stop_action(LargeMotor::STOP_ACTION_HOLD)?;
    right_motor.set_stop_action(LargeMotor::STOP_ACTION_HOLD)?;

    //Gyro calibrate
    let gyro = GyroSensor::find()?;
    gyro.set_mode_gyro_g_and_a()?;

    while true {        
        println!("{}", gyro.get_angle()?);
        walk(30.0,  &left_motor, &right_motor, &gyro)?;
        thread::sleep(Duration::from_secs(1));
        rotate(90.0, &left_motor, &right_motor, &gyro)?;
        thread::sleep(Duration::from_secs(5));
     
        button.process();
        if button.is_right() {
            break;
        }
    }


    Ok(())
}

fn rotate(degrees: f32, left_motor: &LargeMotor, right_motor: &LargeMotor, gyro: &GyroSensor) -> Ev3Result<()> {
    let degrees =  degrees * ODO;
    let degress = degrees as i32;

    println!("Rotate Degress: {}", degress);

    left_motor.set_speed_sp(TURN_SPEED)?;
    right_motor.set_speed_sp(TURN_SPEED)?;

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
    // let target = left_motor.get_position()? + degrees as i32;

    left_motor.set_speed_sp(WALK_SPEED)?;
    right_motor.set_speed_sp(WALK_SPEED)?;

    left_motor.run_to_rel_pos(Some(degrees))?;
    right_motor.run_to_rel_pos(Some(degrees))?;

    left_motor.wait_while(LargeMotor::STATE_RUNNING, Some(Duration::from_secs(10)));

    // let initial_pos = left_motor.get_position()?;

    
    // Comentado pq não funcionar com o gyro
    
    // left_motor.run_direct()?;
    // right_motor.run_direct()?;

    // while left_motor.get_position()? < target {
    //     let left_position = left_motor.get_position()?;

    //     let error = gyro.get_angle()? as f32 * WALK_KP;
    //     let error = cmp::min(cmp::max(error as i32, -100), 100);

    //     println!("Gyro Angle: {}", gyro.get_angle()?);
    //     println!("Error: {}", error);

    //     let dist_from_target = target - left_position;
    //     let dist_from_start = left_position - initial_pos;

    //     let velocidade =  if dist_from_target < 100 {
    //         let controle_velocidade = dist_from_target as f32 / 100.0;
    //         cmp::max(WALK_SPEED * controle_velocidade as i32, 30)

    //     } else if dist_from_start < 100 {
    //         let controle_velocidade = dist_from_start as f32 / 100.0;
    //         cmp::max(WALK_SPEED * controle_velocidade as i32, 30)
    //     } else {
    //         WALK_SPEED
    //     };

    //     // Steering
    //     let mut velocidade_a = velocidade;
    //     let mut velocidade_b = velocidade;

    //     if error < 0 {
    //         velocidade_a = velocidade - error.abs();
    //     } else {
    //         velocidade_b = velocidade - error.abs();
    //     }

    //     left_motor.set_duty_cycle_sp(velocidade_a)?;
    //     right_motor.set_duty_cycle_sp(velocidade_b)?;
    // }

    left_motor.stop()?;
    right_motor.stop()?;

    Ok(())
}
