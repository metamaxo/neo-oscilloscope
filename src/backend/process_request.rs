use crate::audio_to_coords::request::Request as AudioToCoordsRequest;
use crate::backend::settings::Settings;
use crate::coords_to_audio::request::Request as CoordsToAudioRequest;
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
        mode: settings.mode,
        image: img,
        threshold: settings.threshold,
        pix_threshold: settings.pix_threshold,
        result: Vec::new(),
        interpolate: settings.int_amount > 0,
        int_amount: settings.int_amount,
        size: settings.size,
        edge_detection: settings.edge_detection,
    }
}

pub fn image_to_black_coords_request<Im>(settings: &Settings, img: Im) -> ImageToCoordsRequest<Im>
where
    Im: std::ops::Deref<Target = GrayImage>,
{
    ImageToCoordsRequest {
        mode: settings.mode,
        image: img,
        threshold: settings.threshold,
        pix_threshold: 0,
        result: Vec::new(),
        interpolate: false,
        int_amount: 0,
        size: settings.size,
        edge_detection: false,
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
