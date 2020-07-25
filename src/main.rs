use clap::clap_app;

fn main() {
    // Get Input from user, Parse with Clap

    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg verbose: -v --verbose "Print test information verbosely")
        (@subcommand test =>
         (about: "controls testing features")
         (version: "1.3")
         (author: "Someone E. <someone_else@other.com>")
         (@arg debug: -d ... "Sets the level of debugging information")
        )
    ).get_matches();

    // Communicate with embedded Squlite database
    //
    // Print out result if -
    // Create then result of creation
    // Read then show data
    // Update then show result of update
    // Delete then show result of deletion 
}
