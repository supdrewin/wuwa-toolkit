use std::{fmt::Write, io, ops::Deref, path::Path, sync::Arc};

use base16ct::lower;
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use md5::{Digest, Md5};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
    sync::Mutex,
};

use crate::{
    json::resource::Resource,
    utils::{Result, PROGRESS_STYLE},
};

pub struct ResourceHelper {
    inner: Resource,
    pb: Mutex<Option<ProgressBar>>,
}

impl Deref for ResourceHelper {
    type Target = Resource;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ResourceHelper {
    pub fn new(inner: Resource) -> Self {
        let pb = Mutex::new(None);
        Self { inner, pb }
    }

    pub fn with_progress_bar(self) -> Self {
        let Self { inner, .. } = self;

        let pb = ProgressBar::new(inner.size);
        let dest = Path::new(&inner.dest);

        let file_name = dest.file_name().unwrap().to_str().unwrap();
        let file_name = match file_name.len() {
            0..40 => file_name.to_string(),
            _ => format!("{}...", &file_name[..36]),
        };

        let style = ProgressStyle::with_template(PROGRESS_STYLE)
            .unwrap()
            .with_key("file_name", move |_: &ProgressState, w: &mut dyn Write| {
                write!(w, "{file_name}").unwrap()
            })
            .progress_chars("##-");

        pb.set_style(style);

        Self {
            inner,
            pb: Mutex::new(Some(pb)),
        }
    }

    pub async fn with_multi_progress(self, mp: Arc<Mutex<MultiProgress>>) -> Self {
        let Self { inner, pb } = self;
        let mp = mp.lock().await;

        let pb = pb.into_inner().and_then(|pb| Some(mp.add(pb)));
        let pb = Mutex::new(pb);

        Self { inner, pb }
    }

    pub async fn download(&self, base_url: &str, dest_dir: &str) -> Result<()> {
        let dest = &self.dest;

        let file_path = format!("{dest_dir}/Wuthering Waves Game/{dest}");
        let file_path = Path::new(&file_path);

        fs::create_dir_all(file_path.parent().unwrap()).await?;

        while match self.verify(file_path).await {
            Ok(downloaded) => !downloaded,
            Err(_) => true,
        } {
            self.pb(|pb| pb.set_position(0)).await;

            let mut file = File::create(file_path).await?;
            let mut stream = reqwest::get(&format!("{base_url}/{dest}"))
                .await?
                .bytes_stream();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;

                file.write_all(&chunk).await?;
                self.pb(|pb| pb.inc(chunk.len() as u64)).await;
            }

            file.flush().await?;
        }

        Result::Ok(self.pb(|pb| pb.finish()).await)
    }

    async fn verify(&self, file_path: &Path) -> Result<bool> {
        let mut file = File::open(&file_path).await?.into_std().await;
        let mut hasher = Md5::new();

        self.pb(|pb| pb.set_position(self.size)).await;
        io::copy(&mut file, &mut hasher)?;

        let hash = hasher.finalize();
        let hash = lower::encode_string(&hash);

        Result::Ok(hash.eq(&self.md5))
    }

    async fn pb<F: FnOnce(&ProgressBar) -> ()>(&self, op: F) {
        match self.pb.lock().await.deref() {
            Some(pb) => op(pb),
            None => (),
        }
    }
}
