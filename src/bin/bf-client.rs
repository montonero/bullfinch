use bullfinch::errors::BfError;
use bullfinch::Crawler;
use clap::{App, AppSettings, Arg, SubCommand};
use std::str::FromStr;

fn main() -> Result<(), BfError> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        //.setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("VERBOSITY")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("links")
                .about("Get unique links for a given domain.")
                .arg(
                    Arg::with_name("DOMAIN")
                        .help("Domain name")
                        .default_value("https://www.theguardian.com/uk?INTCMP=CE_UK")
                        .required(true),
                )
                .arg(
                    Arg::with_name("DEPTH")
                        .help("Depth to clawl.")
                        .required(true)
                        .takes_value(true)
                        .default_value("1"),
                ),
        )
        /*
        .subcommand(
            SubCommand::with_name("num links")
                .about("Get number of unique links for a given domain.")
                .arg(Arg::with_name("DOMAIN").help("Domain name").required(true)),
        )
        */
        .get_matches();

    let verbose = match matches.occurrences_of("VERBOSITY") {
        0 => false,
        _ => true,
    };

    match matches.subcommand() {
        ("links", Some(matches_)) => {
            let domain: String = matches_
                .value_of("DOMAIN")
                .expect("Domain not supplied")
                .to_string();
            let dep = matches_
                .value_of("DEPTH")
                .expect("Crawling depth not supplied")
                .to_string();

            let mut crawler = Crawler::new(&domain)?;
            crawler.verbose_log = verbose;
            let dep = usize::from_str(&dep).unwrap();
            crawler.crawl_depth = dep;

            println!("Starting to crawl!");
            // This does not block. Crawling started in another thread.
            crawler.start();

            // Wait to see what crawler has crawled
            std::thread::sleep(std::time::Duration::from_secs(1));

            println!("Crawled the following links:");
            let visited = &*crawler.visited.lock().unwrap();
            for l in visited {
                println!("{}", l.to_string());
            }

            Ok(())
        }
        _ => Ok(()),
    }
}
