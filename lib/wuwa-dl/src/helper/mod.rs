use crate::prelude::*;

#[allow(async_fn_in_trait)]
pub trait ResourceHelperExt: ResourceHelperBase {
    async fn download(&self) -> DynResult<()> {
        fs::create_dir_all(self.download_dest().parent().unwrap()).await?;

        while match self.verify() {
            Ok(downloaded) => !downloaded,
            Err(_) => true,
        } {
            self.pb_fn(|pb| pb.set_position(0));

            let mut file = AsyncFile::create(self.download_dest()).await?;
            let mut stream = reqwest::get(self.download_src()).await?.bytes_stream();

            while let Some(Ok(chunk)) = stream.next().await {
                file.write_all(&chunk).await?;

                self.pb_fn(|pb| {
                    pb.inc(chunk.len() as u64);
                });
            }

            file.flush().await?;
        }

        Ok(self.pb_fn(|pb| pb.finish()))
    }
}

pub trait ResourceHelperBase {
    fn md5(&self) -> &str;

    fn size(&self) -> u64;

    fn download_src(&self) -> &str;

    fn download_dest(&self) -> &Path;

    fn pb(&self) -> &Option<ProgressBar>;

    fn pb_fn<F: FnOnce(&ProgressBar) -> ()>(&self, op: F) {
        match &self.pb() {
            Some(pb) => op(pb),
            None => (),
        }
    }

    fn verify(&self) -> DynResult<bool> {
        let mut file = File::open(self.download_dest())?;
        let mut hasher = Md5::new();

        self.pb_fn(|pb| {
            pb.set_position(self.size());
            pb.enable_steady_tick(Duration::from_millis(20));
        });

        io::copy(&mut file, &mut hasher)?;

        let hash = hasher.finalize();

        self.pb_fn(|pb| pb.disable_steady_tick());
        Ok(format!("{hash:02x}").eq(self.md5()))
    }
}

pub mod resource;
