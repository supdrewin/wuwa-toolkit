use std::{error::Error, ops::Not};

pub const PROGRESS_STYLE: &str = r"{spinner:.green} {file_name:40} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes}";
pub const INDEX_JSON_URL: [&str; 2] = [
    r"https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10003_Y8xXrXk65DqFHEDgApn3cpK5lfczpFx5/index.json", // CN LIVE
    r"https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10008_Pa0Q0EMFxukjEqX33pF9Uyvdc8MaGPSz/index.json", // CN BETA
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
