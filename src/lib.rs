//! Biblioteca de utilitários para a aplicação Rust modelo
//!
//! Esta biblioteca contém funções e estruturas auxiliares que podem ser
//! reutilizadas em diferentes partes da aplicação.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Estrutura que representa um usuário do sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub active: bool,
}

impl User {
    /// Cria um novo usuário
    pub fn new(id: u64, name: String, email: String) -> Self {
        User {
            id,
            name,
            email,
            active: true,
        }
    }

    /// Desativa o usuário
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Ativa o usuário
    pub fn activate(&mut self) {
        self.active = true;
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User(id: {}, name: {}, email: {}, active: {})",
            self.id, self.name, self.email, self.active
        )
    }
}

/// Calcula fibonacci de forma otimizada usando iteração
pub fn fibonacci_optimized(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

/// Calcula o fatorial de um número
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}

/// Verifica se um número é primo
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }

    true
}

/// Módulo de processamento de strings
pub mod string_utils {
    /// Converte uma string para título (primeira letra de cada palavra em maiúscula)
    pub fn to_title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + chars.as_str().to_lowercase().as_str()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Conta o número de vogais em uma string
    pub fn count_vowels(s: &str) -> usize {
        s.chars()
            .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
            .count()
    }

    /// Inverte uma string
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(1, "João Silva".to_string(), "joao@example.com".to_string());
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "João Silva");
        assert_eq!(user.email, "joao@example.com");
        assert!(user.active);
    }

    #[test]
    fn test_user_activation() {
        let mut user = User::new(1, "Maria".to_string(), "maria@example.com".to_string());
        assert!(user.active);

        user.deactivate();
        assert!(!user.active);

        user.activate();
        assert!(user.active);
    }

    #[test]
    fn test_fibonacci_optimized() {
        assert_eq!(fibonacci_optimized(0), 0);
        assert_eq!(fibonacci_optimized(1), 1);
        assert_eq!(fibonacci_optimized(10), 55);
        assert_eq!(fibonacci_optimized(20), 6765);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(!is_prime(9));
        assert!(is_prime(11));
        assert!(is_prime(97));
    }

    #[test]
    fn test_title_case() {
        assert_eq!(string_utils::to_title_case("hello world"), "Hello World");
        assert_eq!(
            string_utils::to_title_case("rust é incrível"),
            "Rust É Incrível"
        );
    }

    #[test]
    fn test_count_vowels() {
        assert_eq!(string_utils::count_vowels("hello"), 2);
        assert_eq!(string_utils::count_vowels("Rust"), 1);
        assert_eq!(string_utils::count_vowels("aeiou"), 5);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(string_utils::reverse("hello"), "olleh");
        assert_eq!(string_utils::reverse("Rust"), "tsuR");
    }
}
