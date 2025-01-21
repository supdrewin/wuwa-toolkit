use std::{env, sync::Arc};

use console::Term;
use indicatif::MultiProgress;
use tokio::runtime::Builder;
use wuwa_dl::{
    cli::Cli,
    helper::{resource::ResourceHelper, ResourceHelperExt},
    json::{index::IndexJson, resource::ResourceJson},
    pool::{Pool, PoolOp},
    utils::{Result, INDEX_JSON_URL},
};

fn main() -> Result<()> {
    let mut rt = Builder::new_multi_thread();
    let cli = Cli::new();

    let rt = match cli.threads {
        Some(threads) => rt.worker_threads(threads),
        None => &mut rt,
    }
    .enable_all()
    .build()?;

    rt.block_on(async {
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

        let mut pool = Pool::new()?;
        let mut tasks = vec![];

        for resource in resource_json.resource {
            let dest_dir = dest_dir.clone();
            let base_url = format!("{host}/{base_path}");

            let sender = pool.sender.clone();
            let mp = mp.clone();

            pool.watcher.changed().await?;
            sender.send(PoolOp::Attach).await?;

            tasks.push(rt.spawn(async move {
                let helper = ResourceHelper::new(resource, &base_url, dest_dir.to_str().unwrap())
                    .with_progress_bar()
                    .with_multi_progress(mp);

                wuwa_dl::while_err! { helper.download().await }
                sender.send(PoolOp::Dettach).await
            }));
        }

        wuwa_dl::wait_all!(tasks, 2);

        println!("All the resources are downloaded!");
        println!("Press any key to continue...");

        Ok(Term::stdout().read_key().map(|_| ())?)
    })
}
