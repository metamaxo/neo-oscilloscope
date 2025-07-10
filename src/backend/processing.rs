use crate::backend::state;
use std::sync::Arc;

use crate::backend::process_request::*;
use crate::backend::{self, Backend};
use crate::traits::RequestTrait as _;
use anyhow::Result;
use futures::SinkExt as _;
use log::info;

#[derive(Debug, Clone)]
pub struct ProcessArgs {
    pub request: ProcessRequest,
}

impl Default for ProcessArgs {
    fn default() -> Self {
        ProcessArgs {
            request: ProcessRequest::CoordsToAudio,
        }
    }
}

pub fn image_to_coords(backend: &mut Backend, args: ProcessArgs) -> Result<()> {
    let Some(image) = backend.state.get_image() else {
        return Err(anyhow::anyhow!(
            "No image set in state, cannot process coords."
        ));
    };
    let mut backend_tx = backend.self_tx.clone();
    let settings = backend.settings.clone();
    wasm_bindgen_futures::spawn_local(async move {
        info!("Processing image to coords with args: {:?}", args);
        // Create the request to process the image to audio.
        let mut request = image_to_coords_request(&settings, image.clone());
        request.process();
        let coords = Arc::new(request.result);
        let request = state::SetCoords(coords);
        let (operation, receiver) = request.into_operation();
        if let Err(e) = backend_tx.send(backend::Request::State(operation)).await {
            tracing::error!("Failed to send image to coords processing result: {:?}", e);
            return;
        }
        if let Err(e) = receiver.await {
            tracing::error!("Error receiving response: {:?}", e);
        }
    });
    Ok(())
}

pub fn image_to_black_coords(backend: &mut Backend, args: ProcessArgs) -> Result<()> {
    let Some(image) = backend.state.get_image() else {
        return Err(anyhow::anyhow!(
            "No image set in state, cannot process coords."
        ));
    };
    let mut backend_tx = backend.self_tx.clone();
    let settings = backend.settings.clone();
    wasm_bindgen_futures::spawn_local(async move {
        info!("Processing image to coords with args: {:?}", args);
        // Create the request to process the image to audio.
        let mut request = image_to_black_coords_request(&settings, image.clone());
        request.process();
        let coords = Arc::new(request.result);
        let request = state::SetBlackCoords(coords);
        let (operation, receiver) = request.into_operation();
        if let Err(e) = backend_tx.send(backend::Request::State(operation)).await {
            tracing::error!("Failed to send image to coords processing result: {:?}", e);
            return;
        }
        if let Err(e) = receiver.await {
            tracing::error!("Error receiving response: {:?}", e);
        }
    });
    Ok(())
}

pub fn coords_to_audio(backend: &mut Backend, args: ProcessArgs) -> Result<()> {
    let Some(coords) = backend.state.get_coords() else {
        return Err(anyhow::anyhow!(
            "No coords set in state, cannot process audio."
        ));
    };

    let settings = backend.settings.clone();
    let mut backend_tx = backend.self_tx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        info!("Processing coords to audio with args: {:?}", args);
        let mut request = coords_to_audio_request(&settings, coords);
        info!("audio processing complete, sending audio to backend");
        request.process();
        let request = state::SetAudio(Arc::new(request.result));
        let (operation, receiver) = request.into_operation();
        if let Err(e) = backend_tx.send(backend::Request::State(operation)).await {
            info!("Failed to send coords to audio processing result {:?}", e)
        }
        if let Err(e) = receiver.await {
            info!("Error receiving resonse: {:?}", e);
        }
    });
    Ok(())
}

pub fn audio_to_coords(backend: &mut Backend, _args: ProcessArgs) -> Result<()> {
    let Some(audio) = backend.state.get_audio() else {
        return Err(anyhow::anyhow!(
            "No audio set in state, cannot process coords."
        ));
    };
    let mut backend_tx = backend.self_tx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        info!("processing audio to coords");
        let mut request = audio_to_coords_request(audio);
        let _ = request.process();
        let request = state::SetCoords(Arc::new(request.result));
        let (operation, receiver) = request.into_operation();
        if let Err(e) = backend_tx.send(backend::Request::State(operation)).await {
            info!("Failed to send audio to coords processing result {:?}", e)
        }
        if let Err(e) = receiver.await {
            info!("Error receiving resonse: {:?}", e);
        }
    });
    Ok(())
}
