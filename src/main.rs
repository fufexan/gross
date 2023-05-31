use clap::command;
use clap::{Parser, Subcommand};

mod battery;
mod music;
mod music_time;

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
    /// Battery info
    Battery,
    /// Bluetooth info
    Bluetooth,
    /// Brightness info
    Brightness,
    /// General information about a song
    Music,
    /// Time information about a song
    MusicTime,
    /// System info, including net
    SystemInfo,
    /// Volume info
    Volume,
    /// Workspaces info
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
        Some(Commands::Battery) => {
            println!("Battery command");
            let bat = battery::main().unwrap();
            println!("{:#?}", bat);
            // upower-dbus crate
        }
        Some(Commands::Bluetooth) => {
            println!("Bluetooth command");
            // bluer crate
        }
        Some(Commands::Brightness) => {
            println!("Brightness command");
            // brightness crate
        }
        Some(Commands::Music) => {
            music::main();
        }
        Some(Commands::MusicTime) => {
            music_time::main();
        }
        Some(Commands::SystemInfo) => {
            println!("SystemInfo command");
            // sysinfo crate
            // includes net info
        }
        Some(Commands::Volume) => {
            println!("Volume command");
            // pipewire crate?
        }
        Some(Commands::Workspaces) => {
            println!("Workspaces command");
            // hyprland-rs crate
        }
        None => {}
    }
}
