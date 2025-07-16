use crate::JsInterface;
use crate::backend::settings::*;
use crate::backend::state;
use crate::image_to_coords::method::Method;
use crate::to_js;
use crate::utils;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use serde_wasm_bindgen::from_value;

#[wasm_bindgen]
impl JsInterface {
    #[wasm_bindgen]
    pub async fn set_int_amount(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: usize =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid usize: {}", e)))?;
        self.inner.settings(SetIntAmount(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_threshold(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: u8 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid u8: {}", e)))?;
        self.inner.settings(SetThreshold(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_edge_threshold(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: u8 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid u8: {}", e)))?;
        self.inner
            .settings(SetEdgeThreshold(val))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_pix_threshold(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: u32 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid u32: {}", e)))?;
        self.inner
            .settings(SetPixThreshold(val))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_sample_rate(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: u32 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid u32: {}", e)))?;
        self.inner.settings(SetSampleRate(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_spread_type(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: u32 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid u32: {}", e)))?;
        self.inner.settings(SetSpreadType(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_repeat(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: u32 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid u32: {}", e)))?;
        self.inner.settings(SetRepeat(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_starting_point(&mut self, x: JsValue, y: JsValue) -> Result<(), JsValue> {
        let x_val: f64 =
            from_value(x).map_err(|e| JsValue::from_str(&format!("Invalid f64 for x: {}", e)))?;
        let y_val: f64 =
            from_value(y).map_err(|e| JsValue::from_str(&format!("Invalid f64 for y: {}", e)))?;
        self.inner
            .settings(SetStartingPoint((x_val, y_val)))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_playback_rate(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f32 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid f32: {}", e)))?;
        self.inner
            .settings(SetPlaybackRate(val))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_method(&mut self, value: &str) -> Result<(), JsValue> {
        let method = Method::try_from(value).map_err(|_| JsValue::from_str("Unknown method"))?;
        self.inner.settings(SetMethod(method)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_edge_detection(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner
            .settings(SetEdgeDetection(value))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_flatten(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.settings(SetFlatten(value)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_horizontal(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner
            .settings(SetHorizontal(value))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_double_trace(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner
            .settings(SetDoubleTrace(value))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_scramble(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.settings(SetScramble(value)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_dot_mode(&mut self, value: bool) -> Result<(), JsValue> {
        self.inner.settings(SetDotMode(value)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_scale(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid scale: {}", e)))?;
        self.inner.settings(SetScale(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_stroke(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid stroke: {}", e)))?;
        self.inner.settings(SetStroke(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_persistence(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid persistence: {}", e)))?;
        self.inner
            .settings(SetPersistence(val))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_hue(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid hue: {}", e)))?;
        self.inner.settings(SetHue(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_image_opacity(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid image_opacity: {}", e)))?;
        self.inner
            .settings(SetImageOpacity(val))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_noise(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 =
            from_value(value).map_err(|e| JsValue::from_str(&format!("Invalid noise: {}", e)))?;
        self.inner.settings(SetNoise(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_center_x(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid center_x: {}", e)))?;
        self.inner.settings(SetCenterX(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_center_y(&mut self, value: JsValue) -> Result<(), JsValue> {
        let val: f64 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid center_y: {}", e)))?;
        self.inner.settings(SetCenterY(val)).await.map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_clip_length(&mut self, value: JsValue) -> Result<(), JsValue> {
        let clip_length: f64 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid clip length: {}", e)))?;
        self.inner
            .settings(SetClipLength(clip_length))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_scan_type(&mut self, value: JsValue) -> Result<(), JsValue> {
        let scan_type: u32 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid clip length: {}", e)))?;
        self.inner
            .settings(SetScanType(scan_type))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_scan_line_type(&mut self, value: JsValue) -> Result<(), JsValue> {
        let line_type: u32 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid clip length: {}", e)))?;
        self.inner
            .settings(SetScanlineType(line_type))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_step_amount(&mut self, value: JsValue) -> Result<(), JsValue> {
        let clip_length: u32 = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid clip length: {}", e)))?;
        self.inner
            .settings(SetSnakeStepAmount(clip_length))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn set_directions(&mut self, value: JsValue) -> Result<(), JsValue> {
        let directions: Vec<u32> = from_value(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid directions: {}", e)))?;
        self.inner
            .settings(SetDirections(Some(directions)))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn reset_directions(&mut self) -> Result<(), JsValue> {
        self.inner
            .settings(SetDirections(None))
            .await
            .map_err(to_js);
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn audio_to_backend(&mut self, audio_data: Vec<u8>) -> Result<(), JsValue> {
        self.inner
            .state(state::SetAudio(Arc::new(audio_data)))
            .await
            .map_err(to_js)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn image_to_backend(&mut self, image_data: &[u8]) -> Result<(), JsValue> {
        let image = image::load_from_memory(image_data).map_err(to_js)?;
        let gray_image = image.to_luma8();
        let (result, size) = utils::convert_to_canvas_size(&gray_image);
        self.inner
            .state(state::SetImage(Arc::new(result)))
            .await
            .map_err(to_js)?;
        self.inner.settings(SetSize(size)).await.map_err(to_js)?;
        Ok(())
    }
}
