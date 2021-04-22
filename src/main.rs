use clap::{Arg, App, SubCommand, ArgMatches};

pub mod file_manager;
use crate::file_manager::{FileManager, option2string};


fn get_matches() -> ArgMatches<'static> {
    return App::new("file-manager")
        .version("1.0")
        .author("Streamer272")
        .about("File managing program to help you track and control your files")

        .subcommand(SubCommand::with_name("cmp")
            .about("Compares two files")

            .arg(Arg::with_name("cmp-file1")
                .index(1)
            )

            .arg(Arg::with_name("cmp-file2")
                .index(2)
            )
        )

        .subcommand(SubCommand::with_name("wof")
            .about("Writes out file")

            .arg(Arg::with_name("wof-file")
                .index(1)
            )
        )

        .subcommand(SubCommand::with_name("cln")
            .about("Clones file or directory")

            .arg(Arg::with_name("cln-path1")
                .index(1
            ))

            .arg(Arg::with_name("cln-path2")
                .index(2)
            )
        )

        .subcommand(SubCommand::with_name("rmv")
            .about("Removes file or directory")

            .arg(Arg::with_name("rmv-path")
                .index(1)
            )
        )

        .subcommand(SubCommand::with_name("mk")
            .about("Creates file or directory")

            .arg(Arg::with_name("mk-path")
                .index(1)
            )

            .arg(Arg::with_name("mk-type-file")
                .short("f")
                .long("file")
                .help("Creates file")
                .required(false)
                .takes_value(false)
            )

            .arg(Arg::with_name("mk-type-dir")
                .short("d")
                .long("directory")
                .help("Creates directory")
                .required(false)
                .takes_value(false)
            )
        )

        .subcommand(SubCommand::with_name("rnm")
            .about("Renames file or directory")

            .arg(Arg::with_name("rnm-path1")
                .index(1)
            )

            .arg(Arg::with_name("rnm-path2")
                .index(2)
            )
        )

        .subcommand(SubCommand::with_name("mv")
            .about("Moves file or directory")

            .arg(Arg::with_name("mv-path1")
                .index(1)
            )

            .arg(Arg::with_name("mv-path2")
                .index(2)
            )
        )

        .subcommand(SubCommand::with_name("gwd")
            .about("Prints working directory")
        )

        .get_matches();
}


fn main() {
    FileManager::compare("file1.txt".to_string(),
                         "file2.txt".to_string());
    return;

    let matches = get_matches();

    if let Some(matches) = matches.subcommand_matches("cmp") {
        if matches.is_present("cmp-file1") && matches.is_present("cmp-file2") {
            // println!("{:?}", FileManager::compare(
            //     option2string(matches.value_of("cmp-file1")),
            //     option2string(matches.value_of("cmp-file2"))));
        }
        else {
            println!("Please enter both files!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("wof") {
        if matches.is_present("wof-file") {
            println!("{}", FileManager::read_file(
                option2string(matches.value_of("wof-file"))).unwrap().to_string());
        }
        else {
            println!("Please enter file!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("rmv") {
        if matches.is_present("rmv-path") {
            FileManager::remove(option2string(matches.value_of("rmv-path")));
        }
        else {
            println!("Please enter path!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("mk") {
        if matches.is_present("mk-path") {
            let type_file: bool = matches.is_present("mk-type-file") &&
                !matches.is_present("mk-type-dir");

            FileManager::touch(option2string(matches.value_of("mk-path")),
                               type_file);
        }
        else {
            println!("Please enter path!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("cln") {
        if matches.is_present("cln-path1") && matches.is_present("cln-path2") {
            FileManager::clone(option2string(matches.value_of("cln-path1")),
                               option2string(matches.value_of("cln-path2")));
        }
        else {
            println!("Please enter both paths!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("rnm") {
        if matches.is_present("rnm-path1") && matches.is_present("rnm-path2") {
            FileManager::rename(option2string(matches.value_of("rnm-path1")),
                                option2string(matches.value_of("rnm-path2")));
        }
        else {
            println!("Please enter both paths!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("mv") {
        if matches.is_present("mv-path1") && matches.is_present("mv-path2") {
            let mv_path1 = option2string(matches.value_of("mv-path1"));
            let mut mv_path2 = option2string(matches.value_of("mv-path2"));

            if !mv_path2.ends_with(&mv_path1) {
                if !mv_path2.ends_with("/") {
                    mv_path2.push_str("/");
                }

                mv_path2.push_str(&mv_path1);
            }

            FileManager::rename(mv_path1, mv_path2);
        }
        else {
            println!("Please enter both paths!");
        }
    }

    else if let Some(_matches) = matches.subcommand_matches("gwd") {
        println!("{}", FileManager::get_working_directory());
    }
}
