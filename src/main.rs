use std::thread;
use tato::*;

pub fn main() {
    let args = parse_args();
    let mut rounds_count = 0;

    loop {
        println!("session starting ->");
        thread::sleep(args.session());
        println!("session ended    <-");

        rounds_count += 1;

        if rounds_count == args.round() {
            println!("long pause starting ->");
            thread::sleep(args.long_pause());
            println!("long pause ended    <-");
            rounds_count = 0;
        } else {
            println!("pause starting ->");
            thread::sleep(args.pause());
            println!("pause ended    <-");
        }
    }
}
