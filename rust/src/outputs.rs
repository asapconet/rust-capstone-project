pub fn write_out_txt(outputs: &[u64], address: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create("outputs.txt")?;
    for output in outputs {
        writeln!(file, "{}", output)?;
    }
    writeln!(file, "Address: {}", address)?;
    Ok(())
}
