use crate::image_to_coords::method::Method;
use crate::traits::{Operation, OperationTrait, RequestTrait};
use anyhow::Result;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    // Processing
    pub method: Method,
    pub int_amount: usize,
    pub threshold: u8,
    pub pix_threshold: u32,
    pub sample_rate: u32,
    pub edge_detection: bool,
    pub size: u32,
    pub spread_type: u32,
    pub starting_point: (f64, f64),
    // Front end
    pub loop_audio: bool,
    pub repeat: u32,
    pub playback_rate: f32,
    pub dot_mode: bool,
    pub scale: f64,
    pub stroke: f64,
    pub line_color: String,
    pub persistence: f64,
    pub hue: f64,
    pub image_opacity: f64,
    pub noise: f64,
    pub centerx: f64,
    pub centery: f64,
    pub clip_length: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            method: Method::Full,
            int_amount: 20,
            threshold: 20,
            pix_threshold: 20,
            spread_type: 1,
            sample_rate: 44100,
            repeat: 1,
            loop_audio: true,
            playback_rate: 1.0,
            size: 600,
            edge_detection: true,
            line_color: String::from("#000000"),
            dot_mode: false,
            scale: 300.0,
            stroke: 1.0,
            persistence: 0.05,
            hue: 140.0,
            image_opacity: 0.0,
            noise: 0.0,
            centerx: 300.0,
            centery: 300.0,
            clip_length: 10.0,
            starting_point: (0.0, 0.0),
        }
    }
}
pub struct GetSettings;

impl RequestTrait for GetSettings {
    type State = Settings;
    type Output = Settings;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        let (tx, rx) = futures::channel::oneshot::channel();

        let op = Operation {
            handler: Box::new(|settings: &mut Settings| settings.clone()),
            sender: tx,
        };

        (Box::new(op), rx)
    }
}

pub struct GetMethod;

impl RequestTrait for GetMethod {
    type State = Settings;
    type Output = Method;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Self::State>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        log::info!("setting method");
        let (tx, rx) = futures::channel::oneshot::channel();

        let op = Operation {
            handler: Box::new(|settings: &mut Settings| settings.method),
            sender: tx,
        };

        (Box::new(op), rx)
    }
}

pub struct SetMethod(pub Method);

impl RequestTrait for SetMethod {
    type State = Settings;
    type Output = Method;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Settings>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        let (tx, rx) = futures::channel::oneshot::channel();
        let value = self.0.clone();
        let op = Operation {
            handler: Box::new(move |settings: &mut Settings| {
                settings.method = value.clone();
                log::info!("setting method to: {}", settings.method);
                settings.method
            }),
            sender: tx,
        };

        (Box::new(op), rx)
    }
}

macro_rules! define_setting_requests {
    ($($field:ident: $type:ty),* $(,)?) => {
        paste::paste! {
            $(
                // Get request
                pub struct [<Get $field:camel>];

                impl RequestTrait for [<Get $field:camel>] {
                    type State = Settings;
                    type Output = $type;
                    fn into_operation(self) -> (Box<dyn OperationTrait<State=Settings>>, futures::channel::oneshot::Receiver<Self::Output>) {
                        let (tx, rx) = futures::channel::oneshot::channel();
                        let op = Operation {
                            handler: Box::new(|settings: &mut Settings| settings.$field),
                            sender: tx,
                        };
                        (Box::new(op), rx)
                    }
                }

                // Set request
                pub struct [<Set $field:camel>](pub $type);

                impl RequestTrait for [<Set $field:camel>] {
                    type State = Settings;
                    type Output = $type;
                    fn into_operation(self) -> (Box<dyn OperationTrait<State=Settings>>, futures::channel::oneshot::Receiver<Self::Output>) {
                        let (tx, rx) = futures::channel::oneshot::channel();
                        let value = self.0;
                        let op = Operation {
                            handler: Box::new(move |settings: &mut Settings| {
                                settings.$field = value;
                                settings.$field
                            }),
                            sender: tx,
                        };
                        (Box::new(op), rx)
                    }
                }
            )*
        }
    };
}

// Generate all the request types with one macro call
define_setting_requests! {
    int_amount: usize,
    threshold: u8,
    pix_threshold: u32,
    spread_type: u32,
    sample_rate: u32,
    repeat: u32,
    playback_rate: f32,
    edge_detection: bool,
    starting_point: (f64, f64),
    size: u32,
    dot_mode: bool,
    scale: f64,
    stroke: f64,
    persistence: f64,
    hue: f64,
    image_opacity: f64,
    noise: f64,
    centerx: f64,
    centery: f64,
    clip_length: f64,
}

impl Settings {
    pub async fn handle_request(
        &mut self,
        request: Box<dyn OperationTrait<State = Self>>,
    ) -> Result<()> {
        request.execute(self)
    }
}
