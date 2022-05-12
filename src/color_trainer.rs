extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::ColorSensor;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::error::Error;

use csv::WriterBuilder;
use csv::{ ReaderBuilder, StringRecord };
use serde::Serialize;
use serde::Deserialize;
use clap::Parser;
use std::str::FromStr;
// use std::{thread, time::Duration};

const CSV_FILE_PATH: &str = "foo.csv";
const H_SIZE: usize = 360;
const V_SIZE: usize = 100;

type ColorMap = Vec<Vec<ColorTrain>>;

type RGB = (u8, u8, u8);
type HSV = (usize, usize, usize);

#[derive(Deserialize, Serialize, Clone, Debug)]
enum ColorTrain {
    Red,
    Blue,
    Yellow,
    White,
    Black,
    None,
}

impl FromStr for ColorTrain {

    type Err = ();

    fn from_str(input: &str) -> Result<ColorTrain, Self::Err> {
        match input {
            "Red" => Ok(ColorTrain::Red),
            "Blue" => Ok(ColorTrain::Blue),
            "Yellow" => Ok(ColorTrain::Yellow),
            "White" => Ok(ColorTrain::White),
            "Black" => Ok(ColorTrain::Black),
            _ => Err(()),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    color: String
}

//Train color
fn main() -> Ev3Result<()>{
    move_motors()?;
    // let args = Args::parse();

    // let mut color_map = match csv_read() {
    //     Ok(map) => map,
    //     Err(_) =>  vec![vec![ColorTrain::None; V_SIZE]; H_SIZE]
    // };
    // let color_sensor = ColorSensor::find()?;
    // let button = Ev3Button::new()?;

    // color_sensor.set_mode_rgb_raw()?;

    // loop {
    //     let rgb = color_sensor.get_rgb()?;
    //     write_color(rgb)?;

    //     let rgb = (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
    //     let (h, _, v) = rgb_to_hsv(rgb);
       
    //     color_map[h][v] = ColorTrain::from_str(&args.color).unwrap();
    //     // color_map[h][v] = ColorTrain::Red;

    //     button.process();
    //     if button.is_right() {
    //         break;
    //     }
    // }

    // csv_write(color_map).unwrap();

    Ok(())
}

fn move_motors() -> Ev3Result<()>{
    let large_motor = LargeMotor::get(MotorPort::OutA)?;
    let pos = large_motor.get_position()?;

    let kp = large_motor.get_hold_pid_kp()?;
    let ki = large_motor.get_hold_pid_ki()?;
    let kd = large_motor.get_hold_pid_kd()?;

    println!("Pos: {}", pos);

    println!("KP: {}", kp);
    println!("KI: {}", ki);
    println!("KD: {}", kd);

    // thread::sleep(Duration::from_millis(2000));

    large_motor.run_to_rel_pos(Some(0))?;
    large_motor.wait_until_not_moving(None);

    // thread::sleep(Duration::from_millis(2000));

    large_motor.run_to_rel_pos(Some(90))?;
    large_motor.wait_until_not_moving(None);

    println!("Pos End: {}", pos);

    Ok(())
}

fn csv_read() -> Result<ColorMap, Box<dyn Error>> { 
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(CSV_FILE_PATH)?;

    let mut color_map = vec![vec![ColorTrain::None; V_SIZE]; H_SIZE];

    for (h, result) in rdr.records().enumerate() {
        let record = result?;
        
        let row: Vec<ColorTrain> = record.deserialize(None).unwrap();

        color_map[h] = row;
    }

    Ok(color_map)
}

fn csv_write(color_map: ColorMap) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(CSV_FILE_PATH)?;

    //loop over color_map
    for i in 0..color_map.len() {
        wtr.serialize(&color_map[i])?;
    }

    wtr.flush()?;
    Ok(())
}

fn rgb_to_hsv(rgb: RGB) -> HSV {
    let r = rgb.0 as f32 / 255.0;
    let g = rgb.1 as f32 / 255.0;
    let b = rgb.2 as f32 / 255.0;

    let max = f32::max(f32::max(r, g), b);
    let min = f32::min(f32::min(r, g), b);

    let h = if max == r && g >= b {
        60.0 * ((g - b) / (max - min))
    } else if max == r && g < b {
        60.0 * ((g - b) / (max - min)) + 360.0
    } else if max == g {
        60.0 * ((b - r) / (max - min)) + 120.0
    } else if max == b {
        60.0 * ((r - g) / (max - min)) + 240.0
    } else {
        0.0
    };

    let s = ((max - min) / max) * 100.0;
    let v = max * 100.0;

    println!("--------------");
    println!("{:?}", (h, s, v));
    println!("--------------");
    (h as usize, s as usize, v as usize)
}

fn write_color(rgb: (i32, i32, i32)) -> io::Result<()> {
    let (r, g, b) = rgb;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r as u8, g as u8, b as u8))))?;

    writeln!(&mut stdout, "{}, {}, {}", r as u8, g as u8, b as u8)
}extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::ColorSensor;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::error::Error;

