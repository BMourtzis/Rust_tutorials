use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs;
use std::fs::File;

fn main() {
    not_panicing();
    panicing();
}

fn panicing() {
    // panic!("crash and burn");

    // env var RUST_BACKTRACE=1
}

fn not_panicing() {
    let f = File::open("hello.text");

    let f = match f {
        Ok(file) => file,
        // Err(error) => {
        //     panic!("There was a problem opening the file: {:?}", error)
        // },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(fc) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };

    let f = File::open("hello.txt").map_err(|error | {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error | {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        }
        else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}