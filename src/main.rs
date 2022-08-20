use clap::Parser;

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

fn main() {
    let cli = Cli::parse();

    let posts = parse::get_posts(
        cli.scheme.to_lowercase(),
        cli.username.to_lowercase(),
        cli.domain.to_lowercase(),
    );

    println!("{:#?}", posts);
}
