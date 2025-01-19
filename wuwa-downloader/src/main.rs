use std::{env, sync::Arc};

use console::Term;
use indicatif::MultiProgress;
use tokio::runtime::Builder;
use wuwa_dl::{
    cli::Cli,
    helper::ResourceHelper,
    json::{index::IndexJson, resource::ResourceJson},
    utils::{PoolOp, Result, INDEX_JSON_URL},
};

fn main() -> Result<()> {
    let mut rt = Builder::new_multi_thread();
    let cli = Cli::new();

    let rt = match cli.threads {
        Some(threads) => rt.worker_threads(threads),
        _ => &mut rt,
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

        let mut tasks = vec![];

        let (tx1, mut rx1) = tokio::sync::mpsc::channel::<PoolOp>(1);
        let (tx2, mut rx2) = tokio::sync::watch::channel(0);

        tx2.send(rt.metrics().num_workers() - 1)?;

        tokio::spawn(async move {
            while let Some(op) = rx1.recv().await {
                tx2.send_if_modified(|num| {
                    let result = num.checked_add_signed(op as isize);

                    match result {
                        Some(n) => *num = n,
                        None => (),
                    }

                    result.is_some()
                });
            }

            Result::Ok(())
        });

        for resource in resource_json.resource {
            let mp = mp.clone();
            let tx1 = tx1.clone();

            rx2.changed().await?;
            tx1.send(PoolOp::Attach).await?;

            let dest_dir = dest_dir.clone();
            let base_url = format!("{host}/{base_path}");

            tasks.push(rt.spawn(async move {
                let helper = ResourceHelper::new(resource)
                    .with_progress_bar()
                    .with_multi_progress(mp);

                wuwa_dl::while_err! {
                    helper.download(&base_url, dest_dir.to_str().unwrap()).await
                };

                tx1.send(PoolOp::Dettach).await
            }));
        }

        wuwa_dl::wait_all!(tasks, 2);

        println!("All the resources are downloaded!");
        println!("Press any key to continue...");

        Ok(Term::stdout().read_key().map(|_| ())?)
    })
}
