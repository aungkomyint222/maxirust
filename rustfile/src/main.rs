use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()>{
    let mut file = File::create("hello.html")?;
    file.write_all(b"<div>helloworld</div>")?;
    file.flush()?;

    println!("Successfully wrote to hello.txt");
    Ok(())
}

