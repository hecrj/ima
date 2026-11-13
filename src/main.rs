use iced::futures::channel::mpsc;
use iced::widget::image;
use iced::{Fill, Task, Widget};

use anywho::anywho;

use std::env;
use std::path::Path;
use std::time::Instant;

fn main() -> anywho::Result<()> {
    let Some(path) = env::args().nth(1) else {
        return Err(anywho!("image path not provided"));
    };

    let handle = load(&path)?;

    iced::application(move || Ima::new(&path, &handle), Ima::update, Ima::view).run()?;

    Ok(())
}

struct Ima {
    handle: image::Handle,
    _watcher: notify::RecommendedWatcher,
}

enum Message {
    Changed(image::Handle),
}

impl Ima {
    fn new(path: impl AsRef<Path>, handle: &image::Handle) -> (Self, Task<Message>) {
        let (_watcher, watch) = watch(path);

        (
            Self {
                handle: handle.clone(),
                _watcher,
            },
            Task::run(watch, Message::Changed),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Changed(handle) => {
                self.handle = handle;
            }
        }
    }

    fn view(&self) -> impl Widget<Message> {
        image(&self.handle).width(Fill).height(Fill)
    }
}

fn load(path: impl AsRef<Path>) -> anywho::Result<image::Handle> {
    let image = ::image::ImageReader::open(&path)?.decode()?.to_rgba8();

    Ok(image::Handle::from_rgba(
        image.width(),
        image.height(),
        image.into_raw(),
    ))
}

fn watch(
    path: impl AsRef<Path>,
) -> (
    notify::RecommendedWatcher,
    mpsc::UnboundedReceiver<image::Handle>,
) {
    use notify::Watcher;

    let path = path.as_ref();
    let (sender, receiver) = mpsc::unbounded();

    let mut watcher = {
        let path = path.to_path_buf();
        let mut last_load = Instant::now();

        notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
            let Ok(event) = event else { return };

            if !matches!(
                event.kind,
                notify::EventKind::Modify(
                    notify::event::ModifyKind::Name(_) | notify::event::ModifyKind::Data(_)
                )
            ) {
                return;
            }

            if !event
                .paths
                .iter()
                .any(|candidate| candidate.ends_with(&path))
            {
                return;
            }

            if last_load.elapsed().as_millis() < 250 {
                return;
            }

            last_load = Instant::now();

            let Ok(handle) = load(&path) else {
                return;
            };

            let _ = sender.unbounded_send(handle);
        })
        .expect("create file watcher")
    };

    if let Some(parent) = path
        .parent()
        .map(Path::to_path_buf)
        .or_else(|| env::current_dir().ok())
    {
        let _ = watcher.watch(&parent, notify::RecursiveMode::NonRecursive);
    }

    (watcher, receiver)
}
