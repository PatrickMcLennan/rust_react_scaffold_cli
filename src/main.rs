use std::{env, io::{Error, ErrorKind}, fs, process};

fn display_error(e: Error) {
    println!("{:?}", e);
}

fn main() {
    let args = env::args();
    if args.len() != 2 {
        display_error(Error::new(ErrorKind::Other, "You must provide a project name!"));
        process::exit(1)
    }

    let project_name = match args.last() {
        Some(v) => v,
        None => {
            display_error(Error::new(ErrorKind::Other, "Can't access last arg for some dumb reason"));
            process::exit(1)
        }
    };

    let files = match fs::read_dir(".") {
        Ok(v) => v,
        Err(e) => {
            display_error(e);
            process::exit(1)
        }
    };

    files.for_each(|file_name|  {
        match file_name {
            Ok(v) => {
                let name = match v.file_name().to_str() {
                    Some(v) => v.to_string(),
                    None => return ()
                };
                if name == project_name {
                    display_error(Error::new(ErrorKind::Other, format!("There is already a file or directory in located here named {}, please remove this or use a unique name", project_name)));
                }

            },
            Err(_e) => {
                display_error(Error::new(ErrorKind::Other, "Can't read all files in the directory and can't be sure of duplicates"));
                process::exit(1)
            }
        }
    });
}
