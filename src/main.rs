use std::io::Write;
use std::time::SystemTime;

use clap::{Arg, Command};
use tokio::runtime::Runtime;

mod wudao;

fn main() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(),  "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;

    match matches.subcommand() {
        Some(("date", _matches)) => {
            let now = SystemTime::now();
            let timestamp = match &now.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => {
                    n.as_secs() 
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            write!(std::io::stdout(), "{}\n", timestamp).map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("wd", matches)) => {
            if let Some(word) = matches.get_one::<String>("word") {
                let rt = Runtime::new().unwrap();
                let _result = rt.block_on(wudao::youdao_dict(word));
            }
            // write!(std::io::stdout(), "wd\n").map_err(|e| e.to_string())?;
            // std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
        Some(("quit", _matches)) => {
            write!(std::io::stdout(), "Exiting ...\n").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("my")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLET")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("date")
                .about("display or conversion timestamp")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("wd")
                .about("query word")
                .help_template(APPLET_TEMPLATE)
                .arg(Arg::new("word")
                         .help("the word to translate")
                        .required(true)
                        .index(1)),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .alias("q")
                .about("Quit the MY")
                .help_template(APPLET_TEMPLATE)
        )
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "✨ ").map_err(|e| e.to_string())?;
    // write!(std::io::stdout(), "☯ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
