use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("status") => {
            println!("SPECTER-NET CLI v{}", env!("CARGO_PKG_VERSION"));
            println!("Status: Operational");
            println!("Component: specter-cli");
        }
        Some("version") => {
            println!("SPECTER-NET v{}", env!("CARGO_PKG_VERSION"));
        }
        Some("config") => {
            let config_path = env::var("SPECTER_CONFIG")
                .unwrap_or_else(|_| "/etc/specter-net/specter.toml".to_string());
            match specter_common::SystemConfig::load(&config_path) {
                Ok(config) => {
                    println!("Configuration loaded from: {}", config_path);
                    println!("Site: {}", config.system.site_name);
                    println!("Device: {}", config.rf.device);
                    println!("Sample rate: {} Hz", config.rf.sample_rate);
                    println!("FFT size: {}", config.rf.fft_size);
                    println!(
                        "Auto channel change: {}",
                        config.policy.automatic_channel_change
                    );
                }
                Err(e) => {
                    eprintln!("Failed to load config: {}", e);
                    eprintln!("Using default configuration:");
                    let default = specter_common::SystemConfig::default();
                    println!("Site: {}", default.system.site_name);
                    println!("Device: {}", default.rf.device);
                }
            }
        }
        Some("validate") => {
            let config_path = env::var("SPECTER_CONFIG")
                .unwrap_or_else(|_| "/etc/specter-net/specter.toml".to_string());
            match specter_common::SystemConfig::load(&config_path) {
                Ok(_) => {
                    println!("Configuration is valid");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Configuration validation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("SPECTER-NET CLI v{}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("Usage: specter-cli <command>");
            println!();
            println!("Commands:");
            println!("  status     Show system status");
            println!("  version    Show version");
            println!("  config     Show current configuration");
            println!("  validate   Validate configuration file");
        }
    }
    Ok(())
}
