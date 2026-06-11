//! Shared did-you-mean and token diagnostics helpers.

/// Find the closest candidate within edit distance 2, for
/// "Unknown breakpoint `desktoop`. Did you mean `desktop`?" diagnostics.
pub fn closest_name<'a>(
    target: &str,
    candidates: impl IntoIterator<Item = &'a str>,
) -> Option<String> {
    let mut best: Option<(usize, &str)> = None;
    for candidate in candidates {
        let distance = levenshtein(target, candidate);
        if distance <= 2 && best.map(|(d, _)| distance < d).unwrap_or(true) {
            best = Some((distance, candidate));
        }
    }
    best.map(|(_, name)| name.to_string())
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut previous: Vec<usize> = (0..=b.len()).collect();
    let mut current = vec![0; b.len() + 1];
    for (i, &ca) in a.iter().enumerate() {
        current[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let substitution = previous[j] + usize::from(ca != cb);
            current[j + 1] = substitution.min(previous[j + 1] + 1).min(current[j] + 1);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggests_close_names() {
        assert_eq!(
            closest_name("desktoop", ["mobile", "tablet", "desktop", "wide"]),
            Some("desktop".to_string())
        );
        assert_eq!(closest_name("zzz", ["mobile", "tablet"]), None);
    }
}
