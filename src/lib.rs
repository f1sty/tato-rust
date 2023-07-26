use clap::Parser;
use std::time::Duration;

const DEFAULT_SESSION: Option<Duration> = Some(Duration::from_secs(30 * 60));
const DEFAULT_PAUSE: Option<Duration> = Some(Duration::from_secs(5 * 60));
const DEFAULT_ROUND: Option<u32> = Some(4);
const DEFAULT_LONG_PAUSE: Option<Duration> = Some(Duration::from_secs(15 * 60));

#[derive(Parser, Debug)]
#[command(author, version, about = "Yet another pomodoro timer")]
pub struct Args {
    #[arg(short, long, value_parser = from_minutes_str, help = "single session duration in minutes (default: 30)")]
    pub session: Option<Duration>,

    #[arg(short, long, value_parser = from_minutes_str, help = "pause duration between sessions, in minutes (default: 5)")]
    pub pause: Option<Duration>,

    #[arg(short, long, help = "round (subsequent sessions) count (default: 4)")]
    pub round: Option<u32>,

    #[arg(short, long, value_parser = from_minutes_str, help = "pause duration between rounds, in minutes (default: 15)")]
    pub long_pause: Option<Duration>,
}

fn from_minutes_str(s: &str) -> Result<Duration, std::num::ParseIntError> {
    let minutes: u64 = s.parse()?;

    Ok(Duration::from_secs(minutes * 60))
}

pub fn parse_args() -> Args {
    let mut args = Args::parse();
    args.session = args.session.or(DEFAULT_SESSION);
    args.pause = args.pause.or(DEFAULT_PAUSE);
    args.round = args.round.or(DEFAULT_ROUND);
    args.long_pause = args.long_pause.or(DEFAULT_LONG_PAUSE);
    args
}
