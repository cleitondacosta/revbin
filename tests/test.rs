#[cfg(test)]
mod tests {
    use revbin::Config;

    #[test]
    fn algorithm_integrity() {
        let random_bytes: Vec<u8> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
        ];

        let reverted_random_bytes = Config::not_bytes(&random_bytes);
        let should_be_equal_random_bytes = Config::not_bytes(&reverted_random_bytes);

        assert_eq!(random_bytes, should_be_equal_random_bytes);
    }
}
