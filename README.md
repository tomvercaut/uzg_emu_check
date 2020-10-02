# emu_check

![Travis (.org) branch](https://img.shields.io/travis/tomvercaut/uzg_emu_check/async_impl?style=flat-square)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square)](https://opensource.org/licenses/Apache-2.0)


Library and command line application to check the calculated MUs of an electron beam used in radiotherapy treatment planning.

## Description
The project was created to eliminate the manual calculations performed during the MU verification of an electron beam treatment plan. The applied method is unlikely to fit the requirements for your radiotherapy site and should not be considered as a general tool to verify electron treatment plans.

## Usage
```
USAGE:
    emu_check.exe [dir]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <dir>    Directory containing the outputfactors and field defining apertures per energy. Each applicator has a
             seperate csv file for the output factors and field defining apertures. 
```

## License
`emu_check` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository for more information.