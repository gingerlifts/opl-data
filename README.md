# The OpenPowerlifting Project

[![Build Status](https://gitlab.com/openpowerlifting/opl-data/badges/master/pipeline.svg)](https://gitlab.com/openpowerlifting/opl-data/commits/master)

A permanent, accurate, convenient, accessible, open archive of the world's powerlifting data.<br/>
Presentation of this data is available at [OpenPowerlifting.org](https://www.openpowerlifting.org).

**Powerlifting to the People.**

## Development Chat

Project development is discussed in the [OpenPowerlifting Zulip Chat](https://openpl.zulipchat.com/). Everyone is welcome to join.

## Licensing

### Code Licensing

All OpenPowerlifting code is Free/Libre software under the GNU AGPLv3+.<br/>
Please refer to the LICENSE file.

### Data Licensing

OpenPowerlifting data (`*.csv`) under `meet-data/` is contributed to the public domain.

The OpenPowerlifting database contains facts that, in and of themselves,<br/>
are not protected by copyright law. However, the copyright laws of some jurisdictions<br/>
may cover database design and structure.

To the extent possible under law, all data (`*.csv`) in the `meet-data/` folder is waived</br>
of all copyright and related or neighboring rights. The work is published from the United States.

Although you are under no requirement to do so, if you incorporate OpenPowerlifting</br>
data into your project, please consider adding a statement of attribution</br>
so that people may know about this project and help contribute data.

Sample attribution text:

> This page uses data from the OpenPowerlifting project, https://www.openpowerlifting.org.<br/>
> You may download a copy of the data at https://gitlab.com/openpowerlifting/opl-data.

If you modify the data or add useful new data, please consider contributing<br/>
the changes back so the entire powerlifting community may benefit.

## Development Installation

### Fedora 31

Install dependencies:

```bash
sudo dnf install make npm python3-beautifulsoup4 python3-flake8 ansible parallel uglify-js
```

Install the "nightly" version of the Rust programming language using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When a menu appears, choose "Customize installation".<br/>
Press the Enter key until it asks `Default toolchain?`. Type `nightly` and press Enter.<br/>
Continue pressing Enter at the remaining prompts until Rust is installed.

Log out and log back in to allow `~/.cargo/bin` to be part of your default shell `$PATH`.

Build the project and run the server:

```bash
make
cd server
cargo run --release
```

### Ubuntu 20.04 LTS (Focal)

Follow the instructions for Fedora, but use this alternate command for installing dependencies:

```bash
sudo apt-get install curl make npm python3-bs4 python3-flake8 ansible parallel uglifyjs
```


### Windows 10 (Native)

1. Download and install the [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019).
    * When the installation menu appears, under the "Workloads" tab, select "C++ build tools" and press Install. A reboot will be required.

2. Install the [Rust language for Windows](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe).

    * When a menu appears, choose "Customize installation".
    * Press the Enter key until it asks `Default toolchain?`. Type `nightly` and press Enter.
    * Continue pressing Enter at the remaining prompts until Rust is installed.

3. To clone this repository locally, [install GitHub Desktop](https://desktop.github.com/). When given the option, select "Clone from URL" and enter `https://gitlab.com/openpowerlifting/opl-data.git` or the address to a personal fork.

4. In the Start Menu, open the Command Prompt.

    * Navigate to the repository directory. If you used GitHub Desktop, the command is `cd Documents\GitHub\opl-data`.
    * Run the checker: `cargo run --bin checker`.

### Docker

To run the server using Docker, simply build and run:

```
docker build -t opl .
docker run -p 8000:8000 opl
```

Access at http://localhost:8000/ per usual.
