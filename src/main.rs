use clap::Parser;

mod download;
mod parse;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Your username
    #[clap(value_parser)]
    username: String,

    /// Directory output [default: the username]
    #[clap(short, long, value_parser, value_name = "DIRECTORY")]
    directory: Option<String>,

    /// Domain name
    #[clap(
        long,
        value_parser,
        value_name = "DOMAIN NAME",
        default_value = "prose.sh"
    )]
    domain: String,

    /// Scheme: HTTP/HTTPS
    #[clap(long, value_parser, default_value = "https")]
    scheme: String,

    /// Download special files
    #[clap(short, takes_value = false)]
    special_files: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Retrieve user's posts
    println!("Retrieving posts...");
    let posts = parse::get_posts(
        cli.scheme.to_lowercase(),
        cli.username.to_lowercase(),
        cli.domain.to_lowercase(),
    )
    .await;

    // Defines the output folder name
    let directory = match cli.directory {
        Some(loc) => loc,
        None => cli.username,
    };

    // Download the posts
    println!("Downloads posts...");
    download::download_posts(posts, &directory, cli.special_files).await;
    println!("Download completed in {}/ folder.", directory);
}
