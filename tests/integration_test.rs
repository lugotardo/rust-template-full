use rust_app_exemplo::*;

#[test]
fn test_user_lifecycle() {
    let mut user = User::new(1, "Test User".to_string(), "test@example.com".to_string());

    assert!(user.active);
    assert_eq!(user.name, "Test User");

    user.deactivate();
    assert!(!user.active);

    user.activate();
    assert!(user.active);
}

#[test]
fn test_fibonacci_calculations() {
    assert_eq!(fibonacci_optimized(0), 0);
    assert_eq!(fibonacci_optimized(1), 1);
    assert_eq!(fibonacci_optimized(5), 5);
    assert_eq!(fibonacci_optimized(10), 55);
    assert_eq!(fibonacci_optimized(15), 610);
}

#[test]
fn test_factorial_calculations() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(7), 5040);
}

#[test]
fn test_prime_numbers() {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    for prime in primes {
        assert!(is_prime(prime), "{} should be prime", prime);
    }

    let non_primes = vec![0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16];
    for non_prime in non_primes {
        assert!(!is_prime(non_prime), "{} should not be prime", non_prime);
    }
}

#[test]
fn test_string_utilities() {
    assert_eq!(string_utils::to_title_case("hello world"), "Hello World");
    assert_eq!(
        string_utils::to_title_case("rust programming"),
        "Rust Programming"
    );

    assert_eq!(string_utils::count_vowels("hello world"), 3);
    assert_eq!(string_utils::count_vowels("programming"), 3);

    assert_eq!(string_utils::reverse("hello"), "olleh");
    assert_eq!(string_utils::reverse("rust"), "tsur");
}

#[test]
fn test_user_serialization() {
    let user = User::new(42, "Alice".to_string(), "alice@example.com".to_string());

    let json = serde_json::to_string(&user).expect("Failed to serialize user");
    assert!(json.contains("Alice"));
    assert!(json.contains("alice@example.com"));

    let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize user");
    assert_eq!(user, deserialized);
}
