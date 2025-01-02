#[macro_export]
macro_rules! get_response {
    ( $x:expr, $y:expr ) => {
        {
            let mut response;

            while {
                response = reqwest::get($y).await;
                response.is_err()
            } {
                println!(stringify!(Failed to get $x, retrying...));
                thread::sleep(Duration::from_secs(1));
            }

            response?
        }
        .json::<wuwa_macro_derive::json_type!($x)>()
        .await?
    };
}
