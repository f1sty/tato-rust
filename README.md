# Description

Yet another **pomodoro** timer CLI tool.

# Help

```
Usage: tato [OPTIONS]

Options:
  -d, --block-duration <BLOCK_DURATION>
          single block duration in minutes (default: 30)
  -b, --break-duration <BREAK_DURATION>
          break duration between blocks in minutes (default: 5)
  -l, --long-break-duration <LONG_BREAK_DURATION>
          long break duration in minutes (default: 15)
  -i, --iterations <ITERATIONS>
          how much blocks to do before long break (default: 4)
  -p, --player <PLAYER>
          media-player to use to play sounds (default: 'mpv')
  -s, --break-sound <BREAK_SOUND>
          'break sound' file path (default: '$XDG_CONFIG_HOME/tato/sounds/ding.mp3')
  -S, --long-break-sound <LONG_BREAK_SOUND>
          'long break sound' file path (default: '$XDG_CONFIG_HOME/tato/sounds/long_ding.mp3')
  -h, --help
          Print help
  -V, --version
          Print version
```
