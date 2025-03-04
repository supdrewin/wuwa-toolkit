use crate::prelude::*;
use crate::private::*;

pub struct ResourceHelper {
    inner: Resource,
    download_src: String,
    download_dest: String,
    pb: Option<ProgressBar>,
}

#[rustfmt::skip]
impl ResourceHelperBase for ResourceHelper {
    fn md5(&self) -> &str { &self.inner.md5 }

    fn size(&self) -> u64 { self.inner.size }

    fn download_src(&self) -> &str { &self.download_src }

    fn download_dest(&self) -> &Path { Path::new(&self.download_dest) }

    fn pb(&self) -> &Option<ProgressBar> { &self.pb }
}

impl ResourceHelperExt for ResourceHelper {}

impl ResourceHelper {
    pub fn new(inner: Resource, base_url: &str, dest_dir: &str) -> Self {
        let Resource { dest, .. } = &inner;

        let download_src = format!("{base_url}/{dest}");
        let download_dest = format!("{dest_dir}/Wuthering Waves Game/{dest}");

        Self {
            inner,
            download_src,
            download_dest,
            pb: None,
        }
    }

    pub fn with_progress_bar(self) -> Self {
        let Self {
            inner,
            download_src,
            download_dest,
            ..
        } = self;

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
            download_src,
            download_dest,
            pb: Some(pb),
        }
    }

    pub fn with_multi_progress(self, mp: MultiProgress) -> Self {
        let pb = self.pb.and_then(|pb| Some(mp.add(pb)));
        Self { pb, ..self }
    }
}
