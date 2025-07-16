use crate::audio_to_coords::request::Request as AudioToCoordsRequest;
use crate::backend::settings::Settings;
use crate::coords_to_audio::request::Request as CoordsToAudioRequest;
use crate::image_to_coords::method::Method;
use crate::image_to_coords::request::Request as ImageToCoordsRequest;
use image::GrayImage;

#[derive(Debug, Clone)]
pub enum ProcessRequest {
    ImageToCoords,
    ImageToBlackCoords,
    AudioToCoords,
    CoordsToAudio,
}

pub fn image_to_coords_request<Im>(settings: &Settings, img: Im) -> ImageToCoordsRequest<Im>
where
    Im: std::ops::Deref<Target = GrayImage>,
{
    ImageToCoordsRequest {
        method: settings.method,
        image: img,
        threshold: settings.threshold,
        pix_threshold: settings.pix_threshold,
        spread_type: settings.spread_type,
        result: Vec::new(),
        interpolate: settings.int_amount > 0,
        int_amount: settings.int_amount,
        size: settings.size,
        edge_detection: settings.edge_detection,
        canvas_size: settings.canvas_size,
        scan_type: settings.scan_type,
        scanline_type: settings.scanline_type,
        starting_point: settings.starting_point,
        snake_step_amount: settings.snake_step_amount,
        directions: settings.directions.clone(),
        horizontal: settings.horizontal,
        scramble: settings.scramble,
        double_trace: settings.double_trace,
        edge_threshold: settings.edge_threshold,
        flatten: settings.flatten,
    }
}

pub fn image_to_black_coords_request<Im>(settings: &Settings, img: Im) -> ImageToCoordsRequest<Im>
where
    Im: std::ops::Deref<Target = GrayImage>,
{
    ImageToCoordsRequest {
        method: Method::Black,
        image: img,
        threshold: settings.threshold,
        pix_threshold: 0,
        spread_type: settings.spread_type,
        result: Vec::new(),
        interpolate: false,
        int_amount: 0,
        size: settings.size,
        edge_detection: false,
        canvas_size: settings.canvas_size,
        starting_point: settings.starting_point,
        snake_step_amount: 1,
        scan_type: 1,
        scanline_type: 1,
        directions: None,
        horizontal: true,
        scramble: false,
        double_trace: false,
        edge_threshold: settings.edge_threshold,
        flatten: false,
    }
}

pub fn coords_to_audio_request<C>(settings: &Settings, coords: C) -> CoordsToAudioRequest<C>
where
    C: std::ops::Deref<Target = Vec<(f32, f32)>>,
{
    CoordsToAudioRequest {
        coords,
        sample_rate: settings.sample_rate,
        playback_rate: settings.playback_rate,
        result: Vec::new(),
    }
}

pub fn audio_to_coords_request<A>(audio: A) -> AudioToCoordsRequest<A>
where
    A: std::ops::Deref<Target = Vec<u8>>,
{
    AudioToCoordsRequest {
        audio,
        result: Vec::new(),
    }
}
