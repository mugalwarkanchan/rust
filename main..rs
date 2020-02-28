use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut f = File::open("abc.txt")?;
    println!("{:?}",f);
    Ok(())
}
