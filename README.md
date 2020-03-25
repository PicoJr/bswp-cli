# BSWP-CLI

Another _unpronounceable_ tool.

Swap bytes using patterns and masks.

## Usage

```
USAGE:
    bswp-cli [OPTIONS] -e <pattern>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <input>             input path (if not provided STDIN is used instead)
    -o <output>            output path (if not provided STDOUT is used instead)
    -e <pattern>...        pattern: <value>,<mask>,<periodicity>,<offset>
```

Replace every other bytes by `0x42` (displayed as `B`).

```
echo -n "AAAA" | bswp-cli -e 0x42,0xFF,2,0
```

result:

```
BABA
```

Replace every other 4 bytes groups by `0x52555354` (displayed as `RUST`)

```
echo -n "AAAABBBBCCCCDDDD" | bswp-cli -e 0x52,0xFF,8,0 -e 0x55,0xFF,8,1 -e 0x53,0xFF,8,2 -e 0x54,0xFF,8,3
```

result:

```
RUSTBBBBRUSTDDDD
```

## Tests

Run tests with `cargo test`.

## Build From Source

Clone and build from source:
```
git clone https://github.com/PicoJr/bswp-cli.git
cd bswp-cli
cargo build --release
```

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.