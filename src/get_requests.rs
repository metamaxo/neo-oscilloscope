use crate::JsInterface;
use crate::backend::settings::*;
use crate::backend::state;
use crate::to_js;
use serde_wasm_bindgen::to_value;

use log::info;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl JsInterface {
    #[wasm_bindgen]
    pub async fn get_int_amount(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from_f64(
            self.inner.settings(GetIntAmount).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_threshold(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from_f64(
            self.inner.settings(GetThreshold).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_pix_threshold(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetPixThreshold).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_sample_rate(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetSampleRate).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_repeat(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetRepeat).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_playback_rate(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetPlaybackRate).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_dot_mode(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetDotMode).await.map_err(to_js)?,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_scale(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetScale).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_stroke(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetStroke).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_persistence(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetPersistence).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_hue(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetHue).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_image_opacity(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetImageOpacity).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_noise(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetNoise).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_center_x(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetCenterx).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_center_y(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetCentery).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_clip_length(&mut self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(
            self.inner.settings(GetClipLength).await.map_err(to_js)? as f64,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_settings_json(&mut self) -> Result<JsValue, JsValue> {
        let settings = self.inner.settings(GetSettings).await.map_err(to_js)?;
        Ok(to_value(&settings).unwrap())
    }

    #[wasm_bindgen]
    pub async fn get_black_coords(&mut self) -> Result<JsValue, JsValue> {
        let points = self.inner.state(state::GetBlackCoords).await;
        match points {
            Ok(Some(points)) => {
                let flat: Vec<f32> = points.iter().flat_map(|(x, y)| vec![*x, *y]).collect();

                Ok(js_sys::Float32Array::from(flat.as_slice()).into())
            }
            Ok(None) => Err(to_js(anyhow::anyhow!("Black coords data is missing"))),
            Err(e) => Err(to_js(e)),
        }
    }

    #[wasm_bindgen]
    pub async fn get_audio(&mut self) -> Result<js_sys::Uint8Array, JsValue> {
        info!("fetching audio");
        let audio_data: Result<Option<Arc<Vec<u8>>>, anyhow::Error> =
            self.inner.state(state::GetAudio).await;

        match audio_data {
            Ok(Some(audio)) => Ok(js_sys::Uint8Array::from(audio.as_slice())),
            Ok(None) => Err(to_js(anyhow::anyhow!("Audio data is missing"))),
            Err(e) => Err(to_js(e)),
        }
    }
}
