use crate::JsInterface;
use crate::backend::process_request::ProcessRequest;
use crate::backend::processing;
use crate::to_js;
use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl JsInterface {
    #[wasm_bindgen]
    pub async fn process_image_to_coords(&mut self) -> Result<(), JsValue> {
        info!("processing image to coords");
        let args = processing::ProcessArgs {
            request: ProcessRequest::ImageToCoords,
        };
        self.inner.process(args).await.map_err(to_js)?;
        info!("processing image ok");
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn process_image_to_black_coords(&mut self) -> Result<(), JsValue> {
        info!("processing image to black coords");
        let args = processing::ProcessArgs {
            request: ProcessRequest::ImageToBlackCoords,
        };
        self.inner.process(args).await.map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn process_audio_to_coords(&mut self) -> Result<(), JsValue> {
        info!("processing audio to coords");
        let args = processing::ProcessArgs {
            request: ProcessRequest::AudioToCoords,
        };
        self.inner.process(args).await.map_err(to_js)?;
        info!("processing image ok");
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn process_coords_to_audio(&mut self) -> Result<(), JsValue> {
        info!("processing coords to audio");
        let args = processing::ProcessArgs {
            request: ProcessRequest::CoordsToAudio,
        };
        self.inner.process(args).await.map_err(to_js)?;
        info!("processing image ok");
        Ok(())
    }
}
