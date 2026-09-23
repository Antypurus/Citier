use std::fs;

fn main() -> std::io::Result<()> {
    let log = fs::read_to_string("WoWCombatLog-090726_205649.txt")?;
    println!("Hello, world! {}", log);
    Ok(())
}
