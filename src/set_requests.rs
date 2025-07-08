use crate::JsInterface;
use crate::backend::settings::*;
use crate::backend::state;
use crate::image_to_coords::mode::Mode;
use crate::to_js;
use crate::utils;
use image::GrayImage;
use log::info;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl JsInterface {
    #[wasm_bindgen]
    pub async fn set_int_amount(&mut self, value: JsValue) -> Result<(), JsValue> {
        info!("setting int amount to: {:?}", value);
        self.inner
            .settings(SetIntAmount(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))?
                    as usize,
            ))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_threshold(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetThreshold(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))? as u8,
            ))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_pix_threshold(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetPixThreshold(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number, got: {value}"))?
                    as u32,
            ))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_sample_rate(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetSampleRate(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))? as u32,
            ))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_duration_secs(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetDurationSecs(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))? as u32,
            ))
            .await
            .map_err(to_js)?;
        Ok(())
    }
    #[wasm_bindgen]
    pub async fn set_repeat(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetRepeat(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))? as u32,
            ))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_playback_rate(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetPlaybackRate(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))? as f32,
            ))
            .await
            .map_err(to_js)?;
        info!("playback rate set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_mode(&mut self, value: &str) -> Result<(), JsValue> {
        let mode = Mode::try_from(value).expect("unknow mode");
        self.inner.settings(SetMode(mode)).await.map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_edge_detection(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner
            .settings(SetEdgeDetection(value))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn audio_to_backend(&mut self, audio_data: Vec<u8>) -> Result<(), JsValue> {
        info!("sending audio to backend");
        self.inner
            .state(state::SetAudio(Arc::new(audio_data)))
            .await
            .map_err(to_js)
            .ok();
        info!("audio to backend ok");
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn image_to_backend(&mut self, image_data: &[u8]) -> Result<(), JsValue> {
        let image = image::load_from_memory(image_data).map_err(to_js)?;
        let gray_image: GrayImage = image.to_luma8();
        let (result, size) = utils::convert_to_canvas_size(&gray_image);
        info!("sending image to backend");
        self.inner
            .state(state::SetImage(Arc::new(result)))
            .await
            .map_err(to_js)
            .ok();
        info!("image to backend ok");
        self.inner.settings(SetSize(size)).await.map_err(to_js);
        Ok(())
    }
}
