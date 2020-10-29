use actix_web::web;
use handlebars::Handlebars;
use std::path::PathBuf;
use walkdir::WalkDir;
#[cfg(feature = "watch-file")]
use {
    notify::{raw_watcher, RawEvent, RecommendedWatcher, RecursiveMode, Watcher},
    std::sync::mpsc::{channel, Receiver},
    std::sync::{Mutex, MutexGuard},
    std::thread,
};

pub struct AppState {
    #[cfg(not(feature = "watch-file"))]
    handlebars: Handlebars<'static>,
    #[cfg(feature = "watch-file")]
    handlebars: Mutex<Handlebars<'static>>,
    #[cfg(feature = "watch-file")]
    #[warn(dead_code)]
    watcher: RecommendedWatcher,
}

impl AppState {
    #[cfg(not(feature = "watch-file"))]
    pub fn new(path: PathBuf) -> web::Data<AppState> {
        web::Data::new(AppState {
            handlebars: new_handlebars(path),
        })
    }

    #[cfg(feature = "watch-file")]
    pub fn new(path: PathBuf) -> web::Data<AppState> {
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();

        watcher
            .watch(path.as_path(), RecursiveMode::Recursive)
            .unwrap();

        let state = web::Data::new(AppState {
            handlebars: Mutex::new(new_handlebars(path)),
            watcher,
        });
        let state_clone = state.clone();

        thread::spawn(move || watch(state_clone, rx));

        state
    }

    #[cfg(not(feature = "watch-file"))]
    pub fn get_handlebars(&self) -> &Handlebars<'static> {
        &self.handlebars
    }

    #[cfg(feature = "watch-file")]
    pub fn get_handlebars(&self) -> MutexGuard<'_, Handlebars<'static>> {
        self.handlebars.lock().unwrap()
    }
}

fn new_handlebars(path: PathBuf) -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    for entry in WalkDir::new(path.as_path())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name().to_string_lossy().ends_with(".hbs") {
            handlebars
                .register_template_file(
                    &*entry
                        .path()
                        .strip_prefix(path.as_path())
                        .unwrap()
                        .to_string_lossy(),
                    entry.path(),
                )
                .unwrap();
        }
    }

    handlebars
}

#[cfg(feature = "watch-file")]
fn watch(data: web::Data<AppState>, rx: Receiver<RawEvent>) {
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(_),
                cookie: _,
            }) => {
                if let Some(ext) = path.extension() {
                    if ext == "hbs" {
                        log::info!("file {:?} changed", path);
                        let mut handlebars = data.handlebars.lock().unwrap();
                        handlebars
                            .register_template_file(
                                &*path.clone().file_name().unwrap().to_string_lossy(),
                                path,
                            )
                            .unwrap();
                    }
                }
            }
            Ok(event) => log::warn!("broken event: {:?}", event),
            Err(e) => log::error!("file watch error: {:?}", e.to_string()),
        }
    }
}
