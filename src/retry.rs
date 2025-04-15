#[macro_export]
macro_rules! retry {
    ($f:expr) => {{
        let mut retries = 0;
        let result = loop {
            let result = $f;
            if result.is_ok() {
                break result;
            } else if retries >= 3 {
                break result;
            }
            retries += 1;
            tokio::time::sleep(std::time::Duration::from_millis(459)).await;
        };
        result
    }};
}
