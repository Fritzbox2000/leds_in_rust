# leds_in_rust

getting my LEDs to work using rust on a raspberry pi (version 3 B v1.2)

There's a lot todo but things are moving along

currently it takes 4 command line arguments:
-t: u64 - the time between updates in ms
-b: u8 - the general brightness of the leds
-l: usize - the number of leds (defaults to 15)
-s : String - The light show it will produce settings are as follows

1 -  trans - cycle through the trans colours
2 - trans2 - cycle through the trans colours which should look like the flag more
3 - breathe - Loop over all colours going around a hsv circle
4 -
5 - off - turns the leds off

Some little notes for me :

I'm using a very custom ws2818 driver, written by my friend, it actually cross
compiles for my 32bit arm raspberry pi 1 b, which the old one I was using didn't
so that's nice, means I can make pull requests to it if I need to add functionality
and might suggest wider functionality anyway (in theory it should actually become
no-std capable as well since it was advertised as such but just wasn't before)

I was using the color-art crate before, but after testing turns out that it doesn't
work with arm cpu's or something, or cross compiling, anyway I moved the code that
wasn't working out to a test script and it still didn't work, kinda weird might
be worth looking into

Now I have to deal with the strip not seeming to work correctly. I don't know if
it is spi doing something weird, currently the first ~10 LEDs flash odd colours
and don't seem to always take commands, on top of that *ALL* the LEDs are the wrong
colours and they are inconsistent as to what they change to (actually it seems to
be almost on a loop, maybe this has something to do with clock cycles or the spi
bufsize which I had to increase). This might be a problem with the led strip and
some faulty connections. Although I'm worried it's two problems, the incorrect colours
and the flashing across the first ~10 controllers (30 lights)

15 LEDs works fine which suggests that it is either a problem with sending 100 leds
data, although if I make it think it has 100 leds it doesn't seem to have a problem
which really seems to suggest it's the strip. this would suck
