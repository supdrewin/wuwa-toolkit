use iocraft::prelude::*;
use wuwa_dl::prelude::*;

use std::{env, sync::Arc};

use console::Term;
use indicatif::MultiProgress;
use tokio::runtime::Builder;

fn main() -> DynResult<()> {
    let mut rt = Builder::new_multi_thread();
    let mut cli = Cli::new();

    if env::args().count() == 1 {
        #[derive(Default, Props)]
        struct Props<'a> {
            global: Option<&'a mut bool>,
            beta: Option<&'a mut bool>,
        }

        impl<'a> Props<'a> {
            fn server_switch(&self) -> String {
                match self.global {
                    Some(&mut global) => {
                        let (cn, os) = if global { (" ", "*") } else { ("*", " ") };
                        format!("  Server Type:   CN [{cn}]   OS [{os}]")
                    }
                    None => todo!(),
                }
            }

            fn resource_switch(&self) -> String {
                match self.beta {
                    Some(&mut beta) => {
                        let (live, beta) = if beta { (" ", "*") } else { ("*", " ") };
                        format!("Resource Type: LIVE [{live}] Beta [{beta}]")
                    }
                    None => todo!(),
                }
            }
        }

        #[component]
        fn Prompt<'a>(props: &mut Props<'a>, mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
            let mut system = hooks.use_context_mut::<SystemContext>();
            let mut finished = hooks.use_state(|| false);

            let mut server_switch = hooks.use_state(|| **props.global.as_ref().unwrap());
            let mut resource_switch = hooks.use_state(|| **props.beta.as_ref().unwrap());

            hooks.use_terminal_events(move |event| match event {
                TerminalEvent::Key(KeyEvent { code, kind, .. })
                    if kind != KeyEventKind::Release =>
                {
                    match code {
                        KeyCode::Char('r') => resource_switch.set(!resource_switch.get()),
                        KeyCode::Char('s') => server_switch.set(!server_switch.get()),
                        KeyCode::Char('x') => finished.set(true),
                        _ => (),
                    }
                }
                _ => (),
            });

            if let Some(global) = props.global.as_mut() {
                **global = server_switch.get();
            }

            if let Some(beta) = props.beta.as_mut() {
                **beta = resource_switch.get();
            }

            finished.get().then(|| system.exit());

            element! {
                View(
                    align_items: AlignItems::Center,
                    border_color: Color::Cyan,
                    border_style: BorderStyle::Round,
                    flex_direction: FlexDirection::Column,
                ) {
                    View(margin_top: -1) {
                        Text(content: " WuWa Downloader ", wrap: TextWrap::NoWrap)
                    }

                    View(
                        flex_direction: FlexDirection::Column,
                        margin: 1,
                    ) {
                        View {
                            Text(content: props.server_switch())
                        }

                        View {
                            Text(content: props.resource_switch())
                        }

                    }

                    View(margin_bottom: -1) {
                        Text(content: " [S]erver [R]esource e[X]it ", wrap: TextWrap::NoWrap)
                    }
                }
            }
        }

        smol::block_on(
            element!(Prompt(
                global: &mut cli.global,
                beta: &mut cli.beta,
            ))
            .render_loop(),
        )?;
    }

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

            wuwa_dl::while_err! { pool.watcher.changed().await }
            wuwa_dl::while_err! { sender.send(PoolOp::Attach).await }

            tasks.push(rt.spawn(async move {
                let helper = ResourceHelper::new(resource, &base_url, dest_dir.to_str().unwrap())
                    .with_progress_bar()
                    .with_multi_progress(mp);

                wuwa_dl::while_err! { helper.download().await }
                wuwa_dl::while_err! { sender.send(PoolOp::Dettach).await }
            }));
        }

        wuwa_dl::wait_all!(tasks, 1);

        println!("All the resources are downloaded!");
        println!("Press any key to continue...");

        Ok(Term::stdout().read_key().map(|_| ())?)
    })
}
