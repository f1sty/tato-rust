use clap::Parser;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "Yet another pomodoro timer")]
pub struct Args {
    #[arg(long, value_parser = Self::from_minutes_str, help = "single session duration in minutes (default: 30)")]
    session: Option<Duration>,

    #[arg(long, value_parser = Self::from_minutes_str, help = "pause duration between sessions, in minutes (default: 5)")]
    pause: Option<Duration>,

    #[arg(long, help = "round (subsequent sessions) count (default: 4)")]
    round: Option<u32>,

    #[arg(long, value_parser = Self::from_minutes_str, help = "pause duration between rounds, in minutes (default: 15)")]
    long_pause: Option<Duration>,

    #[arg(long, help = "media-player used to play sounds (default: 'mpv')")]
    player: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "pause sound file path (default: 'sounds/ding.mp3')"
    )]
    pause_sound: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "long pause sound file path (default: 'sound/long_ding.mp3')"
    )]
    long_pause_sound: Option<PathBuf>,
}

impl Args {
    fn from_minutes_str(s: &str) -> Result<Duration, std::num::ParseIntError> {
        let minutes: u64 = s.parse()?;

        Ok(Duration::from_secs(minutes * 60))
    }

    pub fn session(&self) -> Duration {
        self.session.unwrap()
    }

    pub fn pause(&self) -> Duration {
        self.pause.unwrap()
    }

    pub fn round(&self) -> u32 {
        self.round.unwrap()
    }

    pub fn long_pause(&self) -> Duration {
        self.long_pause.unwrap()
    }

    pub fn player(&self) -> &PathBuf {
        self.player.as_ref().unwrap()
    }

    pub fn pause_sound(&self) -> &PathBuf {
        self.pause_sound.as_ref().unwrap()
    }
    pub fn long_pause_sound(&self) -> &PathBuf {
        self.long_pause_sound.as_ref().unwrap()
    }
}

pub fn parse_args() -> Args {
    let default_session: Option<Duration> = Some(Duration::from_secs(30 * 60));
    let default_pause: Option<Duration> = Some(Duration::from_secs(5 * 60));
    let default_round: Option<u32> = Some(4);
    let default_long_pause: Option<Duration> = Some(Duration::from_secs(15 * 60));
    let default_player: Option<PathBuf> = Some(PathBuf::from("mpv"));
    let default_pause_sound = Some(PathBuf::from("sounds/ding.mp3"));
    let default_long_pause_sound = Some(PathBuf::from("sounds/long_ding.mp3"));

    let mut args = Args::parse();

    args.session = args.session.or(default_session);
    args.pause = args.pause.or(default_pause);
    args.round = args.round.or(default_round);
    args.long_pause = args.long_pause.or(default_long_pause);
    args.player = args.player.or(default_player).to_owned();
    args.pause_sound = args.pause_sound.or(default_pause_sound).to_owned();
    args.long_pause_sound = args
        .long_pause_sound
        .or(default_long_pause_sound)
        .to_owned();
    args
}

pub fn play_pause_sound(args: &Args) {
    let _ = Command::new(args.player()).arg(args.pause_sound()).spawn();
}

pub fn play_long_pause_sound(args: &Args) {
    let _ = Command::new(args.player())
        .arg(&args.long_pause_sound())
        .spawn();
}
