use clap::{Arg, App, SubCommand};

mod file_manager;
use crate::file_manager::{FileManager, optionstr_2_string};


fn main() {
    let matches = App::new("file-manager")
        .version("1.0")
        .author("Streamer272")
        .about("File managing program to help you track and control your files")

        .subcommand(SubCommand::with_name("compare")
            .about("Compares two files")

            .arg(Arg::with_name("file1")
                .index(1))

            .arg(Arg::with_name("file2")
                .index(2))

        )

        .subcommand(SubCommand::with_name("cat")
            .about("Writes out file")

            .arg(Arg::with_name("file")
                .index(1))

        )

        .subcommand(SubCommand::with_name("clone")
            .about("Clones file or directory")

            .arg(Arg::with_name("path1")
                .index(1))

            .arg(Arg::with_name("path2")
                .index(2))

        )

        .get_matches();

    if let Some(matches) = matches.subcommand_matches("compare") {
        if matches.is_present("file1") && matches.is_present("file2") {
            println!("{}", FileManager::compare(
                optionstr_2_string(matches.value_of("file1")),
                optionstr_2_string(matches.value_of("file2"))));
        }
        else {
            println!("Please enter both files!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("cat") {
        if matches.is_present("file") {
            println!("{}", FileManager::read_file(
                optionstr_2_string(matches.value_of("file"))));
        }
        else {
            println!("Please enter file!");
        }
    }

    else if let Some(matches) = matches.subcommand_matches("clone") {
        if matches.is_present("path1") && matches.is_present("path2") {
            FileManager::clone(optionstr_2_string(matches.value_of("path1")),
                               optionstr_2_string(matches.value_of("path2")));

            println!("Path cloned successfully!");
        }
        else {
            println!("Please enter both paths!");
        }
    }
}
