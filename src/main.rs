use std::{env, fs::metadata, path::PathBuf, process::{self, Command}};



// take <xsize> <path>
fn main() {
    let path_arg = match env::args_os().nth(1) {
        Some(arg) => PathBuf::from(arg),
        None => {
            process::exit(1);
        }
    };

    //let path = PathBuf::from(path_arg);

    //let mut cmd = Command::new("xsize").arg(path).status().expect("Failed to execute command");


    //get file size
    match std::fs::metadata(&path_arg) {
        Ok(metadata) => print!("{} bytes", metadata.len()),
        Err(err) => eprint!("Failed to read path {}", err),
    }
   
    /* ==== list files/folders in a folder
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display());
    }
    */
}
