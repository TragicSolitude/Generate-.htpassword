extern crate clap;
extern crate bcrypt;
extern crate rpassword;

use std::error::Error;
use std::io;
use clap::Arg;
use clap::App;
use rpassword::prompt_password_stderr;

const BCRYPT_ITERATIONS: u32 = 12;

fn main() -> Result<(), Box<dyn Error>>{
    let matches = App::new("Generate .htpasswd")
        .version("1.0")
        .author("Noah Shuart <noah@riafox.com>")
        .about("Generates an .htpasswd user:hash string")
        .arg(Arg::with_name("username")
            .index(1)
            .required(true))
        .get_matches();

    let username = matches.value_of("username").unwrap();
    let password = prompt_password_stderr("Password (input hidden): ")?;
    let hash = bcrypt::hash(password, BCRYPT_ITERATIONS)?;

    println!("{}:{}", username, hash);

    Ok(())
}
