use std::{error::Error, fmt::Display};


#[derive(Debug)]
enum CustomError{
    error1(String),
    error2(i32),
    error3(i32, String)
}
impl Display for CustomError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::error1(msg) => write!(f, "{} 1 =)", msg),
            Self::error2(code) => write!(f, "cusstom error {} =)", code),
            Self::error3(code, msg) => write!(f, "{} {} =)", msg, code),
        }
    }
}
impl Error for CustomError {}

fn do_somethink() -> Result<(), CustomError>{
    Err(CustomError::error3( 1, "error my love❤ ".to_owned() ))
}

fn main(){
    match do_somethink() {
        Ok(_) => {},
        Err(err) => { println!("{err}")},
    }
}