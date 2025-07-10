use crate::JsInterface;
use crate::backend::settings::*;
use crate::backend::state;
use crate::image_to_coords::method::Method;
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
    pub async fn set_spread_type(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetSpreadType(
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
    pub async fn set_starting_point(&mut self, x: JsValue, y: JsValue) -> Result<(), JsValue> {
        info!("setting starting point: {:?}, {:?}", x, y);
        self.inner
            .settings(SetStartingPoint((
                x.as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))?,
                y.as_f64()
                    .ok_or_else(|| JsValue::from_str("Expected a number"))?,
            )))
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
    pub async fn set_method(&mut self, value: &str) -> Result<(), JsValue> {
        let method = Method::try_from(value).expect("unknow method");
        self.inner
            .settings(SetMethod(method))
            .await
            .map_err(to_js)?;
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
    pub async fn set_dot_mode(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner
            .settings(SetDotMode(value))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_scale(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetScale(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("scale set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_stroke(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetStroke(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("stroke set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_persistence(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetPersistence(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("persistence set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_hue(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetHue(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("hue set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_image_opacity(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetImageOpacity(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("image_opacity set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_noise(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetNoise(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("noise set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_center_x(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetCenterx(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("centerx set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_center_y(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetCentery(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("centery set to: {}", value.as_f64().unwrap());
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_clip_length(&mut self, value: JsValue) -> Result<(), JsValue> {
        self.inner
            .settings(SetClipLength(
                value
                    .as_f64()
                    .ok_or_else(|| JsValue::from_str("ecpected a number"))?,
            ))
            .await
            .map_err(to_js)?;
        info!("clip_length set to: {}", value.as_f64().unwrap());
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
        info!("size is: {}", size);
        let _ = self.inner.settings(SetSize(size)).await.map_err(to_js);
        Ok(())
    }
}
