use std::{error::Error, ops::Not};

pub const PROGRESS_STYLE: &str = r"{spinner:.green} {file_name:40} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes}";
pub const INDEX_JSON_URL: [&str; 4] = [
    r"https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10003_Y8xXrXk65DqFHEDgApn3cpK5lfczpFx5/index.json", // CN LIVE
    r"https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10008_Pa0Q0EMFxukjEqX33pF9Uyvdc8MaGPSz/index.json", // CN BETA
    r"https://prod-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G153/50004_obOHXFrFanqsaIEOmuKroCcbZkQRBC7c/index.json", // OS LIVE
    r"https://prod-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G153/50013_HiDX7UaJOXpKl3pigJwVxhg5z1wllus5/index.json", // OS BETA
];

pub type Boolean = u8;
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub trait AsBoolean {
    fn as_boolean(self: Self) -> Result<bool>
    where
        Self: TryInto<u8>,
        <Self as TryInto<u8>>::Error: 'static + Error + Send + Sync,
    {
        Ok(TryInto::<u8>::try_into(self)?.eq(&0).not())
    }
}

impl AsBoolean for Boolean {}

#[macro_export]
macro_rules! get_response {
    ( $x:expr, $y:expr ) => {
        {
            use std::{thread, time::Duration};

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

#[macro_export]
macro_rules! wait_all {
    ( $x:expr, $y:expr ) => {
        for handle in $x {
            wuwa_macro_derive::n_try!(handle.await, $y);
        }
    };
}
