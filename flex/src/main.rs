use std::{path::PathBuf, process::Command};

use ibkr_rust_flex::FlexReader;
use structopt::StructOpt;

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    // Initialize logger
    if !opt.quiet {
        tracing_subscriber::fmt()
            .with_max_level(match opt.verbose {
                0 => tracing::Level::WARN,
                1 => tracing::Level::INFO,
                2 => tracing::Level::DEBUG,
                _ => tracing::Level::TRACE,
            })
            .init();
    }

    // let token = match Command::new("sh").arg("-c").arg(&opt.token).output().await
    // {

    let token = match Command::new("sh").arg("-c").arg(&opt.token).output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(e) => {
            tracing::error!(
                "Failed to launch password command for {}: {}",
                &opt.token,
                e
            );
            return Err(format!(
                "Failed to launch password command for {}: {}",
                &opt.token, e
            )
            .into());
        },
    };
    tracing::error!("{} - {}", token, opt.query);
    let reader = FlexReader {
        write_to_path:      opt.dump_path,
        override_file_name: opt.override_file_name,
    };
    let _response = reader
        .fetch_flex_statement(token.clone(), opt.query.clone())
        .await;
    tracing::error!("{:?}", _response);
    // let file = download_flex_statement(token, opt.query, opt.dump_path).await;
    // let file =
    //    Runtime::new()
    //        .unwrap()
    //        .block_on(download_flex_statement(token, opt.query, opt.dump_path));
    // match file {
    //    Ok(path) => {
    //        println!("{}", path.as_path().display());
    Ok(())
    //    },
    //    Err(e) => Err(e.to_string().into()),
    //}
}

#[derive(Debug, StructOpt)]
//#[structopt(
//    raw(name = "crate_name!()"),
//    raw(version = "crate_version!()"),
//    raw(author = "crate_authors!()"),
//    raw(about = "crate_description!()")
//)]
struct Opt {
    /// Config file
    #[structopt(short = "f", long = "flex_query")]
    query: String,

    /// token file
    #[structopt(short = "t", long = "token")]
    token: String,

    /// download files into destination folder
    #[structopt(short = "d", long = "dump_to_folder", parse(from_os_str))]
    dump_path: Option<PathBuf>,

    /// download files into destination folder
    #[structopt(short = "n", long = "override_file_name")]
    override_file_name: Option<String>,

    /// Silence all log output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Verbose logging mode (-v, -vv, -vvv)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn init() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
    }
}
