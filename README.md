# Gifer
Utility for creating web-friendly gifs

Very much in alpha and for my own usage.

## Usage
```
Utility for creating web-friendly gifs

USAGE:
    gifer --input <INFILE> <OUTFILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INFILE>    Sets the input file to use

ARGS:
    <OUTFILE>    Sets the output file to write
```

### Example
```bash
gifer -i some_video.mp4 to_share.gif 
```

## Development
### Build, Test
```bash
cargo build
```

```
cargo run -- --help
```

## TODO
- tunable file size, quality
- simplify code
- fix arg ordering in  documentation (bug with clap crate)
- progress based on FFmpeg output
- write palette to unique location so concurrent execution is possible

## License
Copyright © Stevan Dedovic
Distributed under the MIT License.