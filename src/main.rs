use std::error::Error;

use crate::user::{NameStyle, User};

mod user;

fn main() -> Result<(), Box<dyn Error>> {
    let user = User::new("alice");
    println!("{}", user.name(NameStyle::Raw)?);
    println!("{}", user.name(NameStyle::template("<<{name}>>"))?);

    Ok(())
}
