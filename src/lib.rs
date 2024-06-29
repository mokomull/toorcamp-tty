use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, OscillatorNode, OscillatorType::Sine};

#[wasm_bindgen]
pub struct Player {
    ctx: AudioContext,
    oscillator: OscillatorNode,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Player, JsValue> {
        let ctx = AudioContext::new()?;
        JsFuture::from(ctx.suspend()?).await?;
        let oscillator = OscillatorNode::new(&ctx)?;
        oscillator.set_type(Sine);
        oscillator.frequency().set_value(1004.0);

        oscillator
            .connect_with_audio_node(&ctx.destination())
            .unwrap();

        Ok(Self { ctx, oscillator })
    }

    #[wasm_bindgen]
    pub async fn play(&self) {
        JsFuture::from(self.ctx.resume().unwrap()).await.unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn lol() {}