use csv::WriterBuilder;
use csv::{ ReaderBuilder, StringRecord };
use serde::Serialize;
use serde::Deserialize;
use clap::Parser;
use std::str::FromStr;
// use std::{thread, time::Duration};

const CSV_FILE_PATH: &str = "foo.csv";
const H_SIZE: usize = 360;
const V_SIZE: usize = 100;

type ColorMap = Vec<Vec<ColorTrain>>;

type RGB = (u8, u8, u8);
type HSV = (usize, usize, usize);

#[derive(Deserialize, Serialize, Clone, Debug)]
enum ColorTrain {
    Red,
    Blue,
    Yellow,
    White,
    Black,
    None,
}

impl FromStr for ColorTrain {

    type Err = ();

    fn from_str(input: &str) -> Result<ColorTrain, Self::Err> {
        match input {
            "Red" => Ok(ColorTrain::Red),
            "Blue" => Ok(ColorTrain::Blue),
            "Yellow" => Ok(ColorTrain::Yellow),
            "White" => Ok(ColorTrain::White),
            "Black" => Ok(ColorTrain::Black),
            _ => Err(()),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    color: String
}

//Train color
fn main() -> Ev3Result<()>{
    move_motors()?;
    // let args = Args::parse();

    // let mut color_map = match csv_read() {
    //     Ok(map) => map,
    //     Err(_) =>  vec![vec![ColorTrain::None; V_SIZE]; H_SIZE]
    // };
    // let color_sensor = ColorSensor::find()?;
    // let button = Ev3Button::new()?;

    // color_sensor.set_mode_rgb_raw()?;

    // loop {
    //     let rgb = color_sensor.get_rgb()?;
    //     write_color(rgb)?;

    //     let rgb = (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
    //     let (h, _, v) = rgb_to_hsv(rgb);
       
    //     color_map[h][v] = ColorTrain::from_str(&args.color).unwrap();
    //     // color_map[h][v] = ColorTrain::Red;

    //     button.process();
    //     if button.is_right() {
    //         break;
    //     }
    // }

    // csv_write(color_map).unwrap();

    Ok(())
}

fn move_motors() -> Ev3Result<()>{
    let large_motor = LargeMotor::get(MotorPort::OutA)?;
    let pos = large_motor.get_position()?;

    let kp = large_motor.get_hold_pid_kp()?;
    let ki = large_motor.get_hold_pid_ki()?;
    let kd = large_motor.get_hold_pid_kd()?;

    println!("Pos: {}", pos);

    println!("KP: {}", kp);
    println!("KI: {}", ki);
    println!("KD: {}", kd);

    // thread::sleep(Duration::from_millis(2000));

    large_motor.run_to_rel_pos(Some(0))?;
    large_motor.wait_until_not_moving(None);

    // thread::sleep(Duration::from_millis(2000));

    large_motor.run_to_rel_pos(Some(90))?;
    large_motor.wait_until_not_moving(None);

    println!("Pos End: {}", pos);

    Ok(())
}

fn csv_read() -> Result<ColorMap, Box<dyn Error>> { 
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(CSV_FILE_PATH)?;

    let mut color_map = vec![vec![ColorTrain::None; V_SIZE]; H_SIZE];

    for (h, result) in rdr.records().enumerate() {
        let record = result?;
        
        let row: Vec<ColorTrain> = record.deserialize(None).unwrap();

        color_map[h] = row;
    }

    Ok(color_map)
}

fn csv_write(color_map: ColorMap) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(CSV_FILE_PATH)?;

    //loop over color_map
    for i in 0..color_map.len() {
        wtr.serialize(&color_map[i])?;
    }

    wtr.flush()?;
    Ok(())
}

fn rgb_to_hsv(rgb: RGB) -> HSV {
    let r = rgb.0 as f32 / 255.0;
    let g = rgb.1 as f32 / 255.0;
    let b = rgb.2 as f32 / 255.0;

    let max = f32::max(f32::max(r, g), b);
    let min = f32::min(f32::min(r, g), b);

    let h = if max == r && g >= b {
        60.0 * ((g - b) / (max - min))
    } else if max == r && g < b {
        60.0 * ((g - b) / (max - min)) + 360.0
    } else if max == g {
        60.0 * ((b - r) / (max - min)) + 120.0
    } else if max == b {
        60.0 * ((r - g) / (max - min)) + 240.0
    } else {
        0.0
    };

    let s = ((max - min) / max) * 100.0;
    let v = max * 100.0;

    println!("--------------");
    println!("{:?}", (h, s, v));
    println!("--------------");
    (h as usize, s as usize, v as usize)
}

fn write_color(rgb: (i32, i32, i32)) -> io::Result<()> {
    let (r, g, b) = rgb;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r as u8, g as u8, b as u8))))?;

    writeln!(&mut stdout, "{}, {}, {}", r as u8, g as u8, b as u8)
}extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::ColorSensor;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::error::Error;

