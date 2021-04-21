use clap::{Arg, App, SubCommand};

mod file_manager;
use crate::file_manager::{FileManager, optionstr_2_string};


fn main() {
    // TODO: add move, rename

    let matches = App::new("file-manager")
        .version("1.0")
        .author("Streamer272")
        .about("File managing program to help you track and control your files")

        .subcommand(SubCommand::with_name("cmp")
            .about("Compares two files")

            .arg(Arg::with_name("cmp-file1")
                .index(1))

            .arg(Arg::with_name("cmp-file2")
                .index(2))

        )

        .subcommand(SubCommand::with_name("wof")
            .about("Writes out file")

            .arg(Arg::with_name("wof-file")
                .index(1))

        )

        .subcommand(SubCommand::with_name("cln")
            .about("Clones file or directory")

            .arg(Arg::with_name("cln-path1")
                .index(1))

            .arg(Arg::with_name("cln-path2")
                .index(2))

        )

        .subcommand(SubCommand::with_name("rmv")
            .about("Removes file or directory")

            .arg(Arg::with_name("rmv-path")
                .index(1))

        )

        .subcommand(SubCommand::with_name("mk")
            .about("Creates file or directory")

            .arg(Arg::with_name("mk-path")
                .index(1))

            .arg(Arg::with_name("mk-type-file")
                .short("--file")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("mk-type-dir")
                .short("--dir")
                .long("--directory")
                .required(false)
                .takes_value(false))

        )

        .get_matches();

    if let Some(matches) = matches.subcommand_matches("cmp") {
        if matches.is_present("cmp-file1") && matches.is_present("cmp-file2") {
            println!("{}", FileManager::compare(
                optionstr_2_string(matches.value_of("cmp-file1")),
                optionstr_2_string(matches.value_of("cmp-file2"))));
        }
        else {
            println!("Please enter both files!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("wof") {
        if matches.is_present("wof-file") {
            println!("{}", FileManager::read_file(
                optionstr_2_string(matches.value_of("wof-file"))));
        }
        else {
            println!("Please enter file!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("rmv") {
        if matches.is_present("rmv-path") {
            FileManager::remove(optionstr_2_string(matches.value_of("rmv-path")));
        }
        else {
            println!("Please enter path!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("mk") {
        if matches.is_present("mk-path") {
            let type_file: bool = matches.is_present("mk-type-file") &&
                !matches.is_present("mk-type-dir");

            FileManager::touch(optionstr_2_string(matches.value_of("mk-path")),
                               type_file);
        }
        else {
            println!("Please enter path!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("cln") {
        if matches.is_present("cln-path1") && matches.is_present("cln-path2") {
            FileManager::clone(optionstr_2_string(matches.value_of("cln-path1")),
                               optionstr_2_string(matches.value_of("cln-path2")));

            println!("Path cloned successfully!");
        }
        else {
            println!("Please enter both paths!");
        }
    }
}
