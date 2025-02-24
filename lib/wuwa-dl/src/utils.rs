use crate::prelude::*;

pub const PROGRESS_STYLE: &str = r"{spinner:.green} {file_name:40} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes}";
pub const INDEX_JSON_URL: [&str; 4] = [
    r"https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10003_Y8xXrXk65DqFHEDgApn3cpK5lfczpFx5/index.json", // CN LIVE
    r"https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10008_Pa0Q0EMFxukjEqX33pF9Uyvdc8MaGPSz/index.json", // CN BETA
    r"https://prod-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G153/50004_obOHXFrFanqsaIEOmuKroCcbZkQRBC7c/index.json", // OS LIVE
    r"https://prod-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G153/50013_HiDX7UaJOXpKl3pigJwVxhg5z1wllus5/index.json", // OS BETA
];

pub type Boolean = u8;
pub type DynResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub trait AsBoolean {
    fn as_boolean(self: Self) -> DynResult<bool>
    where
        Self: TryInto<u8>,
        <Self as TryInto<u8>>::Error: 'static + Error + Send + Sync,
    {
        Ok(TryInto::<u8>::try_into(self)?.eq(&0).not())
    }
}

pub trait Volatile {
    fn volatile(self) -> Self
    where
        Self: Sized,
    {
        self
    }
}

impl AsBoolean for Boolean {}

#[macro_export]
macro_rules! get_response {
    ( $json_name:expr, $json_url:expr ) => {
        {
            println!(stringify!(Getting $json_name, please wait a minute...));

            let mut response;

            while {
                response = reqwest::get($json_url).await;
                response.is_err()
            } {
                println!(stringify!(Failed to get $json_name, retrying...));
            }

            response?
        }
        .json::<wuwa_macro_derive::json_type!($json_name)>()
        .await?
    };
}

#[macro_export]
macro_rules! wait_all {
    ( $handles:expr, $n:expr ) => {
        for handle in $handles {
            wuwa_macro_derive::n_try!(handle.await, $n);
        }
    };
}

#[macro_export]
macro_rules! while_err {
    { $x:expr } => {
        while { $x.is_err() } {}
    };
    { $x:block } => {
        while { $x.is_err() } {}
    };
}

#[macro_export]
macro_rules! while_none {
    { $x:expr } => {
        while { $x.is_none() } {}
    };
    { $x:block } => {
        while { $x.is_none() } {}
    };
}
