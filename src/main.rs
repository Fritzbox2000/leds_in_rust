use clap::Parser;
use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};
use std::{thread, time};

/// Simple progam to run my LEDs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Time between updating lights in ms
    #[arg(short, long)]
    time: u64,

    /// Brightness of the leds
    #[arg(short, long, default_value = "255")]
    brightness: u8,

    /// If the leds strip should be turned off
    #[arg(
        short = 'o',
        long = "off",
        default_value = "false",
        default_missing_value = "true"
    )]
    turn_off: bool,
}

fn maker(pin: i32, led_count: i32, brightness: Option<u8>) -> Controller {
    ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(pin)
                .count(led_count)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(brightness.unwrap_or(255))
                .build(),
        )
        .build()
        .unwrap()
}
// it's BRG

fn trans_colours_basic(mut controller: Controller, time: u64) -> ! {
    let mut counter: usize = 0;
    let mut modifier: usize = 0;
    let trans: [[u8; 4]; 3] = [[250, 91, 206, 0], [184, 245, 169, 0], [255, 255, 255, 0]];
    loop {
        let leds = controller.leds_mut(0);
        leds[(counter + modifier) % leds.len()] = trans[counter % 3];
        counter += 1;
        if counter % leds.len() == 0 {
            modifier += 1;
        }
        controller.render();

        thread::sleep(time::Duration::from_millis(time));
    }
}

fn turn_off(mut controller: Controller) {
    let off: [u8; 4] = [0, 0, 0, 0];
    (0..controller.leds(0).len()).for_each(|led| {
        let leds = controller.leds_mut(0);
        leds[led] = off;
    });
    controller.render();
    std::process::exit(0x0001);
}

fn main() {
    let args = Args::parse();

    let controller = maker(18, 15, Some(args.brightness));
    if args.turn_off {
        turn_off(controller);
    } else {
        trans_colours_basic(controller, args.time);
    }
}
