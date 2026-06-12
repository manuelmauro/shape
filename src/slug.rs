//! Slug and title helpers shared across commands.

/// Reduce arbitrary text to a kebab-case slug: lowercase ASCII alphanumerics,
/// runs of anything else collapse to a single hyphen, with no leading or
/// trailing hyphen.
pub fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(ch.to_ascii_lowercase());
            pending_dash = false;
        } else {
            pending_dash = true;
        }
    }
    slug
}

/// Turn a slug into a human title: `auto-pay` → `Auto Pay`.
pub fn titleize(slug: &str) -> String {
    slug.split('-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes() {
        assert_eq!(slugify("Auto Pay!"), "auto-pay");
        assert_eq!(slugify("autopay"), "autopay");
        assert_eq!(slugify("  multi   word  "), "multi-word");
        assert_eq!(slugify("Trailing--dash--"), "trailing-dash");
        assert_eq!(slugify("!!!"), "");
    }

    #[test]
    fn titleize_capitalizes_words() {
        assert_eq!(titleize("auto-pay"), "Auto Pay");
        assert_eq!(titleize("autopay"), "Autopay");
    }
}
