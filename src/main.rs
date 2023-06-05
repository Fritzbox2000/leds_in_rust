use rs_ws281x

fn main() {
    let controller = rs_ws281x::ControllerBuilder::new().freq(800_000).dma(10).channel(ChannelBuilder::new().pin(18).counter(10).strip_type(StripType::WS2811Rgb).brightness(255).build()).build();
    let leds = controller.leds_mut(0);
    leds[0] = [255,255,255,0];
    controller.render();
}
