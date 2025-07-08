use crate::traits::OperationTrait;
use anyhow::Result;
use futures::{
    StreamExt as _,
    channel::mpsc::{Receiver, Sender},
};
use process_request::ProcessRequest;
use processing::ProcessArgs;

pub mod process_request;
pub mod processing;
pub mod settings;
pub mod state;

pub enum Request {
    Settings(Box<dyn OperationTrait<State = settings::Settings>>),
    State(Box<dyn OperationTrait<State = state::State>>),
    Process(processing::ProcessArgs),
}

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Request::Settings(_) => write!(f, "Request::Settings"),
            Request::State(_) => write!(f, "Request::State"),
            Request::Process(args) => write!(f, "Request::Process({:?})", args),
        }
    }
}

impl From<Box<dyn OperationTrait<State = settings::Settings>>> for Request {
    fn from(op: Box<dyn OperationTrait<State = settings::Settings>>) -> Self {
        Request::Settings(op)
    }
}

impl From<Box<dyn OperationTrait<State = state::State>>> for Request {
    fn from(op: Box<dyn OperationTrait<State = state::State>>) -> Self {
        Request::State(op)
    }
}

pub struct Backend {
    pub rx: Receiver<Request>,
    pub self_tx: Sender<Request>,

    pub settings: settings::Settings,
    pub state: state::State,
}

impl Backend {
    pub fn new(self_tx: Sender<Request>, rx: Receiver<Request>) -> Backend {
        Backend {
            rx,
            self_tx,
            settings: settings::Settings::default(),
            state: state::State::default(),
        }
    }

    /// Start the backend with a receiver and a sender for requests
    pub fn start(rx: Receiver<Request>, self_tx: Sender<Request>) {
        wasm_bindgen_futures::spawn_local(Self::new(self_tx, rx).run());
    }

    /// Run the backend, processing requests and handling periodic tasks
    pub async fn run(mut self) {
        while let Some(request) = self.rx.next().await {
            if let Err(e) = self.handle_request(request).await {
                tracing::error!("Error handling request: {:?}", e);
            }
        }
    }

    /// Handle incoming requests
    pub async fn handle_request(&mut self, request: Request) -> Result<()> {
        tracing::debug!("Handling request: {:?}", request);
        use Request::*;
        match request {
            Settings(req) => self.settings.handle_request(req).await,
            State(req) => self.state.handle_request(req).await,
            Process(args) => self.handle_processing(args).await,
        }
    }

    pub async fn handle_processing(&mut self, args: ProcessArgs) -> Result<()> {
        match args.request {
            ProcessRequest::ImageToCoords => processing::image_to_coords(self, args),
            ProcessRequest::AudioToCoords => processing::audio_to_coords(self, args),
            ProcessRequest::CoordsToAudio => processing::coords_to_audio(self, args),
            ProcessRequest::ImageToBlackCoords => processing::image_to_black_coords(self, args),
        }
    }
}
