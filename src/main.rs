extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::motors::LargeMotor ;
// use std::thread;
// use std::time::Duration;
use std::{thread, time::Duration};
// use ev3dev_lang_rust;
use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use std::env;
    
fn main() -> Ev3Result<()> {
    // TODO
    // Colocar na maior velocidade possível
    // Girar 90 Graus
    // tunar o PID

    //usar clap para setar kp ki kd
    env::set_var("RUST_BACKTRACE", "1");
    let motor = LargeMotor::find()?;
    motor.reset()?;

    motor.set_speed_pid_kp(10.0)?;
    motor.set_speed_pid_ki(2.0)?;
    motor.set_speed_pid_kd(1.0)?;

    motor.set_hold_pid_kp(10.0)?;
    motor.set_hold_pid_ki(2.0)?;
    motor.set_hold_pid_kd(1.0)?;


    motor.set_stop_action("hold")?;
    motor.set_speed_sp(800)?;
    motor.set_position_sp(90)?;

    println!("pos {:?}", motor.get_position()?);
    println!("{:?}", motor.get_position_sp()?);

    motor.run_to_rel_pos(None)?;

    motor.wait_until_not_moving(None);

    

    println!("{:?}", motor.get_position_sp()?);
    

    // println!("começpu");

    // motor.run_forever()?;
    // let button = Ev3Button::new()?;
    // while true {
    //     let position = motor.get_position()? as f32;
    //     let count_per_rot = motor.get_count_per_rot()? as f32;
    //     println!("Position {}", position);
    //     println!("CountPerRot{}", count_per_rot);
    //     // Calculate the rotation count.
    //     let rotations = position / count_per_rot;

    //     println!("The motor did {:.2} rotations", rotations);

    //     button.process();
    //     if button.is_right() {
    //         break;
    //     }

    //     thread::sleep(Duration::from_millis(500));
    // }

    motor.stop()?;

    println!("{:?}", motor.get_position_sp()?);
    println!("pos {:?}", motor.get_position()?);

    Ok(())
}

