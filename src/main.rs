use std::thread;
use tato::*;

pub fn main() {
    let args = parse_args();
    let mut iteration = 1;

    loop {
        println!("==> started block: {iteration}");
        thread::sleep(args.block_duration());

        println!("==> finished block: {iteration}");
        if iteration == args.iterations() {
            play_long_break_sound(&args);
            println!("==> long break started");
            thread::sleep(args.long_break_duration());
            play_long_break_sound(&args);
            println!("==> long break ended");
            iteration = 0;
        } else {
            play_break_sound(&args);
            println!("==> break started");
            thread::sleep(args.break_duration());
            play_break_sound(&args);
            println!("==> break ended");
        }

        iteration += 1;
    }
}
