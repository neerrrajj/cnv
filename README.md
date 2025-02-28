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
- `ds` (Data Storage) - Converts between different data storage units.
- `dt` (Data Transfer) - Converts between different data transfer units.
- `time` (Time) - Converts between different time units.
- `volume` (Volume) - Converts between different volume units.
- `area` (Area) - Converts between different area units.
- `freq` (Frequency) - Converts between different frequency units.
- `energy` (Energy) - Converts between different energy units.
- `force` (Force) - Converts between different force units.
- `power` (Power) - Converts between different power units.
- `speed` (Speed) - Converts between different speed units.
- `currency` (Currency) - Converts between different currencies.

### Units:

- `--list`, `-L` - Lists down all the supported units for each category.
- Usage: `cnv <COMMAND> --list`

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.
