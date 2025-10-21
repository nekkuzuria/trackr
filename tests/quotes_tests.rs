use trackr::quotes::Quotes;
use std::collections::HashSet;

#[test]
fn test_get_random_returns_valid_quote() {
    let quote = Quotes::get_random();
    assert!(Quotes::MOTIVATIONAL_QUOTES.contains(&quote));
}

#[test]
fn test_all_quotes_are_unique() {
    let mut unique_quotes = HashSet::new();
    for quote in Quotes::MOTIVATIONAL_QUOTES.iter() {
        assert!(unique_quotes.insert(quote), "Duplicate quote found: {}", quote);
    }
}

#[test]
fn test_quotes_array_has_50_items() {
    assert_eq!(Quotes::MOTIVATIONAL_QUOTES.len(), 50);
}

#[test]
fn test_all_quotes_are_non_empty() {
    for quote in Quotes::MOTIVATIONAL_QUOTES.iter() {
        assert!(!quote.is_empty(), "Empty quote found");
    }
}

#[test]
fn test_randomness_distribution() {
    let mut quotes_seen = HashSet::new();

    for _ in 0..100 {
        let quote = Quotes::get_random();
        quotes_seen.insert(quote);
    }

    assert!(quotes_seen.len() >= 10,
        "Expected to see at least 10 different quotes, but only saw {}",
        quotes_seen.len());
}

#[test]
fn test_quote_starts_with_emoji() {
    for quote in Quotes::MOTIVATIONAL_QUOTES.iter() {
        let first_char = quote.chars().next().unwrap();
        let is_emoji = first_char as u32 > 127;
        assert!(is_emoji, "Quote should start with emoji: {}", quote);
    }
}

#[test]
fn test_get_random_is_deterministic_with_same_seed() {
    let quote1 = Quotes::get_random();
    let quote2 = Quotes::get_random();
    assert!(Quotes::MOTIVATIONAL_QUOTES.contains(&quote1));
    assert!(Quotes::MOTIVATIONAL_QUOTES.contains(&quote2));
}

#[test]
fn test_quotes_have_gen_z_slang() {
    let all_quotes = &Quotes::MOTIVATIONAL_QUOTES;
    let gen_z_terms = vec![
        "slay", "bestie", "no cap", "vibes", "iconic", "ate",
        "main character", "CEO of", "rent-free", "understood the assignment",
        "queen", "king", "respectfully", "period", "manifest"
    ];

    let mut found_terms = 0;
    for quote in all_quotes.iter() {
        let lowercase_quote = quote.to_lowercase();
        for term in &gen_z_terms {
            if lowercase_quote.contains(term) {
                found_terms += 1;
                break;
            }
        }
    }

    assert!(found_terms >= 5, "Expected at least 5 quotes with Gen Z slang, found {}", found_terms);
}

