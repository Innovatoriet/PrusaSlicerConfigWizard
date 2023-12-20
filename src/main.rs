use clap::Command;




fn main() {
    let cli = Command::new("PrusaSlicer Configuration Wizard")
        .author("Linus Michelsson(Innovatoriet)")
        .version("0.0.1-alpha1")
        .about("Command line interface for managing prusa slicer configurations using git repositories")
        .subcommand(
            Command::new("init")
        )
        .subcommand(
            Command::new("update")
        );

    let matches = cli.get_matches();
    
}
