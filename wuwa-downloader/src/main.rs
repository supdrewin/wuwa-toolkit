use std::{env, sync::Arc};

use console::Term;
use indicatif::MultiProgress;
use tokio::{
    sync::Mutex,
    time::{self, Duration},
};
use wuwa_dl::{
    helper::ResourceHelper,
    json::{index::IndexJson, resource::ResourceJson},
    utils::{Result, INDEX_JSON_URL},
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
        let mp = Arc::clone(&multi_progress);

        let dest_dir = Arc::clone(&dest_dir);
        let base_url = format!("{host}/{base_path}");

        while {
            time::sleep(Duration::from_millis(1)).await;

            let mut threads = threads.lock().await;
            let status = threads.checked_sub(1);

            match status {
                Some(t) => *threads = t,
                None => (),
            }

            status.is_none()
        } {}

        handles.push(tokio::spawn(async move {
            let helper = ResourceHelper::new(resource)
                .with_progress_bar()
                .with_multi_progress(mp)
                .await;

            let mut result;

            while {
                result = helper.download(&base_url, dest_dir.to_str().unwrap()).await;
                result.is_err()
            } {}

            *threads.lock().await += 1;
        }));
    }

    wuwa_dl::wait_all!(handles, 1);

    Ok({
        println!("All the resources are downloaded!");
        println!("Press any key to continue...");

        Term::read_key(&Term::stdout())?;
    })
}

mod cli;
