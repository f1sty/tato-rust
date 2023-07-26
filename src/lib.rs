use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "Yet another pomodoro timer")]
pub struct Args {
    #[arg(short = 'd', long, value_parser = Self::from_minutes_str, help = "single block duration in minutes (default: 35)")]
    block_duration: Option<Duration>,

    #[arg(short, long, value_parser = Self::from_minutes_str, help = "break duration between blocks in minutes (default: 5)")]
    break_duration: Option<Duration>,

    #[arg(short, long, value_parser = Self::from_minutes_str, help = "long break duration in minutes (default: 15)")]
    long_break_duration: Option<Duration>,

    #[arg(
        short,
        long,
        help = "how much blocks to do before long break (default: 3)"
    )]
    iterations: Option<u32>,

    #[arg(
        short,
        long,
        help = "media-player to use to play sounds (default: 'mpv')"
    )]
    player: Option<PathBuf>,

    #[arg(
        short = 's',
        long,
        help = "'break sound' file path (default: '$XDG_CONFIG_HOME/tato/sounds/ding.mp3')"
    )]
    break_sound: Option<PathBuf>,

    #[arg(
        short = 'S',
        long,
        help = "'long break sound' file path (default: '$XDG_CONFIG_HOME/tato/sounds/long_ding.mp3')"
    )]
    long_break_sound: Option<PathBuf>,
}

impl Args {
    fn from_minutes_str(s: &str) -> Result<Duration, std::num::ParseIntError> {
        let minutes: u64 = s.parse()?;

        Ok(Duration::from_secs(minutes * 60))
    }

    pub fn block_duration(&self) -> Duration {
        self.block_duration.unwrap()
    }

    pub fn break_duration(&self) -> Duration {
        self.break_duration.unwrap()
    }

    pub fn iterations(&self) -> u32 {
        self.iterations.unwrap()
    }

    pub fn long_break_duration(&self) -> Duration {
        self.long_break_duration.unwrap()
    }

    pub fn player(&self) -> &PathBuf {
        self.player.as_ref().unwrap()
    }

    pub fn break_sound(&self) -> &PathBuf {
        self.break_sound.as_ref().unwrap()
    }
    pub fn long_break_sound(&self) -> &PathBuf {
        self.long_break_sound.as_ref().unwrap()
    }
}

pub fn parse_args() -> Args {
    let default_block_duration: Option<Duration> = Some(Duration::from_secs(35 * 60));
    let default_break_duration: Option<Duration> = Some(Duration::from_secs(5 * 60));
    let default_iterations: Option<u32> = Some(3);
    let default_long_break_duration: Option<Duration> = Some(Duration::from_secs(15 * 60));
    let default_player: Option<PathBuf> = Some(PathBuf::from("mpv"));

    let xdg_config_home = match env::var("XDG_CONFIG_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => PathBuf::from("~/.config"),
    };

    let default_break_sound = Some(xdg_config_home.join(PathBuf::from("tato/sounds/ding.mp3")));
    let default_long_break_sound =
        Some(xdg_config_home.join(PathBuf::from("tato/sounds/long_ding.mp3")));

    let mut args = Args::parse();

    args.block_duration = args.block_duration.or(default_block_duration);
    args.break_duration = args.break_duration.or(default_break_duration);
    args.iterations = args.iterations.or(default_iterations);
    args.long_break_duration = args.long_break_duration.or(default_long_break_duration);
    args.player = args.player.or(default_player).to_owned();
    args.break_sound = args.break_sound.or(default_break_sound).to_owned();
    args.long_break_sound = args
        .long_break_sound
        .or(default_long_break_sound)
        .to_owned();
    args
}

pub fn play_break_sound(args: &Args) {
    let _ = Command::new(args.player()).arg(args.break_sound()).spawn();
}

pub fn play_long_break_sound(args: &Args) {
    let _ = Command::new(args.player())
        .arg(&args.long_break_sound())
        .spawn();
}
