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

        {
            const PERIOD: f64 = 1.0 / 45.45;
            const SPACE: f32 = 1800.0;
            const MARK: f32 = 1400.0;

            let frequency = oscillator.frequency();
            for (i, val) in [SPACE, MARK, MARK, SPACE, SPACE, SPACE, MARK].into_iter().enumerate() {
                frequency.set_value_at_time(val, i as f64 * PERIOD)?;
            }
        }

        oscillator
            .connect_with_audio_node(&ctx.destination())
            .unwrap();

        Ok(Self { ctx, oscillator })
    }

    #[wasm_bindgen]
    pub async fn play(&self) {
        self.oscillator.start().unwrap();
        JsFuture::from(self.ctx.resume().unwrap()).await.unwrap();
    }
}

#[wasm_bindgen(start)]
pub fn lol() {}
