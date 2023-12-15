use std::io::Write;

#[inline(always)]
pub fn write_stdout<T, U>(filesize: T, filepath: U) -> Result<(), std::io::Error>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    writeln!(std::io::stdout(), "{}\t{}", filesize, filepath)?;
    Ok(())
}
