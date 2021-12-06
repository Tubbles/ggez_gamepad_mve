# Minimum viable example for loading lag using gamepad

    monkey@mandes19 Manjaro_Linux ~/dev/ggez_gamepad_mve (main)
    $ cargo run --release
       Compiling remusys v0.1.0 (/home/monkey/dev/ggez_gamepad_mve)
        Finished release [optimized] target(s) in 2.26s
         Running `target/release/remusys`
    Time to first loop: 0.19670273 s
    [0] 211206-160855

After connecting the wireless 8BitDo SN30 pro controller:

    monkey@mandes19 Manjaro_Linux ~/dev/ggez_gamepad_mve (main)
    $ cargo run --release
        Finished release [optimized] target(s) in 0.07s
         Running `target/release/remusys`
    Time to first loop: 35.38699 s
    [0] 211206-161001

That is a 180 times longer startup..
