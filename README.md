# Term Color Support

## Description
Determine whether a terminal supports color or not, and if it supports color, identify the level of color support available.

## Features
- Detect if the terminal supports color.
- Identify the level of color support (e.g., no color, basic 16 colors, 256 colors, true color).

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (Ensure you have Rust and Cargo installed)

### Adding as a Dependency
To use this package, add the following to your `Cargo.toml`:

```toml
[dependencies]
term_color_support = "0.1.0"  # Replace with the latest version
```

## Usage
```
use term_color_support::ColorSupport;
fn main() {
    // Detect and print color support for stdout
    println!("Color support for stdout: {:?}", ColorSupport::stdout());

    // Detect and print color support for stderr
    println!("Color support for stderr: {:?}", ColorSupport::stderr());
}
```

The output of the above code will be something like this:

```
Color support for stdout: ColorInfo { level: TrueColor, has_basic: true, has_256: true, has_16m: true }
Color support for stderr: ColorInfo { level: TrueColor, has_basic: true, has_256: true, has_16m: true }
```

## API

### Structs

#### `ColorInfo`

```rust
pub struct ColorInfo {
    /// The color support level.
    pub level: ColorSupportLevel,
    /// Indicates if basic color support is available.
    pub has_basic: bool,
    /// Indicates if 256-color support is available.
    pub has_256: bool,
    /// Indicates if true color support (16 million colors) is available.
    pub has_16m: bool,
}
```

#### `ColorSupportLevel`

```rust
pub enum ColorSupportLevel {
    /// No color support.
    NoColor,
    /// Basic color support.
    Basic,
    /// Support for 256 colors.
    Colors256,
    /// True color support.
    TrueColor,
}

```

## Examples

For detailed examples of how to use the package, including more complex use cases and scenarios, you can refer to the [`main.rs`](src/bin/main.rs) file in the `src/bin` directory. A simple example is provided there.

## Contributing

Guidelines for contributing to Term Color Support:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature-name`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature/your-feature-name`).
6. Open a Pull Request.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

Term Color Support was inspired by the npm package [supports-color](https://www.npmjs.com/package/supports-color), which is used to detect whether a terminal supports color. The concepts of Term Color Support are influenced by supports-color.

We are thankful to the maintainers of supports-color, [Sindre Sorhus](https://github.com/sindresorhus) and [Josh Junon](https://github.com/qix-), for their contribution to the open-source community.

## Contact

For support or questions, you can contact me via:

- Email: [itsmenirajpaudel@gmail.com](mailto:itsmenirajpaudel@gmail.com)
- LinkedIn: [itsmenirajpaudel](https://www.linkedin.com/in/itsmenirajpaudel/)
- Website: [https://nirajpaudel.me](https://nirajpaudel.me)




 
