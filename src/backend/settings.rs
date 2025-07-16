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
    pub edge_threshold: u8,
    pub pix_threshold: u32,
    pub sample_rate: u32,
    pub edge_detection: bool,
    pub size: u32,
    pub spread_type: u32,
    pub starting_point: (f64, f64),
    pub directions: Option<Vec<u32>>,
    pub canvas_size: u32,
    pub scan_type: u32,
    pub scanline_type: u32,
    pub snake_step_amount: u32,
    pub horizontal: bool,
    pub scramble: bool,
    pub flatten: bool,
    pub double_trace: bool,
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
    pub center_x: f64,
    pub center_y: f64,
    pub clip_length: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            method: Method::Full,
            int_amount: 20,
            threshold: 20,
            edge_threshold: 2,
            pix_threshold: 20,
            spread_type: 1,
            sample_rate: 44100,
            repeat: 1,
            loop_audio: true,
            playback_rate: 1.0,
            scan_type: 1,
            scanline_type: 2,
            horizontal: true,
            snake_step_amount: 1,
            scramble: false,
            flatten: false,
            double_trace: false,
            size: 600,
            edge_detection: true,
            canvas_size: 600,
            line_color: String::from("#000000"),
            dot_mode: false,
            scale: 300.0,
            stroke: 1.0,
            persistence: 0.05,
            hue: 140.0,
            image_opacity: 0.0,
            noise: 0.0,
            center_x: 300.0,
            center_y: 300.0,
            clip_length: 10.0,
            starting_point: (0.0, 0.0),
            directions: None,
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
        let value = self.0;
        let op = Operation {
            handler: Box::new(move |settings: &mut Settings| {
                settings.method = value;
                log::info!("setting method to: {}", settings.method);
                settings.method
            }),
            sender: tx,
        };

        (Box::new(op), rx)
    }
}

pub struct SetStartingPoint(pub (f64, f64));

impl RequestTrait for SetStartingPoint {
    type State = Settings;
    type Output = (f64, f64);

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Settings>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        let (tx, rx) = futures::channel::oneshot::channel();
        let value = self.0;
        let op = Operation {
            handler: Box::new(move |settings: &mut Settings| {
                settings.starting_point = value;
                log::info!("setting starting point to: {:?}", settings.starting_point);
                settings.starting_point
            }),
            sender: tx,
        };

        (Box::new(op), rx)
    }
}

pub struct SetDirections(pub Option<Vec<u32>>);

impl RequestTrait for SetDirections {
    type State = Settings;
    type Output = Option<Vec<u32>>;

    fn into_operation(
        self,
    ) -> (
        Box<dyn OperationTrait<State = Settings>>,
        futures::channel::oneshot::Receiver<Self::Output>,
    ) {
        let (tx, rx) = futures::channel::oneshot::channel();
        let value = self.0;
        let op = Operation {
            handler: Box::new(move |settings: &mut Settings| {
                settings.directions = value.clone();
                log::info!("setting directions to: {:?}", settings.directions);
                settings.directions.clone()
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
    edge_threshold: u8,
    pix_threshold: u32,
    spread_type: u32,
    sample_rate: u32,
    repeat: u32,
    playback_rate: f32,
    edge_detection: bool,
    size: u32,
    dot_mode: bool,
    scale: f64,
    stroke: f64,
    persistence: f64,
    hue: f64,
    image_opacity: f64,
    noise: f64,
    center_x: f64,
    center_y: f64,
    clip_length: f64,
    scan_type: u32,
    scanline_type: u32,
    snake_step_amount: u32,
    horizontal: bool,
    scramble: bool,
    double_trace: bool,
    flatten: bool,
}

impl Settings {
    pub async fn handle_request(
        &mut self,
        request: Box<dyn OperationTrait<State = Self>>,
    ) -> Result<()> {
        request.execute(self)
    }
}
