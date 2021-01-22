use clap::{crate_authors, crate_version, Clap};
use librrt::*;
use std::convert::TryFrom;
use std::str::FromStr;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    // token: Option<String>,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    /// Generate a new token
    #[clap(version = "1.3", author = "Chevdor <chevdor@gmail.com>")]
    New(New),

    /// Check an existing token
    #[clap(version = "1.3", author = "Chevdor <chevdor@gmail.com>")]
    Check(Check),
}

/// A subcommand for generating new tokens
#[derive(Clap, Debug)]
struct New {
    /// The application
    #[clap(long)]
    app: u8,

    /// The version of the App
    #[clap(long)]
    version: u8,

    /// The Network. 01: Polkadot, 02: Kusama, 42: Westend
    #[clap(long)]
    network: u8,

    /// Registrar index 0..255
    #[clap(long)]
    index: u8,

    /// The case ID of our process
    #[clap(long)]
    id: u64,

    /// The channel: TW | EM | MX
    #[clap(long)]
    channel: String,

    /// The random secret token
    #[clap(long)]
    secret: String,
}

#[derive(Clap, Debug)]
struct Check {
    /// Print debug info
    #[clap(short)]
    token: String,
}

fn print_token(token: impl Tokenize + std::fmt::Debug) {
    let chan = token.channel();
    println!(
        r#"          │  │  │  │  │     │  │        └╴╴╴╴checksum  : {checksum}
          │  │  │  │  │     │  └╴╴╴╴╴╴╴╴╴╴╴╴╴secret    : {secret}
          │  │  │  │  │     └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴channel   : {channel}
          │  │  │  │  └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴case Id   : {case_id}
          │  │  │  └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴reg_index : 0x{index:02x}
          │  │  └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴network   : {network}
          │  └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴version   : 0x{version:02x}
          └╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴app       : 0x{app:02x}
        "#,
        checksum = token.checksum(),
        secret = token.secret(),
        channel = token.channel(),
        case_id = token.case_id(),
        version = token.version(),
        network = token.network(),
        app = token.app(),
        index = token.index(),
    );
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::New(n) => {
            println!("new: {:?}", n);
            let token = TokenV01::new(
                n.app,
                Version::try_from(n.version).expect("version not supported"),
                Network::from(n.network),
                n.index,
                n.id,
                Channel::from(n.channel.to_string()),
            );
            println!("{:?}", token);
        }
        SubCommand::Check(tkn) => {
            let str = &tkn.token;
            // We ask the builder to create a token.
            let candidate = Builder::build_with_variant(&tkn.token);

            match candidate {
                Ok(x) => match x {
                    Token::V00(t) => {
                        println!("Checking: {}", t.format_string("_"));
                        print_token(t);
                    }
                    Token::V01(t) => {
                        println!("Checking: {}", t.format_string("_"));
                        print_token(t);
                    }
                },
                Err(e) => println!("No valid token found:\n{:?}", e),
            }
        }
    }
}
