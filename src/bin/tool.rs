#![feature(box_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(type_ascription)]

use clap::{App, Arg, SubCommand};
use nk_data::*;
use ron::de::from_reader;
use std::fs::File;

fn main() {
    let map_sub = SubCommand::with_name("map")
        .version("0.1.0")
        .author("Tiago Nascimento <xtheosirian@gmail.com>")
        .about("NK maps tool")
        .arg(
            Arg::with_name("map")
                .short("m")
                .long("map")
                .value_name("MAP_FILE")
                .help("Specifies which map to open as default")
                .takes_value(true)
                .multiple(true),
        );

    let data_sub = SubCommand::with_name("data")
        .version("0.1.0")
        .author("Tiago Nascimento <xtheosirian@gmail.com>")
        .about("NK data tool")
        .arg(
            Arg::with_name("flavor")
                .short("f")
                .long("flavor")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse flavor definitions from")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("preparation")
                .short("p")
                .long("preparation")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse preparation definitions from")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("topping")
                .short("t")
                .long("topping")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse topping definitions from")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("hissatsu")
                .long("hissatsu")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse hissatsu definitions from")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("house")
                .long("house")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse house definitions from")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("team")
                .long("team")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse team definitions from")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("player")
                .long("player")
                .value_name("RON_FILE")
                .help("Specifies the file from which to try and parse player definitions from")
                .takes_value(true)
                .multiple(true),
        );

    let matches = App::new("Naisu Kurimu")
        .version("preview-003")
        .author("Tiago Nascimento <xtheosirian@gmail.com>")
        .about("Ferramentas do Jogo Naisu Kurimu")
        .subcommand(map_sub)
        .subcommand(data_sub)
        .get_matches();

    if let Some(_map_matches) = matches.subcommand_matches("map") {
        println!("[ERROR] map not implemented!");
    }

    if let Some(data_matches) = matches.subcommand_matches("data") {
        if let Some(flavors) = data_matches.values_of("flavor") {
            flavors.into_iter().for_each(move |path| {
                println!("[INFO] file '{}'", path);
                if match from_reader(File::open(&path).expect("[ERROR] failed to open!")):
                    Result<Vec<_>, _>
                {
                    Ok(x) => {
                        let mut res = false;
                        if x.iter().any(|f: &FlavorDef| f.key.is_empty()) {
                            res = true;
                            println!("[WARNING] flavors with empty keys detected!");
                        }
                        res
                    }
                    Err(e) => {
                        println!("[ERROR] could not parse file as flavor definition: {}", e);
                        return;
                    }
                } {
                    println!("[WARNING] file has been correctly parsed but has problems!");
                } else {
                    println!("[INFO] file is valid!");
                }
            });
        }

        if let Some(preparations) = data_matches.values_of("preparation") {
            preparations.into_iter().for_each(move |path| {
                println!("[INFO] file '{}'", path);
                if match from_reader(File::open(&path).expect("[ERROR] failed to open!")):
                    Result<Vec<_>, _>
                {
                    Ok(x) => {
                        let mut res = false;
                        if x.iter().any(|p: &PreparationDef| p.key.is_empty()) {
                            res = true;
                            println!("[WARNING] preparations with empty keys detected!");
                        }
                        res
                    }
                    Err(e) => {
                        println!(
                            "[ERROR] could not parse file as preparation definition: {}",
                            e
                        );
                        return;
                    }
                } {
                    println!("[WARNING] file has been correctly parsed but has problems!");
                } else {
                    println!("[INFO] file is valid!");
                }
            });
        }

        if let Some(toppings) = data_matches.values_of("topping") {
            toppings.into_iter().for_each(move |path| {
                println!("[INFO] file '{}'", path);
                if match from_reader(File::open(&path).expect("[ERROR] failed to open!")):
                    Result<Vec<_>, _>
                {
                    Ok(x) => {
                        let mut res = false;
                        if x.iter().any(|t: &ToppingDef| t.key.is_empty()) {
                            res = true;
                            println!("[WARNING] toppings with empty keys detected!");
                        }
                        res
                    }
                    Err(e) => {
                        println!("[ERROR] could not parse file as topping definition: {}", e);
                        return;
                    }
                } {
                    println!("[WARNING] file has been correctly parsed but has problems!");
                } else {
                    println!("[INFO] file is valid!");
                }
            });
        }

        if let Some(_hissatsu) = data_matches.values_of("hissatsu") {
            println!("[ERROR] hissatsu not implemented!");
        }

        if let Some(_houses) = data_matches.values_of("house") {
            println!("[ERROR] house not implemented!");
        }

        if let Some(_teams) = data_matches.values_of("team") {
            println!("[ERROR] team not implemented!");
        }

        if let Some(_players) = data_matches.values_of("player") {
            println!("[ERROR] player not implemented!");
        }
    }
}
