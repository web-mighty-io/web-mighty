use crate::actor;
use crate::actor::{Database, Hub, Mail};
use actix::prelude::*;
use actix_web::web;
use deadpool_postgres::Pool;
use handlebars::Handlebars;
use ignore::WalkBuilder;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR};
use walkdir::WalkDir;
#[cfg(feature = "watch-file")]
use {
    notify::{raw_watcher, RawEvent, RecommendedWatcher, RecursiveMode, Watcher},
    std::path::PathBuf,
    std::process::Command,
    std::sync::mpsc::{channel, Receiver},
    std::sync::{Mutex, MutexGuard},
    std::thread,
};

/// AppState of this server
///
/// 1. `handlebars`: Needs for rendering page. Can get from calling `get_handlebars()`.
/// 2. `watcher`: Just to be alive whole time. (available in `watch-file` feature)
/// 3. `resources`: Needs for resource files. All file except `.hbs` file is saved here. Can get from calling `get_resources()`.
/// 4. `hub`: Websocket main server address.
pub struct AppState {
    #[cfg(not(feature = "watch-file"))]
    handlebars: Handlebars<'static>,
    #[cfg(feature = "watch-file")]
    handlebars: Mutex<Handlebars<'static>>,
    #[cfg(feature = "watch-file")]
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    #[cfg(not(feature = "watch-file"))]
    resources: HashMap<String, Vec<u8>>,
    #[cfg(feature = "watch-file")]
    resources: Mutex<HashMap<String, Vec<u8>>>,
    pub hub: Addr<Hub>,
    pub db: Addr<Database>,
    pub mail: Addr<Mail>,
}

impl AppState {
    #[cfg(not(feature = "watch-file"))]
    pub fn new<P: AsRef<Path>>(path: P, pool: Pool, mail: Mail) -> web::Data<AppState> {
        let db = actor::Database::new(pool).start();
        web::Data::new(AppState {
            handlebars: make_handlebars(&path),
            resources: get_resources(&path),
            hub: Hub::new(db.clone()).start(),
            db,
            mail: mail.start(),
        })
    }

    #[cfg(feature = "watch-file")]
    pub fn new<P: AsRef<Path>>(path: P, pool: Pool, mail: Mail) -> web::Data<AppState> {
        let path = path.as_ref();
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();

        watcher.watch(path, RecursiveMode::Recursive).unwrap();

        let db = actor::Database::new(pool).start();
        let state = web::Data::new(AppState {
            handlebars: Mutex::new(make_handlebars(&path)),
            watcher,
            resources: Mutex::new(get_resources(&path)),
            hub: actor::Hub::new(db.clone()).start(),
            db,
            mail: mail.start(),
        });
        let state_clone = state.clone();
        let path_clone = path.to_path_buf();

        thread::spawn(move || watch(state_clone, rx, path_clone));

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

    #[cfg(not(feature = "watch-file"))]
    pub fn get_resources(&self) -> &HashMap<String, Vec<u8>> {
        &self.resources
    }

    #[cfg(feature = "watch-file")]
    pub fn get_resources(&self) -> MutexGuard<'_, HashMap<String, Vec<u8>>> {
        self.resources.lock().unwrap()
    }
}

fn make_handlebars<P: AsRef<Path>>(path: P) -> Handlebars<'static> {
    let path = path.as_ref();
    let mut handlebars = Handlebars::new();

    for entry in WalkDir::new(path).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name().to_string_lossy().ends_with(".hbs") {
            handlebars
                .register_template_file(
                    &*entry
                        .path()
                        .strip_prefix(path)
                        .unwrap()
                        .to_string_lossy()
                        .replace(MAIN_SEPARATOR, "/"),
                    entry.path(),
                )
                .unwrap();
        }
    }

    handlebars
}

fn get_resources<P: AsRef<Path>>(path: P) -> HashMap<String, Vec<u8>> {
    let path = path.as_ref();
    let mut resources = HashMap::new();

    for entry in WalkBuilder::new(path.join("res"))
        .hidden(false)
        .git_global(false)
        .git_ignore(false)
        .git_exclude(false)
        .require_git(false)
        .ignore_case_insensitive(true)
        .parents(false)
        .build()
        .filter_map(|e| e.ok())
    {
        if let Ok(content) = fs::read(entry.path()) {
            resources.insert(
                (&*entry.path().strip_prefix(path.join("res")).unwrap().to_string_lossy())
                    .to_owned()
                    .replace(MAIN_SEPARATOR, "/"),
                content,
            );
        }
    }

    resources
}

#[cfg(feature = "watch-file")]
fn watch(data: web::Data<AppState>, rx: Receiver<RawEvent>, root: PathBuf) -> ! {
    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(_),
                cookie: _,
            }) => {
                log::info!("file {:?} changed", path);
                let stripped_path = &*path.strip_prefix(&root).unwrap().to_string_lossy();
                if let Some(ext) = path.extension() {
                    if ext == "hbs" {
                        let mut handlebars = data.handlebars.lock().unwrap();
                        handlebars
                            .register_template_file(&*stripped_path.replace(MAIN_SEPARATOR, "/"), &path)
                            .unwrap();
                        drop(handlebars);
                        continue;
                    }

                    if ext == "scss" {
                        let _ = Command::new("sass")
                            .arg(&path)
                            .arg(
                                root.join(
                                    path.strip_prefix(&root)
                                        .unwrap()
                                        .to_string_lossy()
                                        .replace("scss", "css"),
                                ),
                            )
                            .output();
                    }
                }

                if let Ok(stripped_path) = path.strip_prefix(root.join("res")) {
                    let stripped_path = &*stripped_path.to_string_lossy();
                    let stripped_path = stripped_path.to_owned().replace(MAIN_SEPARATOR, "/");
                    let mut resources = data.resources.lock().unwrap();
                    if resources.contains_key(&stripped_path) {
                        if let Ok(content) = fs::read(&path) {
                            resources.insert(stripped_path, content);
                        }
                    }
                    drop(resources);
                }
            }
            Ok(event) => log::warn!("broken event: {:?}", event),
            Err(e) => log::error!("file watch error: {:?}", e.to_string()),
        }
    }
}
