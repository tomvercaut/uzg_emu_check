# emu_check

Library and commandline application to check the calculated MUs of an electron beam used in radiotherapy treatment planning.

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