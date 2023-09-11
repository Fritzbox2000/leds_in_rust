use clap::Parser;
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
/// hsv_to_rgb takes hsv values and converts to an rgb tuple in the range of 0..255
/// # Arguments
///
/// * hue - an f32 hue (generally best to be clamped between 0 and 360)
/// * saturation - an f32 of how saturated the colour is (best kept between 0 and 1)
/// * value - similar to saturation (best kept between 0 and 1)
///
/// # Example
/// ```
/// use rust_leds::hsv_to_rgb
/// let (r,g,b) = hsv_to_rgb(37.5, 1,0.7);
/// ```
///
/// This function should work for 32 bit arm cpu's, the % operator is often used in
/// this calculation, the problem being that that operation doesn't work arm 32 bit,
/// so this function has been constructed around it missing. If for whatever reason
/// you decide to not use a raspberry pi 1 b (crazy as that might be ;) ) then you
/// can skip using this function and use something like ecolor's rgb_from_hsv(), it's
/// almost certainly faster. Though this function is pretty performant, and the cycle
/// looks good enough for me so! who cares?!
pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let chroma = value * saturation;
    let h = hue / 60.0;

    let mut r1 = 0.0;
    let mut g1 = 0.0;
    let mut b1 = 0.0;

    if h >= 0.0 && h < 1.0 {
        r1 = chroma;
        g1 = chroma * h;
    } else if h >= 1.0 && h < 2.0 {
        r1 = chroma * (2.0 - h);
        g1 = chroma;
    } else if h >= 2.0 && h < 3.0 {
        g1 = chroma;
        b1 = chroma * (h - 2.0);
    } else if h >= 3.0 && h < 4.0 {
        g1 = chroma * (4.0 - h);
        b1 = chroma;
    } else if h >= 4.0 && h < 5.0 {
        r1 = chroma * (h - 4.0);
        b1 = chroma;
    } else {
        r1 = chroma;
        b1 = chroma * (6.0 - h);
    }

    let m = value - chroma;
    let (r, g, b) = ((r1 + m) * 255.0, (g1 + m) * 255.0, (b1 + m) * 255.0);

    (r as u8, g as u8, b as u8)
}

fn breathe_colours(mut adapter: WS28xxSpiAdapter, args: Args) -> ! {
    let mut counter: f32 = 0.0;
    loop {
        let mut spi_bits = vec![];
        // let's create the colour
        let (r, g, b) = hsv_to_rgb(counter, 1.0, 1.0); //rgb_from_hsv((counter, 0.9, 0.9));
                                                       //rgb.iter_mut().for_each(|x| *x *= 255.0);
        println!("{}", counter);
        println!("{},{},{}", r, g, b);
        //println!("{:?}", rgb);
        // lets turn that colour into rgb
        // now we should run a function to turn the string into useful numbers
        (0..args.leds)
            .for_each(|_| spi_bits.extend_from_slice(&encode_rgb(r as u8, g as u8, b as u8)));
        counter += 1.0;
        if counter == 360.0 {
            counter = 0.0
        }
        adapter.write_encoded_rgb(&spi_bits).unwrap();
        thread::sleep(time::Duration::from_millis(args.time));
    }
}
/// Cycles the leds in the strip over a set colour list.
///
/// # Arguments
///
/// * adapter - The WS28xxSpiAdapter the core of the operation
/// * args - the arguments taken from the command line (generally for getting the number of leds)
/// * colour_list - A Vector of u8's which holds the commands to be sent to the adapter
///
/// I use this function a bit (good examples are `trans_colours_basic()` and `trans_colours_two()` )
fn cycle_n_colours(mut adapter: WS28xxSpiAdapter, args: Args, colour_list: Vec<[u8; 48]>) {
    let mut counter: usize = 0;
    loop {
        let mut spi_bits = vec![];
        let colour_number: usize = colour_list.len();
        for i in 0..args.leds {
            spi_bits.extend_from_slice(&colour_list[(i + counter) % colour_number])
        }
        counter += 1;
        if counter == args.leds {
            counter = 0
        }
        adapter.write_encoded_rgb(&spi_bits).unwrap();
        thread::sleep(time::Duration::from_millis(args.time))
    }
}

fn trans_colours_basic(adapter: WS28xxSpiAdapter, args: Args) {
    let trans: Vec<[u8; 48]> = vec![
        encode_rgb(91, 206, 250),
        encode_rgb(245, 169, 184),
        encode_rgb(255, 255, 255),
    ];
    cycle_n_colours(adapter, args, trans);
}

fn trans_colours_two(adapter: WS28xxSpiAdapter, args: Args) {
    let trans: Vec<[u8; 48]> = vec![
        encode_rgb(91, 206, 250),
        encode_rgb(245, 169, 184),
        encode_rgb(255, 255, 255),
        encode_rgb(245, 169, 184),
        encode_rgb(91, 206, 250),
    ];
    cycle_n_colours(adapter, args, trans)
}

fn set_colour(mut adapter: WS28xxSpiAdapter, args: Args, colour: [u8; 3]) {
    let mut spi_bits = vec![];
    for _ in 0..args.leds {
        spi_bits.extend_from_slice(&encode_rgb(colour[0], colour[1], colour[2]))
    }
    adapter.write_encoded_rgb(&spi_bits).unwrap();
}

fn turn_off(adapter: WS28xxSpiAdapter, args: Args) {
    set_colour(adapter, args, [0, 0, 0]);
}

fn control_lights(args: Args, program: LedProgram) {
    let adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").unwrap();

    match program {
        LedProgram::Trans => trans_colours_basic(adapter, args),
        LedProgram::Off => turn_off(adapter, args),
        LedProgram::TransTwo => trans_colours_two(adapter, args),
        LedProgram::Breathe => breathe_colours(adapter, args),
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
