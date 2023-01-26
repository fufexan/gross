use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Output battery info
    Battery,
    /// Output Bluetooth info
    Bluetooth,
    /// Output brightness info
    Brightness,
    /// Output music info
    Music,
    /// Output internet info
    Net,
    /// Output system info
    SystemInfo,
    /// Output volume info
    Volume,
    /// Output workspaces info
    Workspaces,
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {}
        1 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Workspaces) => {
            println!("Workspaces command");
        }
        Some(Commands::Music) => {
            println!("Music command");
        }
        Some(Commands::Battery) => {
            println!("Battery command");
        }
        Some(Commands::Bluetooth) => {
            println!("Bluetooth command");
        }
        Some(Commands::Net) => {
            println!("Net command");
        }
        Some(Commands::Volume) => {
            println!("Volume command");
        }
        Some(Commands::Brightness) => {
            println!("Brightness command");
        }
        Some(Commands::SystemInfo) => {
            println!("SystemInfo command");
        }
        None => {}
    }

    // Continued program logic goes here...
}
