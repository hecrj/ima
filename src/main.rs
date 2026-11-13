use iced::widget::image;
use iced::{Fill, Widget};

use anywho::anywho;

use std::env;

fn main() -> anywho::Result<()> {
    let Some(path) = env::args().skip(1).next() else {
        return Err(anywho!("image path not provided"));
    };

    let image = ::image::ImageReader::open(path)?.decode()?.to_rgba8();
    let handle = image::Handle::from_rgba(image.width(), image.height(), image.into_raw());

    iced::application(move || Ima::new(&handle), Ima::update, Ima::view).run()?;

    Ok(())
}

struct Ima {
    handle: image::Handle,
}

impl Ima {
    fn new(handle: &image::Handle) -> Self {
        Self {
            handle: handle.clone(),
        }
    }

    fn update(&mut self, message: !) {
        match message {}
    }

    fn view(&self) -> impl Widget<!> {
        image(&self.handle).width(Fill).height(Fill)
    }
}
