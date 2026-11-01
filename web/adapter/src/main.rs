use std::{env, fs::File, io::Read};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = env::args()
        .nth(1)
        .ok_or("usage: scenarium-compare REQUEST.json | --examples")?;
    if arg == "--examples" {
        println!("{}", scenarium_web::examples());
    } else {
        let mut input = String::new();
        File::open(arg)?
            .take(scenarium_web::MAX_REQUEST as u64 + 1)
            .read_to_string(&mut input)?;
        println!("{}", scenarium_web::compare(&input));
    }
    Ok(())
}
