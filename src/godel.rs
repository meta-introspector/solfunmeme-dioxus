use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for Gödel numbering operations
pub trait Godel {
    /// Get the Gödel number for this entity
    fn godel_number(&self) -> u64;
    
    /// Compose multiple entities into a single Gödel number
    fn compose(entities: &[&dyn Godel]) -> u64;
    
    /// Decompose a Gödel number back into constituent entities
    fn decompose(godel_number: u64) -> Vec<u64>;
    
    /// Get prime factorization of a Gödel number
    fn prime_factorization(godel_number: u64) -> HashMap<u64, u32>;
    
    /// Check if two Gödel numbers are equivalent
    fn are_equivalent(a: u64, b: u64) -> bool;
    
    /// Calculate Euclidean distance between two Gödel numbers
    fn euclidean_distance(a: u64, b: u64) -> f64;
    
    /// Calculate Manhattan distance between two Gödel numbers
    fn manhattan_distance(a: u64, b: u64) -> u64;
    
    /// Calculate Chebyshev distance between two Gödel numbers
    fn chebyshev_distance(a: u64, b: u64) -> u64;
    
    /// Get the geometric center of multiple Gödel numbers
    fn geometric_center(numbers: &[u64]) -> f64;
    
    /// Calculate the variance of Gödel numbers
    fn variance(numbers: &[u64]) -> f64;
    
    /// Find the closest Gödel number to a target
    fn find_closest(target: u64, candidates: &[u64]) -> Option<u64>;
    
    /// Calculate the average distance between all pairs
    fn average_pairwise_distance(numbers: &[u64]) -> f64;
}

/// Helper struct for Gödel number operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GodelNumber<T> {
    pub value: u64,
    pub entity: T,
}

impl<T> GodelNumber<T> {
    pub fn new(value: u64, entity: T) -> Self {
        Self { value, entity }
    }
    
    /// Calculate distance to another Gödel number
    pub fn distance_to(&self, other: &GodelNumber<T>) -> f64 {
        Self::euclidean_distance(self.value, other.value)
    }
    
    /// Check if this number is within a certain radius of another
    pub fn is_within_radius(&self, other: &GodelNumber<T>, radius: f64) -> bool {
        self.distance_to(other) <= radius
    }
    
    /// Get the magnitude (distance from origin) of this Gödel number
    pub fn magnitude(&self) -> f64 {
        (self.value as f64).sqrt()
    }
    
    /// Normalize this Gödel number to unit magnitude
    pub fn normalize(&self) -> f64 {
        self.value as f64 / self.magnitude()
    }
}

impl<T> Godel for GodelNumber<T> {
    fn godel_number(&self) -> u64 {
        self.value
    }
    
    fn compose(entities: &[&dyn Godel]) -> u64 {
        if entities.is_empty() {
            return 1;
        }
        
        let mut result = 1u64;
        for (i, entity) in entities.iter().enumerate() {
            let prime = nth_prime(i + 1);
            let power = entity.godel_number();
            result *= prime.pow(power as u32);
        }
        result
    }
    
    fn decompose(godel_number: u64) -> Vec<u64> {
        let mut factors = Vec::new();
        let mut n = godel_number;
        let mut prime_index = 1;
        
        while n > 1 {
            let prime = nth_prime(prime_index);
            let mut power = 0;
            while n % prime == 0 {
                power += 1;
                n /= prime;
            }
            factors.push(power);
            prime_index += 1;
        }
        
        factors
    }
    
    fn prime_factorization(godel_number: u64) -> HashMap<u64, u32> {
        let mut factors = HashMap::new();
        let mut n = godel_number;
        let mut prime_index = 1;
        
        while n > 1 {
            let prime = nth_prime(prime_index);
            let mut power = 0;
            while n % prime == 0 {
                power += 1;
                n /= prime;
            }
            if power > 0 {
                factors.insert(prime, power);
            }
            prime_index += 1;
        }
        
        factors
    }
    
    fn are_equivalent(a: u64, b: u64) -> bool {
        a == b
    }
    
    fn euclidean_distance(a: u64, b: u64) -> f64 {
        let diff = if a > b { a - b } else { b - a };
        (diff as f64).sqrt()
    }
    
    fn manhattan_distance(a: u64, b: u64) -> u64 {
        if a > b { a - b } else { b - a }
    }
    
    fn chebyshev_distance(a: u64, b: u64) -> u64 {
        if a > b { a - b } else { b - a }
    }
    
    fn geometric_center(numbers: &[u64]) -> f64 {
        if numbers.is_empty() {
            return 0.0;
        }
        let sum: u64 = numbers.iter().sum();
        sum as f64 / numbers.len() as f64
    }
    
    fn variance(numbers: &[u64]) -> f64 {
        if numbers.len() < 2 {
            return 0.0;
        }
        
        let mean = Self::geometric_center(numbers);
        let sum_squared_diff: f64 = numbers.iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum();
        
        sum_squared_diff / (numbers.len() - 1) as f64
    }
    
    fn find_closest(target: u64, candidates: &[u64]) -> Option<u64> {
        candidates.iter()
            .min_by(|&&a, &&b| {
                let dist_a = Self::euclidean_distance(target, a);
                let dist_b = Self::euclidean_distance(target, b);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
    
    fn average_pairwise_distance(numbers: &[u64]) -> f64 {
        if numbers.len() < 2 {
            return 0.0;
        }
        
        let mut total_distance = 0.0;
        let mut pair_count = 0;
        
        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                total_distance += Self::euclidean_distance(numbers[i], numbers[j]);
                pair_count += 1;
            }
        }
        
        total_distance / pair_count as f64
    }
}

// Helper function to get the nth prime number
fn nth_prime(n: usize) -> u64 {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    if n <= primes.len() {
        primes[n - 1]
    } else {
        // For larger primes, use a simple algorithm
        let mut candidate = primes[primes.len() - 1] + 2;
        let mut found = primes.len();
        
        while found < n {
            if is_prime(candidate) {
                found += 1;
            }
            candidate += 2;
        }
        
        candidate - 2
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
} 