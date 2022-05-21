/*!
Example updating an executable to the latest version released via GitHub
*/

fn update(ver: &str) -> Result<(), Box<dyn ::std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("arriven")
        .repo_name("db1000n")
        .bin_name("db1000n")
        .bin_install_path("db1000n")
        .target("linux_amd64")
        //.show_download_progress(true)
        //.target_version_tag("v9.9.10")
        //.show_output(false)
        .no_confirm(true)
        //
        // For private repos, you will need to provide a GitHub auth token
        // **Make sure not to bake the token into your app**; it is recommended
        // you obtain it via another mechanism, such as environment variables
        // or prompting the user for input
        //.auth_token(env!("DOWNLOAD_AUTH_TOKEN"))
        .current_version(ver)
        .build()?
        .update()?;
    println!("Update status: `{}`!", status);
    Ok(())
}

fn get_version() -> Result<String, Box<dyn ::std::error::Error>> {
    use std::process::Command;

    let output = Command::new("./db1000n")
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

pub fn main()-> Result<(), Box<dyn ::std::error::Error>> {
    let version = get_version();

    match version {
        Ok(v) => update(&v)?,
        Err(_) => update("0.0.0")?,
    }
    Ok(())
}