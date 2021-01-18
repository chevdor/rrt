use clap::{crate_authors, crate_version, Clap};
use librrt::{Builder, Tokenize};

// use rrtlib::rrt::RRT;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    token: Option<String>,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    New(New),
}

/// A subcommand for generating new tokens
#[derive(Clap)]
struct New {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

fn print_token(token: impl Tokenize + std::fmt::Debug) {
    //  Show token here
    println!(
        r#"       │ │ │ │ │    │ │       └╴╴╴╴checksum: {checksum:?}
       │ │ │ │ │    │ └╴╴╴╴╴╴╴╴╴╴secret: {secret}
       │ │ │ │ │    └╴╴╴╴╴╴╴╴╴╴channel: {channel}"
       │ │ │ │ └╴╴╴╴╴╴╴╴╴╴╴case Id: {case_id}
       │ │ │ └╴╴╴╴╴╴╴╴╴╴╴index: {index}
       │ │ └╴╴╴╴╴╴╴╴╴╴╴network: {network}
       │ └╴╴╴╴╴╴╴╴╴╴version: 0x{version:02x}
       └╴╴╴╴╴╴╴╴╴app: {app}
        "#,
        // token = token.format_string("_"),
        checksum = token.checksum(),
        secret = token.secret(),
        channel = token.channel(),
        case_id = token.case_id(),
        version = token.version(),
        network = token.network(),
        app = token.app(),
        index = token.index(),
    );
    println!("{:#?}", token);
}

fn main() {
    let opts: Opts = Opts::parse();
    // let token = RRT::from_string(&opts.token).expect("This is not a valid token");
    // let token = println!("token: {}", token.format_string("-"));
    // let analysis = Detector::analyze(&opts.token);
    // assert_eq!(Ok((Some(Version::V00), 22)), analysis);

    if let Some(token) = opts.token {
        match Builder::build(&token) {
            // Some(token) => print_token(token),
            Some(token) => {
                println!("version: {:?}", token.version());
                println!("input: {}", token);
                print_token(token);
            }
            None => println!("Got None where we expected Some Token_V00"),
        };
    }

    match opts.subcmd {
        SubCommand::New(t) => {
            if t.debug {
                println!("make a new one");
            }
        }
    }
}
