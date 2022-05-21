// convert arch to go format
fn get_arch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "x86" => "386",
        "aarch64" => "arm64",
        _ => std::env::consts::ARCH,
    }
}

// generate release targets compatible with goreleaser
fn get_target() -> String {
    format!("{}_{}", std::env::consts::OS, get_arch())
}

fn get_bin_name() -> String {
    format!("db1000n{}", std::env::consts::EXE_SUFFIX)
}

fn get_command() -> String {
    format!("./{}", get_bin_name())
}

fn update(ver: &str) -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("arriven")
        .repo_name("db1000n")
        .bin_name(&get_bin_name())
        .bin_install_path(get_bin_name())
        .target(&get_target())
        .show_output(false)
        .no_confirm(true)
        .current_version(ver)
        .build()?
        .update()?;
    println!("Update status: `{}`!", status);
    Ok(())
}

fn get_version() -> Result<String, Box<dyn ::std::error::Error>> {
    use std::process::Command;

    let output = Command::new(&get_command())
                .args(["--version", "--log-format", "json"])
                .output()?;

    let input = std::str::from_utf8(&output.stderr)?;

    use serde_json::Value;

    // parse into generic JSON value
    let root: Value = serde_json::from_str(input)?;

    // access element using .get()
    let version = root.get("version").and_then(|value| value.as_str()).ok_or("no version")?;

    Ok(version.to_string())
}

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value_t = 3600)]
    interval: u64,
}

pub fn main()-> Result<(), Box<dyn ::std::error::Error>> {
    let args = Cli::parse();
    loop {
        let version = get_version();

        match version {
            Ok(v) => update(&v)?,
            Err(_) => update("0.0.0")?,
        }

        let mut db1000n = std::process::Command::new(&get_command()).spawn()?;
        std::thread::sleep(std::time::Duration::from_secs(args.interval));
        db1000n.kill()?;
    }
}