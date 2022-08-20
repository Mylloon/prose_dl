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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Retrieve user's posts
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
    download::download_posts(posts, directory).await;
}
