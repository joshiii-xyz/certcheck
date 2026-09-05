use anyhow::Result;
use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host to check
    host: String,
    /// Port number
    #[arg(short, long, default_value_t = 443)]
    port: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = Command::new("openssl")
        .args([
            "s_client",
            "-connect",
            &format!("{}:{}", args.host, args.port),
            "-servername",
            &args.host,
        ])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().take(30) {
        if !line.is_empty() {
            println!("{}", line);
        }
    }
    Ok(())
}
