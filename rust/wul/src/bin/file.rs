use std::fs::File;
use std::io::{Read, Write};
use std::io;
use std::env;

fn read_content(filename: &str) -> Result<String, io::Error> {
    let mut fin = File::open(filename)?;
    //let s = io::read_to_string(&mut fin);
    let mut s = String::new();
    fin.read_to_string(&mut s)?;
    return Ok(s);
}
	
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let content:String = match read_content(&args[1]) {
	Ok(v) => v,
	Err(msg) => {
	    println!("Read failed:{}", msg);
	    "".to_string()
	}

    };
    let fn2 = args[1].clone() + ".bak";
    File::create(&fn2).unwrap().write_all(&content.as_bytes());

}
