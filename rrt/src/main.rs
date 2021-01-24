use clap::{crate_authors, crate_version, Clap};
use librrt::*;
use std::convert::TryFrom;
use termion::{color, style};

// TODO: Fix doc below
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
    #[clap(author = "Chevdor <chevdor@gmail.com>")]
    New(New),

    /// Check an existing token
    #[clap(author = "Chevdor <chevdor@gmail.com>")]
    Check(Check),
}

/// A subcommand for generating new tokens
#[derive(Clap, Debug)]
struct New {
    /// The application
    #[clap(long, default_value = "0")]
    app: u8,

    /// The version of the App
    #[clap(long, default_value = "1")]
    version: u8,

    /// The Network. 01: Polkadot, 02: Kusama, 42: Westend
    #[clap(long)]
    network: u8,

    /// Registrar index 0..255
    #[clap(long, default_value = "1")]
    index: u8,

    /// The case ID of our process
    #[clap(long)]
    id: u64,

    /// The channel: TW | EM | MX
    #[clap(long)]
    channel: String,

    #[clap(long)]
    separator: Option<String>,
}

#[derive(Clap, Debug)]
struct Check {
    /// Print debug info
    #[clap(short)]
    token: String,
}

fn print_token(token: impl Tokenize + std::fmt::Debug) {
    let c1 = color::Fg(color::Red);
    let c2 = color::Fg(color::Green);
    let c3 = color::Fg(color::Blue);
    let c4 = color::Fg(color::Yellow);

    println!(
        "{app}{S}{version}{S}{network}{S}{index}{S}{case_id}{S}{channel}{S}{secret}{S}{checksum}",
        S = format!("{}{}{}", color::Fg(color::Yellow), "-", style::Reset),
        app = format!("{}{:02x}{}", c1, token.app(), style::Reset),
        version = format!("{}{:02x}{}", c3, token.version(), style::Reset),
        network = format!(
            "{}{:02x}{}",
            c2,
            Into::<u8>::into(token.network()),
            style::Reset
        ),
        index = format!("{}{:02x}{}", c1, token.index(), style::Reset),
        case_id = format!("{}{:05x}{}", c3, token.case_id(), style::Reset),
        channel = format!("{}{}{}", c2, token.channel().to_string(), style::Reset),
        secret = format!("{}{}{}", c1, token.secret(), style::Reset),
        checksum = format!("{}{}{}", c4, token.checksum(), style::Reset),
    );

    println!(
        r#"{C1}│  {C3}│  {C2}│  {C1}│  {C3}│     {C2}│  {C1}│        {C4}└╴╴╴╴checksum  : {checksum}{NORM}
{C1}│  {C3}│  {C2}│  {C1}│  {C3}│     {C2}│  {C1}└╴╴╴╴╴╴╴╴╴╴╴╴╴secret    : {secret}{NORM}
{C1}│  {C3}│  {C2}│  {C1}│  {C3}│     {C2}└╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴channel   : {channel}{NORM}
{C1}│  {C3}│  {C2}│  {C1}│  {C3}└╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴case Id   : {case_id} (hex: {case_id:05X}){NORM}
{C1}│  {C3}│  {C2}│  {C1}└╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴reg_index : {C1}0x{index:02x}{NORM}
{C1}│  {C3}│  {C2}└╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴network   : {network}{NORM}
{C1}│  {C3}└╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴version   : 0x{version:02x}{NORM}
{C1}└╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴╴app       : 0x{app:02x}{NORM}
        "#,
        app = token.app(),
        version = token.version(),
        network = match token.network() {
            Network::Known(n) => format!("{:?}", n),
            Network::Unknown(u) => format!("0x{:02x}", u),
        },
        index = token.index(),
        case_id = token.case_id(),
        channel = token.channel().format_str(),
        secret = token.secret(),
        checksum = token.checksum(),
        C1 = c1,
        C2 = c2,
        C3 = c3,
        C4 = c4,
        NORM = style::Reset,
    );
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::New(n) => {
            let token = match (n.app, Version::try_from(n.version)) {
                (0, Ok(Version::V00)) => Token::V00(TokenV00::new(
                    n.app,
                    Version::try_from(n.version).expect("version not supported"),
                    Network::from(n.network),
                    n.index,
                    n.id,
                    Channel::from(n.channel.as_str()),
                )),
                (0, Ok(Version::V01)) => Token::V01(TokenV01::new(
                    n.app,
                    Version::try_from(n.version).expect("version not supported"),
                    n.network,
                    n.index,
                    n.id,
                    Channel::from(n.channel.as_str()),
                )),
                _ => panic!("App/Version not supported"),
            };

            // The following could be done in debug
            // println!("Token   : {}", token.format_string(""));
            // println!("Token   : {}", token.format_string("_"));
            // print_token(token);

            // Output the generated token
            let sep = match n.separator {
                None => String::from(""),
                Some(s) => s,
            };
            println!("{}", token.format_string(&sep));
        }
        SubCommand::Check(tkn) => {
            let candidate = Builder::build_with_variant(&tkn.token);
            match candidate {
                Ok(t) => {
                    // println!("Checking: {}", t.format_string("_"));
                    print_token(t);
                }
                Err(e) => println!("No valid token found:\n{:?}", e),
            }
        }
    }
}
