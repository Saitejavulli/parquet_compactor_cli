use anyhow::{bail, Result};

pub fn parse_size(input: &str) -> Result<u64> {
    let s = input.trim().to_uppercase();

    if let Some(num) = s.strip_suffix("KB") {
        return Ok(num.trim().parse::<u64>()? * 1024);
    }

    if let Some(num) = s.strip_suffix("MB") {
        return Ok(num.trim().parse::<u64>()? * 1024 * 1024);
    }

    if let Some(num) = s.strip_suffix("GB") {
        return Ok(num.trim().parse::<u64>()? * 1024 * 1024 * 1024);
    }

    bail!("Unsupported size format: {}", input)
}