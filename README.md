# Unifi Search Tool - Download [Here](https://github.com/Crypto-Spartan/unifi-search-tool/releases/latest)

Does your unifi controller have lots of sites? Do you frequently have equipment returned from those sites and you can't remember where it's adopted in the controller? Enter Unifi Search Tool.

This tool was designed to allow searching a MAC address from a Unifi device across **_all_** of the different sites in a Unifi controller. Currently, the search feature in Unifi only works _within_ a site, there's no way to search across multiple sites.

## How to Use

![examplev2](https://raw.githubusercontent.com/Crypto-Spartan/unifi-search-tool/main/screenshots/examplev2.png "examplev2")

1. Enter your username & password for your Unifi Controller

2. Enter your Unifi Controller domain/IP. You must include the proper http:// or https:// with the appropriate port number at the end, unless it runs on 80/443. (You will see this in the address bar of your browser when you open up your Unifi Controller.)

3. Enter the MAC Address of the device you're searching for

4. Click search

5. Profit

The tool will tell you which site in the controller that the device was adopted to. If it hasn't been adopted, the tool will tell you that the device could not be found.

## Build From Source

> [!IMPORTANT]
> Rust **nightly** compiler is required

![Minimum Rust Version](https://img.shields.io/badge/Minimum%20Rust%20Version-1.81.0--nightly_(2024--07--01)-brightgreen.svg)

1. Download the Zip of the source files and extract it

2. Open up a terminal in the directory

3. Run `cargo build --release` in the terminal

4. Go to the `target/release` directory to find the unifi-search-tool.exe file

### If you would like to optimize the binary for size

1. Install the appropriate toolchain and the rust-src component
```bash
$ rustup toolchain install nightly
$ rustup component add rust-src --toolchain nightly
```
2. Find your host's target triple
```bash 
$ rustc -vV
...
host: x86_64-unknown-linux-gnu
```
3. Run the build command
```bash
# Use that target triple when building with build-std.
# Add the =std,panic_abort to the option to make panic = "abort" Cargo.toml option work.
# See: https://github.com/rust-lang/wg-cargo-std-aware/issues/56
$ cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release
```

- see <https://github.com/johnthagen/min-sized-rust> for more details

## Star History

<a href="https://star-history.com/#crypto-spartan/unifi-search-tool&Date">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=crypto-spartan/unifi-search-tool&type=Date&theme=dark" />
        <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=crypto-spartan/unifi-search-tool&type=Date" />
        <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=crypto-spartan/unifi-search-tool&type=Date" />
    </picture>
</a>
