use std::collections::HashMap;

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
    pub async fn new(text: String) -> Result<Player, JsValue> {
        let ctx = AudioContext::new()?;
        JsFuture::from(ctx.suspend()?).await?;
        let oscillator = OscillatorNode::new(&ctx)?;
        oscillator.set_type(Sine);

        let table: HashMap<char, &[u8; 5]> = [
            ('A', b"11000"),
            ('B', b"10011"),
            ('C', b"01110"),
            ('D', b"10010"),
            ('E', b"10000"),
            ('F', b"10110"),
            ('G', b"01011"),
            ('H', b"00101"),
            ('I', b"01100"),
            ('J', b"11010"),
            ('K', b"11110"),
            ('L', b"01001"),
            ('M', b"00111"),
            ('N', b"00110"),
            ('O', b"00011"),
            ('P', b"01101"),
            ('Q', b"11101"),
            ('R', b"01010"),
            ('S', b"10100"),
            ('T', b"00001"),
            ('U', b"11100"),
            ('V', b"01111"),
            ('W', b"11001"),
            ('X', b"10111"),
            ('Y', b"10101"),
            ('Z', b"10001"),
            (' ', b"00100"),
        ]
        .into_iter()
        .collect();

        {
            const PERIOD: f64 = 1.0 / 45.45;
            const SPACE: f32 = 1800.0;
            const MARK: f32 = 1400.0;

            let frequency = oscillator.frequency();

            for (i, &bits) in text.chars().flat_map(|c| table.get(&c)).enumerate() {
                let start = i as f64 * 8.0 * PERIOD;
                frequency.set_value_at_time(SPACE, start)?;

                for (j, bit) in bits.into_iter().enumerate() {
                    let target = match bit {
                        b'0' => SPACE,
                        b'1' => MARK,
                        _ => panic!(),
                    };
                    frequency.set_value_at_time(target, start + (j + 1) as f64 * PERIOD)?;
                }

                frequency.set_value_at_time(MARK, start + 6.0 * PERIOD)?;
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
