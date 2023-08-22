pub fn dislay_help() {
    println!(
        r"CMake version manager 'cvm'.
Usage:
    cvm [OPTIONS]

OPTIONS:
    -h, --help              Shows list of command-line options.
    -v, --version           Shows the version of cvm

    <cmake version>         Installs verson if it does not exist. Otherwise it
                            switches to specified version.

    list [OPTIONS]          Lists versions of cmake that can be installed.
                            With no option will display past 10 releases.
        --all               Lists past 100 releases

    current                 Shows currently selected version.

    remove [OPTIONS]        Removes specified CMake version.
        <version>           Single version you would like to remove. Example: 3.19.0
        --all               Removes all versions and caches (nukes .cvm dir).

    install <version>       Installs the specified option. If no secondary
                            option is mentioned an interactive mode will be
                            used. If the version is installed we will just
                            switch.
        <version>           Specify a version to install example:
                            cvm install 3.20.3

    switch <version>        Switches to the specified version if installed.
                            If not installed it will ask to call 'cvm install'
                            If no secondary option is mentioned, an
                            interactive mode will be used.
        <version>           Specify a version to switch to. example:
                            cvm switch 3.20.2
"
    );
}
