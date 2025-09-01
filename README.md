# a440

tune to an A440

## Build

```sh
cargo [build|run]
```

## Usage

```sh
❯ a440 --help
Tune your damn instruments

Usage: a440 [OPTIONS]

Options:
  -r, --reference <REFERENCE>  Frequency of A4 in Hertz, must be within [20, 1000] Hz [default: 440]
  -o, --offset <OFFSET>        Offset in semitones, must be within [-12, 12] semitones [default: 0]
  -h, --help                   Print help
```

Example uses:

```sh
❯ a440 # plays a sine at 440 Hz
❯ a440 --reference 420 --offset -2 # tuning a guitar to D standard in Baroque tuning
❯ a440 -r 420 -o -2 # the same but in short form
```