use csv::WriterBuilder;
use csv::{ ReaderBuilder, StringRecord };
use serde::Serialize;
use serde::Deserialize;
use clap::Parser;
use std::str::FromStr;
// use std::{thread, time::Duration};

const CSV_FILE_PATH: &str = "foo.csv";
const H_SIZE: usize = 360;
const V_SIZE: usize = 100;

type ColorMap = Vec<Vec<ColorTrain>>;

type RGB = (u8, u8, u8);
type HSV = (usize, usize, usize);

#[derive(Deserialize, Serialize, Clone, Debug)]
enum ColorTrain {
    Red,
    Blue,
    Yellow,
    White,
    Black,
    None,
}

impl FromStr for ColorTrain {

    type Err = ();

    fn from_str(input: &str) -> Result<ColorTrain, Self::Err> {
        match input {
            "Red" => Ok(ColorTrain::Red),
            "Blue" => Ok(ColorTrain::Blue),
            "Yellow" => Ok(ColorTrain::Yellow),
            "White" => Ok(ColorTrain::White),
            "Black" => Ok(ColorTrain::Black),
            _ => Err(()),
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    color: String
}

//Train color
fn main() -> Ev3Result<()>{
    move_motors()?;
    // let args = Args::parse();

    // let mut color_map = match csv_read() {
    //     Ok(map) => map,
    //     Err(_) =>  vec![vec![ColorTrain::None; V_SIZE]; H_SIZE]
    // };
    // let color_sensor = ColorSensor::find()?;
    // let button = Ev3Button::new()?;

    // color_sensor.set_mode_rgb_raw()?;

    // loop {
    //     let rgb = color_sensor.get_rgb()?;
    //     write_color(rgb)?;

    //     let rgb = (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);
    //     let (h, _, v) = rgb_to_hsv(rgb);
       
    //     color_map[h][v] = ColorTrain::from_str(&args.color).unwrap();
    //     // color_map[h][v] = ColorTrain::Red;

    //     button.process();
    //     if button.is_right() {
    //         break;
    //     }
    // }

    // csv_write(color_map).unwrap();

    Ok(())
}

fn move_motors() -> Ev3Result<()>{
    let large_motor = LargeMotor::get(MotorPort::OutA)?;
    let pos = large_motor.get_position()?;

    let kp = large_motor.get_hold_pid_kp()?;
    let ki = large_motor.get_hold_pid_ki()?;
    let kd = large_motor.get_hold_pid_kd()?;

    println!("Pos: {}", pos);

    println!("KP: {}", kp);
    println!("KI: {}", ki);
    println!("KD: {}", kd);

    // thread::sleep(Duration::from_millis(2000));

    large_motor.run_to_rel_pos(Some(0))?;
    large_motor.wait_until_not_moving(None);

    // thread::sleep(Duration::from_millis(2000));

    large_motor.run_to_rel_pos(Some(90))?;
    large_motor.wait_until_not_moving(None);

    println!("Pos End: {}", pos);

    Ok(())
}

fn csv_read() -> Result<ColorMap, Box<dyn Error>> { 
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(CSV_FILE_PATH)?;

    let mut color_map = vec![vec![ColorTrain::None; V_SIZE]; H_SIZE];

    for (h, result) in rdr.records().enumerate() {
        let record = result?;
        
        let row: Vec<ColorTrain> = record.deserialize(None).unwrap();

        color_map[h] = row;
    }

    Ok(color_map)
}

fn csv_write(color_map: ColorMap) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(CSV_FILE_PATH)?;

    //loop over color_map
    for i in 0..color_map.len() {
        wtr.serialize(&color_map[i])?;
    }

    wtr.flush()?;
    Ok(())
}

fn rgb_to_hsv(rgb: RGB) -> HSV {
    let r = rgb.0 as f32 / 255.0;
    let g = rgb.1 as f32 / 255.0;
    let b = rgb.2 as f32 / 255.0;

    let max = f32::max(f32::max(r, g), b);
    let min = f32::min(f32::min(r, g), b);

    let h = if max == r && g >= b {
        60.0 * ((g - b) / (max - min))
    } else if max == r && g < b {
        60.0 * ((g - b) / (max - min)) + 360.0
    } else if max == g {
        60.0 * ((b - r) / (max - min)) + 120.0
    } else if max == b {
        60.0 * ((r - g) / (max - min)) + 240.0
    } else {
        0.0
    };

    let s = ((max - min) / max) * 100.0;
    let v = max * 100.0;

    println!("--------------");
    println!("{:?}", (h, s, v));
    println!("--------------");
    (h as usize, s as usize, v as usize)
}

fn write_color(rgb: (i32, i32, i32)) -> io::Result<()> {
    let (r, g, b) = rgb;
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r as u8, g as u8, b as u8))))?;

    writeln!(&mut stdout, "{}, {}, {}", r as u8, g as u8, b as u8)
}