use std::{env, sync::Arc};

use console::Term;
use indicatif::MultiProgress;
use wuwa_dl::{
    helper::ResourceHelper,
    json::{index::IndexJson, resource::ResourceJson},
    pool::Pool,
    utils::{Result, INDEX_JSON_URL},
};

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();

    let pool = Pool::new(
        cli.threads
            .and_then(|t| Some(usize::min(t, num_cpus::get())))
            .unwrap_or(num_cpus::get()),
    );

    let dest_dir = Arc::new(cli.path.unwrap_or(env::current_dir()?));
    let mp = MultiProgress::new();

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
        let pool = pool.attach().await;
        let mp = mp.clone();

        let dest_dir = dest_dir.clone();
        let base_url = format!("{host}/{base_path}");

        handles.push(tokio::spawn(async move {
            let helper = ResourceHelper::new(resource)
                .with_progress_bar()
                .with_multi_progress(mp);

            wuwa_dl::while_err! {
                helper.download(&base_url, dest_dir.to_str().unwrap()).await
            };

            pool.detattch().await;
        }));
    }

    wuwa_dl::wait_all!(handles, 1);

    println!("All the resources are downloaded!");
    println!("Press any key to continue...");

    Ok(Term::stdout().read_key().map(|_| ())?)
}

mod cli;
