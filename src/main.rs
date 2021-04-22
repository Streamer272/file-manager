use clap::{Arg, App, SubCommand, ArgMatches};

mod file_manager;
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

        .subcommand(SubCommand::with_name("mks")
            .about("Creates file or directory")

            .arg(Arg::with_name("mks-path")
                .index(1)
            )

            .arg(Arg::with_name("mks-type-file")
                .short("f")
                .long("file")
                .help("Creates file")
                .required(false)
                .takes_value(false)
            )

            .arg(Arg::with_name("mks-type-dir")
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

        .subcommand(SubCommand::with_name("mvt")
            .about("Moves file or directory")

            .arg(Arg::with_name("mvt-path1")
                .index(1)
            )

            .arg(Arg::with_name("mvt-path2")
                .index(2)
            )
        )

        .subcommand(SubCommand::with_name("gwd")
            .about("Prints working directory")
        )

        .get_matches();
}


fn main() {
    let matches = get_matches();

    if let Some(matches) = matches.subcommand_matches("cmp") {
        if matches.is_present("cmp-file1") && matches.is_present("cmp-file2") {
            println!("{:?}", FileManager::compare(
                option2string(matches.value_of("cmp-file1")),
                option2string(matches.value_of("cmp-file2"))));
        }
        else {
            println!("Please enter both files!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("wof") {
        if matches.is_present("wof-file") {
            println!("{:?}", FileManager::read_file(
                option2string(matches.value_of("wof-file"))));
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

    else if let Some(matches) = matches.subcommand_matches("mks") {
        if matches.is_present("mks-path") {
            let type_file: bool = matches.is_present("mks-type-file") &&
                !matches.is_present("mks-type-dir");

            FileManager::touch(option2string(matches.value_of("mks-path")),
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

    else if let Some(matches) = matches.subcommand_matches("mvt") {
        if matches.is_present("mvt-path1") && matches.is_present("mvt-path2") {
            let mvt_path1 = option2string(matches.value_of("mvt-path1"));
            let mut mvt_path2 = option2string(matches.value_of("mvt-path2"));

            if !mvt_path2.ends_with(&mvt_path1) {
                if !mvt_path2.ends_with("/") {
                    mvt_path2.push_str("/");
                }

                mvt_path2.push_str(&mvt_path1);
            }

            FileManager::rename(mvt_path1, mvt_path2);
        }
        else {
            println!("Please enter both paths!");
        }
    }

    else if let Some(_matches) = matches.subcommand_matches("gwd") {
        println!("{}", FileManager::get_working_directory());
    }
}
