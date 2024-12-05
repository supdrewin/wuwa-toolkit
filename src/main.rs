use std::{
    env,
    error::Error,
    fmt::Write,
    fs, io,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use base16ct::lower;
use clap::Parser;
use console::Term;
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use md5::{Digest, Md5};
use serde_json::Value;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[rustfmt::skip]
const STYLE: &str = "{spinner:.green} {item:40} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes}";
const URL: &str = "https://prod-cn-alicdn-gamestarter.kurogame.com/pcstarter/prod/game/G152/10008_Pa0Q0EMFxukjEqX33pF9Uyvdc8MaGPSz/index.json";

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(short, long, value_name = "IDX")]
    mirror: Option<usize>,

    #[arg(short, long, value_name = "NUM")]
    threads: Option<usize>,

    #[arg(short, long, value_name = "DIR")]
    path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let threads = Arc::new(Mutex::new(
        cli.threads
            .and_then(|t| Some(usize::min(t, num_cpus::get())))
            .unwrap_or(num_cpus::get()),
    ));

    let dir = Arc::new(cli.path.unwrap_or(env::current_dir()?));
    let bars = Arc::new(Mutex::new(MultiProgress::new()));

    let mut handles = vec![];

    let index = reqwest::get(URL).await?.json::<Value>().await?;

    let path = index["default"]["resources"].as_str().unwrap();
    let base = index["default"]["resourcesBasePath"].as_str().unwrap();
    let hosts = &index["default"]["cdnList"];

    let host = hosts[cli.mirror.unwrap_or_default()]["url"]
        .as_str()
        .unwrap_or(hosts[0]["url"].as_str().unwrap());

    let resources = reqwest::get(format!("{host}{path}"))
        .await?
        .json::<Value>()
        .await?;

    let verify = |path: &Path, md5: &str| -> Result<bool> {
        let mut file = fs::File::open(&path)?;
        let mut hasher = Md5::new();

        io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();

        Ok(lower::encode_string(&hash).eq(md5))
    };

    for resource in resources["resource"].as_array().unwrap() {
        let threads = Arc::clone(&threads);
        let dir = Arc::clone(&dir);
        let bars = Arc::clone(&bars);

        let dest = resource["dest"].as_str().unwrap().to_string();
        let md5 = resource["md5"].as_str().unwrap().to_string();
        let size = resource["size"].as_u64().unwrap();

        let url = format!("{host}{base}{dest}");

        while {
            thread::sleep(Duration::from_millis(1));
            let mut threads = threads.lock().await;

            match *threads {
                0 => true,
                _ => {
                    *threads -= 1;
                    false
                }
            }
        } {}

        handles.push(tokio::spawn(async move {
            let mut downloaded = false;

            let path = format!("{}/Wuthering Waves Game{dest}", dir.display());
            let path = Path::new(&path);

            let item = path.file_name().unwrap().to_str().unwrap().to_string();

            let bar = {
                let bars = bars.lock().await;
                bars.add(ProgressBar::new(size))
            };

            bar.set_style(
                ProgressStyle::with_template(STYLE)?
                    .with_key("item", move |_: &ProgressState, w: &mut dyn Write| {
                        write!(w, "{item}").unwrap()
                    })
                    .progress_chars("##-"),
            );

            fs::create_dir_all(path.parent().unwrap())?;

            if fs::exists(path)? {
                bar.set_position(size);

                if !md5.is_empty() {
                    downloaded = verify(path, &md5)?;
                }
            }

            while !downloaded {
                bar.set_position(0);

                let mut file = File::create(path).await?;
                let mut stream = reqwest::get(&url).await?.bytes_stream();

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;

                    file.write_all(&chunk).await?;
                    bar.inc(chunk.len() as u64);
                }

                file.flush().await?;
                downloaded = verify(path, &md5)?;
            }

            *threads.lock().await += 1;
            Result::Ok(bar.finish())
        }));
    }

    for handle in handles {
        handle.await??;
    }

    Ok({
        println!("All resources downloaded!");
        Term::read_key(&Term::stdout())?;
    })
}
