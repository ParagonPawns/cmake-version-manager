# CMake Version Manager
CMake Version Manager (`cvm`) is a command line tool that help manage currently
installed versions of cmake.

## Install
To install `cvm` you will need to have cargo. Installing Rust will provide
cargo tools. Visit https://www.rust-lang.org/tools/install for installation
steps. Once installed you can run `cargo install cmake-version-manager`.

## Examples
Below are some visual examples on how to use `cvm`. But keep in mind that at any
point you can use `cvm --help` to find a list of commands that can be used.
### Install
```sh
$ cvm install 3.20.2

# For interactive mode
$ cvm install
```

### Switch
```sh
$ cvm switch 3.20.1

# For interactive mode to choose through installed versions
$ cvm switch
```

### Remove
```sh
$ cvm remove 3.20.2

# For interactive mode to remove currently installed version
```

### List
```sh
# Lists the latest 10 releases
$ cvm list

# Lists all latests releases (max 100)
$ cvm list --all
```

## Supported Platforms
 * Linux
 * OSX (WIP)
 * Windows (N/A but will be done asap)
