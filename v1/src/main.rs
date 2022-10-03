use anyhow::Result;

fn main() -> Result<()> {
    let command: u32 = anki::start()?;
    anki::dispatch(command)?;
    Ok(())
}
