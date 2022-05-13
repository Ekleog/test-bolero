pub fn fibonacci(number: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..number {
        b += core::mem::replace(&mut a, b);
    }

    b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
