use bitvec::prelude::*;

pub fn primes(max: u64) -> Vec<u64> {
    let mut bits = bitvec![];
    for _ in 0..=max {
        bits.push(false);
    }

    let upper_sqrt = f64::sqrt(max as f64) as i64;

    for i in 1..=upper_sqrt {
        let max = max as i64;

        for j in 1..=upper_sqrt {
            let mut n = 4 * i * i + j * j;
            if n <= max && (n % 12 == 1 || n % 12 == 5) {
                let n = n as usize;
                *bits.get_mut(n).unwrap() = !bits[n];
            }

            n = 3 * i * i + j * j;
            if n <= max && (n % 12 == 7) {
                let n = n as usize;
                *bits.get_mut(n).unwrap() = !bits[n];
            }

            n = 3 * i * i - j * j;
            if i > j && n <= max && (n % 12 == 11) {
                let n = n as usize;
                *bits.get_mut(n).unwrap() = !bits[n];
            }
        }
    }

    for i in 5..=upper_sqrt as usize {
        if bits[i] {
            let mut j = i * i;
            while j <= max as usize {
                *bits.get_mut(j).unwrap() = false;

                j += i * i;
            }
        }
    }

    let mut primes: Vec<u64> = vec![2, 3];

    let mut i = 5;
    while i <= max as usize {
        if bits[i] {
            primes.push(i as u64);
        }
        i += 2;
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_primes(max: u64) -> Vec<u64> {
        let mut primes = vec![];

        (2..=max)
            .filter(|&candidate| {
                if !primes.iter().any(|p| candidate % p == 0) {
                    primes.push(candidate);
                    true
                } else {
                    false
                }
            })
            .take_while(|&p| p < max)
            .last();

        primes
    }

    #[test]
    fn test_primes_for_20() {
        let expected: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(expected, primes(20));
    }

    #[test]
    fn test_primes() {
        assert_eq!(build_primes(10000), primes(10000));
    }
}
