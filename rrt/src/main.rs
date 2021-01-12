use clap::{Clap, crate_version, crate_authors};

use rrtlib::rrt::RRT;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    token: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("input: {}", opts.token);
    let token = RRT::from_string(&opts.token).expect("This is not a valid token");
    println!("token: {}", token.format_string("-"));
    println!("       │  │  │  │     │  │        └╴╴╴╴checksum: {:?}", token.checksum());
    println!("       │  │  │  │     │  └╴╴╴╴╴╴╴╴╴token: {}", token.token());
    println!("       │  │  │  │     └╴╴╴╴╴╴╴╴channel: {}", token.channel());
    println!("       │  │  │  └╴╴╴╴╴╴╴╴╴╴case Id: {}", token.case_id());
    println!("       │  │  └╴╴╴╴╴╴╴╴╴version: 0x{:02x}", token.version());
    println!("       │  └╴╴╴╴╴╴╴╴network: {}-{}", token.network().0, token.network().1);
    println!("       └╴╴╴╴╴╴╴registrar #{}", token.index());
    
    println!("This token is {}", if token.is_valid() { "VALID" } else { "INVALID" });
    // println!("{:#?}", token);
}
