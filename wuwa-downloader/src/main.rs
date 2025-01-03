use std::{env, fmt::Write, fs, io, path::Path, sync::Arc, thread, time::Duration};

use base16ct::lower;
use console::Term;
use futures_util::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use md5::{Digest, Md5};
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex};
use wuwa_dl::{
    json::{index::IndexJson, resource::ResourceJson},
    utils::{Result, INDEX_JSON_URL, PROGRESS_STYLE},
};

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();

    let threads = Arc::new(Mutex::new(
        cli.threads
            .and_then(|t| Some(usize::min(t, num_cpus::get())))
            .unwrap_or(num_cpus::get()),
    ));

    let dest_dir = Arc::new(cli.path.unwrap_or(env::current_dir()?));
    let multi_progress = Arc::new(Mutex::new(MultiProgress::new()));

    let index_json = wuwa_dl::get_response!(
        index.json,
        INDEX_JSON_URL[((cli.global as usize) << 1) + cli.beta as usize]
    );

    let resources = &index_json.default.resources;
    let base_path = &index_json.default.resources_base_path;

    let host = &index_json
        .default
        .cdn_list
        .get(cli.mirror.unwrap_or_default())
        .unwrap_or(&index_json.default.cdn_list[0])
        .url;

    let resource_json = wuwa_dl::get_response!(resource.json, format!("{host}/{resources}"));

    let mut handles = vec![];

    for resource in resource_json.resource {
        let threads = Arc::clone(&threads);
        let dest_dir = Arc::clone(&dest_dir);
        let multi_progress = Arc::clone(&multi_progress);

        let dest = resource.dest;
        let download_url = format!("{host}/{base_path}/{dest}");

        while {
            let mut threads = threads.lock().await;

            threads.checked_sub(1).is_none_or(|t| {
                *threads = t;
                false
            })
        } {
            thread::sleep(Duration::from_millis(1));
        }

        handles.push(tokio::spawn(async move {
            let dest_dir = dest_dir.display();

            let file_path = format!("{dest_dir}/Wuthering Waves Game/{dest}");
            let file_path = Path::new(&file_path);

            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

            let pb = {
                let mp = multi_progress.lock().await;
                mp.add(ProgressBar::new(resource.size))
            };

            pb.set_style(
                ProgressStyle::with_template(PROGRESS_STYLE)?
                    .with_key("file_name", move |_: &ProgressState, w: &mut dyn Write| {
                        write!(w, "{file_name}").unwrap()
                    })
                    .progress_chars("##-"),
            );

            fs::create_dir_all(file_path.parent().unwrap())?;

            while match (|| {
                let mut file = fs::File::open(&file_path)?;
                let mut hasher = Md5::new();

                pb.set_position(resource.size);
                io::copy(&mut file, &mut hasher)?; // FIXME

                let hash = hasher.finalize();
                let hash = lower::encode_string(&hash);

                Result::Ok(hash.eq(&resource.md5))
            })() {
                Ok(downloaded) => !downloaded,
                Err(_) => true,
            } {
                pb.set_position(0);

                let mut file = File::create(file_path).await?;
                let mut stream = reqwest::get(&download_url).await?.bytes_stream();

                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;

                    file.write_all(&chunk).await?;
                    pb.inc(chunk.len() as u64);
                }

                file.flush().await?;
            }

            *threads.lock().await += 1;
            Result::Ok(pb.finish())
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

mod cli;
