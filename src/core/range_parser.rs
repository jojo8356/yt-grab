use crate::error::{AppError, Result};

/// Parse a range string like "1,3-5,8" into a sorted, deduplicated Vec of indices.
/// Also accepts "all" to select everything.
pub fn parse_ranges(input: &str, max: usize) -> Result<Vec<usize>> {
    let input = input.trim();

    if input.eq_ignore_ascii_case("all") {
        return Ok((1..=max).collect());
    }

    let mut indices = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if let Some((start_str, end_str)) = part.split_once('-') {
            let start: usize = start_str
                .trim()
                .parse()
                .map_err(|_| AppError::InvalidRange(format!("invalid number in \"{part}\"")))?;
            let end: usize = end_str
                .trim()
                .parse()
                .map_err(|_| AppError::InvalidRange(format!("invalid number in \"{part}\"")))?;

            if start == 0 || end == 0 {
                return Err(AppError::InvalidRange("indices start at 1".into()));
            }
            if start > end {
                return Err(AppError::InvalidRange(format!("{start} > {end}")));
            }
            if end > max {
                return Err(AppError::InvalidRange(format!(
                    "{end} exceeds playlist size ({max})"
                )));
            }

            indices.extend(start..=end);
        } else {
            let idx: usize = part
                .parse()
                .map_err(|_| AppError::InvalidRange(format!("invalid number \"{part}\"")))?;

            if idx == 0 {
                return Err(AppError::InvalidRange("indices start at 1".into()));
            }
            if idx > max {
                return Err(AppError::InvalidRange(format!(
                    "{idx} exceeds playlist size ({max})"
                )));
            }

            indices.push(idx);
        }
    }

    indices.sort_unstable();
    indices.dedup();

    if indices.is_empty() {
        return Err(AppError::InvalidRange("no indices selected".into()));
    }

    Ok(indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(parse_ranges("3", 10).unwrap(), vec![3]);
    }

    #[test]
    fn test_comma_separated() {
        assert_eq!(parse_ranges("1,3,5", 10).unwrap(), vec![1, 3, 5]);
    }

    #[test]
    fn test_range() {
        assert_eq!(parse_ranges("2-5", 10).unwrap(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(parse_ranges("1,3-5,8", 10).unwrap(), vec![1, 3, 4, 5, 8]);
    }

    #[test]
    fn test_all() {
        assert_eq!(parse_ranges("all", 4).unwrap(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_dedup() {
        assert_eq!(parse_ranges("1,1,2", 5).unwrap(), vec![1, 2]);
    }

    #[test]
    fn test_out_of_bounds() {
        assert!(parse_ranges("11", 10).is_err());
    }

    #[test]
    fn test_zero() {
        assert!(parse_ranges("0", 10).is_err());
    }
}
