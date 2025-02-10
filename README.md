# cnv - A Simple Unit Conversion CLI

`cnv` is a command-line tool for converting units of measurement, written in Rust. It is designed to be user-friendly.

## Installation

To install `cnv`, run:

```sh
cargo install cnv
```

Alternatively, you can clone the repository and build it manually:

```sh
git clone https://github.com/neerrrajj/cnv
cd cnv
cargo build --release
```

## Usage

Run the command with the following format:

```sh
cnv <measurement> <value> <from_unit> <to_unit>
```

### Example:

```sh
cnv dist 10 miles km
```

**Output:**

```
10 miles = 16.0934 km
```

### Supported Categories:

- `dist` (Distance) - Converts between different distance units.
- `weight` (Weight) - Converts between different weight units.
- `temp` (Temperature) - Converts between different temperature units.

_More categories will be supported in upcoming releases._

### Units:

- `--list`, `-L` - Lists down all the supported units for each category.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.
