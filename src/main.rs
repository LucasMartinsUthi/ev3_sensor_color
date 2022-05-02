extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{ Ev3Button, Ev3Result };
use ev3dev_lang_rust::sensors::ColorSensor;

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::error::Error;

use rand::Rng;
use csv::WriterBuilder;
use csv::ReaderBuilder;
use serde::Serialize;

const CSV_FILE_PATH: &str = "foo.csv";
const H_SIZE: usize = 360;
const V_SIZE: usize = 100;

type ColorMap = Vec<Vec<ColorTrain>>;

type RGB = (u8, u8, u8);
type HSV = (usize, usize, usize);

#[derive(Serialize, Clone)]
enum ColorTrain {
    Red,
    Blue,
    Yellow,
    White,
    Black,
    None,
}

fn main() -> Ev3Result<()>{
    let mut color_map = vec![vec![ColorTrain::None; H_SIZE]; V_SIZE];

    let color_sensor = ColorSensor::find()?;
    let button = Ev3Button::new()?;

    color_sensor.set_mode_rgb_raw()?;

    loop {
        let rgb = color_sensor.get_rgb()?;
        write_color(rgb)?;

        let rgb = (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8);

        let (h, _, v) = rgb_to_hsv(rgb);
       
        color_map[h][v] = ColorTrain::Red;

        button.process();
        if button.is_right() {
            break;
        }
    }

    csv_write(color_map).unwrap();

    Ok(())
}

fn csv_read() -> Result<(), Box<dyn Error>> {
    // let mut rdr = ReaderBuilder::new()
    //     .delimiter(b'\t')
    //     .has_headers(false)
    //     .from_path(CSV_FILE_PATH)?;

    // for result in rdr.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    Ok(())
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

// fn get_color_map<'a>() -> &'a ColorMap {
//     // let color_map = vec![vec![Color::None; H_SIZE]; V_SIZE];

//     //csv_read

//     // for i in 0..color_map.len() {
//     //     for j in 0..color_map[i].len() {
//     //         color_map[i][j] = Color::Red;
//     //     }
//     // }

//     &'a color_map
// }



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
