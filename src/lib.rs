
/// The inverse of format!().
/// The format argument is the format string, and the s argument is the string to match the format
/// against.
///
/// Examples:
///  - `readf("Hello, {}!", "Hello, world!")` => `Some(vec!["world"])`
///  - `readf("I hope {} are {}!", "I hope you are doing well!")` => `Some(vec!["you", "well"])`
///  - `readf("Goodbye, {}!", "Hello, world!")` => `None`
pub fn readf(format: &str, mut s: &str) -> Option<Vec<String>> {
    if !format.contains("{}") {
        return if format == s { Some(vec![]) } else { None };
    }
    if format.contains("{}{}") {
        panic!("Ambiguous argument: '{}' found in format string.", "{}{}");
    }
    let mut f = format;
    let mut occurences = 0;
    while let Some(idx) = f.find("{}") {
        f = &f[(idx + 2)..];
        occurences += 1;
    }
    let mut result: Vec<String> = Vec::with_capacity(occurences);

    let mut f = format;
    for n in 0..=occurences {
        // shave off space until next {}
        let i = if let Some(x) = f.find("{}") { x } else { f.len() };
        if f.len() < i || s.len() < i {
            return None;
        }
        if &f[0..i] != &s[0..i] {
            return None;
        }
        if n == occurences {
            break;
        }

        // abcde | a{}cd{} => bcde | cd{}
        f = &f[(i + 2)..]; 
        s = &s[i..];
        // populate
        match f.find("{}") { // has next
            Some(x) => { // yes, append until next segment
                // bcde | cd{}
                // |      \ |
                // gets the next segment, and appends everything until the beginning of it
                result.push(s[0..s.find(&f[0..x])?].into());
            },
            None => { // no, append until last segment
                // gets the last segment (last {} until end of string) and pushes up until it
                result.push(s[0..(s.len() - (format.len() - format.rfind("{}").unwrap() - 2))].into())
            },
        }
        // shave the appended string away
        s = &s[result[n].len()..];
    }
    if result.len() != occurences {
        None
    } else {
        Some(result)
    }
}

/// Convenience function for single-item format strings. Extra items are dropped.
///
/// Examples:
///  - `readf1("Hello, {}!", "Hello, world!")` => `Some("world")`
///  - `readf1("I hope {} are doing well!", "I hope you are doing well!")` => `Some("you")`
///  - `readf1("Goodbye, {}!", "Hello, world!")` => `None`
pub fn readf1(format: &str, s: &str) -> Option<String> {
    let r = readf(format, s);
    r.map(|x| if x.len() == 0 { "".into() } else { x[0].clone() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_readf1() {
        assert_eq!(readf1("hello, {}", "hello, person"), Some("person".into()));
        assert_eq!(readf1("hello, {}!", "hello, person!"), Some("person".into()));
        assert_eq!(readf1("Hello, {}", "hello, person"), None);
        assert_eq!(readf1("Hello, {}!", "hello, person"), None);
        assert_eq!(readf1("hello!", "hello!"), Some("".into()));
        assert_eq!(readf1("Hello!", "hello!"), None);
        assert_eq!(readf1("{}", "person"), Some("person".into()));
    }

    #[test]
    fn t_readf() {
        assert_eq!(readf("hello, {} and {}", "hello, person 1 and person 2"), Some(vec!["person 1".into(), "person 2".into()]));
        assert_eq!(readf("hello, {} and {}!", "hello, person 1 and person 2!"), Some(vec!["person 1".into(), "person 2".into()]));
        assert_eq!(readf("hello, {} and {}", "hello, person"), None);
        assert_eq!(readf("hello, {} and {}!", "hello, person!"), None);
        assert_eq!(readf("Hello, {} and {}", "hello, person 1 and person 2"), None);
        assert_eq!(readf("Hello, {} and {}!", "hello, person 1 and person 2!"), None);
        assert_eq!(readf("hello!", "hello!"), Some(vec![]));
        assert_eq!(readf("Hello!", "hello!"), None);
        assert_eq!(readf("{}, {}", "person 1, person 2"), Some(vec!["person 1".into(), "person 2".into()]));
    }
}
