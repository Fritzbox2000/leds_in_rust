use clap::{builder::styling::Color, Parser};
use color_art::Color as ColorConverter;
use regex::Regex;
use std::{
    io::BufReader,
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread, time,
};
use ws2818_rgb_led_spi_driver::adapter_gen::WS28xxAdapter;
use ws2818_rgb_led_spi_driver::adapter_spi::WS28xxSpiAdapter;
use ws2818_rgb_led_spi_driver::encoding::encode_rgb;

enum LedProgram {
    Trans,
    TransTwo,
    Off,
    Breathe,
}

#[doc = r"Simple program to run my LEDs"]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time between updating lights in ms
    #[arg(short, long)]
    time: u64,

    /// Brightness of the leds
    #[arg(short, long, default_value = "255")]
    brightness: u8,

    /// Number of leds
    #[arg(short, long, default_value = "15")]
    leds: usize,

    /// Setting for the led strip, current settings are: trans, trans2, breathe, off
    #[arg(short = 's', long = "setting", default_value = "trans")]
    setting: String,
}

fn extract_numbers(input: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect()
}

fn breath_colours(mut adapter: WS28xxSpiAdapter, args: Args) {
    let mut counter: f64 = 0.0;
    loop {
        let mut spi_bits = vec![];
        // let's work out the colour we want
        let out_colour = ColorConverter::from_hsv(counter, 0.7, 0.7).unwrap();
        // now calculate the rgb
        let out_colour = out_colour.rgb();
        // now we should run a function to turn the string into useful numbers
        let rgb = extract_numbers(&out_colour);
        println!("{:?}", rgb);
        for i in 0..args.leds {
            let r: u8 = rgb[0] as u8;
            let g: u8 = rgb[1] as u8;
            let b: u8 = rgb[2] as u8;
            spi_bits.extend_from_slice(&encode_rgb(r, g, b))
        }
        counter += 1.0;
        if counter == 360.0 {
            counter = 0.0
        }
    }
}

fn cycle_n_colours(mut adapter: WS28xxSpiAdapter, args: Args, colour_list: Vec<[u8; 48]>) {
    let mut counter: usize = 0;
    loop {
        let mut spi_bits = vec![];
        let colour_number: usize = colour_list.len();
        for i in 0..args.leds {
            spi_bits.extend_from_slice(&colour_list[(i + counter) % colour_number])
        }
        counter += 1;
        if (counter == args.leds) {
            counter = 0
        }
        adapter.write_encoded_rgb(&spi_bits).unwrap();
        thread::sleep(time::Duration::from_millis(args.time))
    }
}

fn trans_colours_basic(mut adapter: WS28xxSpiAdapter, args: Args) {
    let trans: Vec<[u8; 48]> = vec![
        encode_rgb(91, 206, 250),
        encode_rgb(245, 169, 184),
        encode_rgb(255, 255, 255),
    ];
    cycle_n_colours(adapter, args, trans);
}

fn trans_colours_two(mut adapter: WS28xxSpiAdapter, args: Args) {
    let trans: Vec<[u8; 48]> = vec![
        encode_rgb(91, 206, 250),
        encode_rgb(245, 169, 184),
        encode_rgb(255, 255, 255),
        encode_rgb(245, 169, 184),
        encode_rgb(91, 206, 250),
    ];
    cycle_n_colours(adapter, args, trans)
}

fn turn_off() {
    unimplemented!();
}
fn control_lights(args: Args, program: LedProgram) {
    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    match program {
        LedProgram::Trans => trans_colours_basic(adapter, args),
        LedProgram::Off => turn_off(),
        LedProgram::TransTwo => trans_colours_two(adapter, args),
        LedProgram::Breathe => breath_colours(adapter, args),
    }
}

fn main() {
    let args = Args::parse();
    let program = match &args.setting[..] {
        "trans" => LedProgram::Trans,
        "trans2" => LedProgram::TransTwo,
        "breathe" => LedProgram::Breathe,
        "off" => LedProgram::Off,
        _ => LedProgram::Off,
    };
    control_lights(args, program)
}
