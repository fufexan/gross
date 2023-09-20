use clap::command;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

mod battery;
mod hyprland;
mod music;
mod music_time;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[command(flatten)]
    verbose: Verbosity,

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
    /// Hyprland info
    Hyprland,
    /// General information about a song
    Music,
    /// Time information about a song
    MusicTime,
    /// System info, including net
    SystemInfo,
    /// Volume info
    Volume,
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Some(Commands::Battery) => {
            battery::main();
        }
        Some(Commands::Bluetooth) => {
            todo!("Bluetooth command");
            // bluer crate
        }
        Some(Commands::Brightness) => {
            todo!("Brightness command");
            // brightness crate
        }
        Some(Commands::Hyprland) => {
            hyprland::main();
        }
        Some(Commands::Music) => {
            music::main();
        }
        Some(Commands::MusicTime) => {
            music_time::main();
        }
        Some(Commands::SystemInfo) => {
            todo!("SystemInfo command");
            // sysinfo crate
            // includes net info
        }
        Some(Commands::Volume) => {
            todo!("Volume command");
            // wireplumber?
        }
        None => {}
    }
}
