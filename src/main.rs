use std::thread;
use tato::*;

pub fn main() {
    let args = parse_args();
    let mut rounds_count = 0;

    loop {
        thread::sleep(args.session());

        rounds_count += 1;

        println!("==> finished session: {rounds_count}");
        if rounds_count == args.round() {
            play_long_pause_sound(&args);
            println!("==> long pause started");
            thread::sleep(args.long_pause());
            play_long_pause_sound(&args);
            println!("==> long pause ended");
            rounds_count = 0;
        } else {
            play_pause_sound(&args);
            println!("==> pause started");
            thread::sleep(args.pause());
            play_pause_sound(&args);
            println!("==> pause ended");
        }
    }
}
