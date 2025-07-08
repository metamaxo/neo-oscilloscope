use anyhow::Result;
use std::sync::Arc;

use image::GrayImage;

use crate::traits::{Operation, OperationTrait, RequestTrait};

#[derive(Default, Debug)]
pub struct State {
    image: Option<Arc<GrayImage>>,
    audio: Option<Arc<Vec<u8>>>,
    coords: Option<Arc<Vec<(f32, f32)>>>,
    black_coords: Option<Arc<Vec<(f32, f32)>>>,
}

impl State {
    /// NOTE: This returns a clone of the `Arc`, so very light and cheap to call.
    /// It does not block or wait for any processing to finish.
    pub fn get_image(&self) -> Option<Arc<GrayImage>> {
        self.image.clone()
    }

    /// NOTE: This replaces the current image `Arc` with the new one,
    /// Even if there is some processing step running that is using the old image.
    /// This is totally fine and safe, because the `Arc` ensures that the old image
    /// is not dropped until all references to it are gone.
    pub fn set_image(&mut self, image: Arc<GrayImage>) -> Option<Arc<GrayImage>> {
        let old_image = self.image.take();
        self.image = Some(image);
        old_image
    }

    pub fn get_audio(&self) -> Option<Arc<Vec<u8>>> {
        self.audio.clone()
    }

    pub fn set_audio(&mut self, audio: Arc<Vec<u8>>) -> Option<Arc<Vec<u8>>> {
        let old_audio = self.audio.take();
        self.audio = Some(audio);
        old_audio
    }

    pub fn get_coords(&mut self) -> Option<Arc<Vec<(f32, f32)>>> {
        self.coords.clone()
    }

    pub fn set_coords(&mut self, coords: Arc<Vec<(f32, f32)>>) -> Option<Arc<Vec<(f32, f32)>>> {
        let old_coords = self.coords.take();
        self.coords = Some(coords);
        old_coords
    }

    pub fn get_black_coords(&mut self) -> Option<Arc<Vec<(f32, f32)>>> {
        self.black_coords.clone()
    }

    pub fn set_black_coords(
        &mut self,
        black_coords: Arc<Vec<(f32, f32)>>,
    ) -> Option<Arc<Vec<(f32, f32)>>> {
        let old_black_coords = self.black_coords.take();
        self.black_coords = Some(black_coords);
        old_black_coords
    }

    pub async fn handle_request(
        &mut self,
        request: Box<dyn OperationTrait<State = Self>>,
    ) -> Result<()> {
        request.execute(self)
    }
}

pub struct GetImage;

impl RequestTrait for GetImage {
    type State = State;
    type Output = Option<Arc<GrayImage>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("getting image");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(|state: &mut State| state.get_image()),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct SetImage(pub Arc<GrayImage>);

impl RequestTrait for SetImage {
    type State = State;
    type Output = Option<Arc<GrayImage>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("settting image");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(move |state: &mut State| state.set_image(self.0.clone())),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct GetAudio;

impl RequestTrait for GetAudio {
    type State = State;
    type Output = Option<Arc<Vec<u8>>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("getting audio");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(|state: &mut State| state.get_audio()),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct SetAudio(pub Arc<Vec<u8>>);

impl RequestTrait for SetAudio {
    type State = State;
    type Output = Option<Arc<Vec<u8>>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("setting audio");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(move |state: &mut State| {
                state.set_audio(self.0.clone());
                state.audio.clone()
            }),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct GetCoords;

impl RequestTrait for GetCoords {
    type State = State;
    type Output = Option<Arc<Vec<(f32, f32)>>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("gettting coords");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(|state: &mut State| state.get_coords()),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct SetCoords(pub Arc<Vec<(f32, f32)>>);

impl RequestTrait for SetCoords {
    type State = State;
    type Output = Option<Arc<Vec<(f32, f32)>>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("setting coords");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(move |state: &mut State| {
                state.set_coords(self.0.clone());
                state.coords.clone()
            }),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct GetBlackCoords;

impl RequestTrait for GetBlackCoords {
    type State = State;
    type Output = Option<Arc<Vec<(f32, f32)>>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("gettting black coords");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(|state: &mut State| state.get_black_coords()),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}

pub struct SetBlackCoords(pub Arc<Vec<(f32, f32)>>);

impl RequestTrait for SetBlackCoords {
    type State = State;
    type Output = Option<Arc<Vec<(f32, f32)>>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("setting black coords");
        let (tx, rx) = futures::channel::oneshot::channel();
        let op = Operation {
            handler: Box::new(move |state: &mut State| {
                state.set_black_coords(self.0.clone());
                state.black_coords.clone()
            }),
            sender: tx,
        };
        (Box::new(op), rx)
    }
}
