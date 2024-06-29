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

        #[derive(Clone, Copy, PartialEq)]
        enum Case {
            Letters,
            Figures,
        }
        use Case::*;

        let table: HashMap<char, (Case, &[u8; 5])> = [
            // Letters
            ('A', (Letters, b"11000")),
            ('B', (Letters, b"10011")),
            ('C', (Letters, b"01110")),
            ('D', (Letters, b"10010")),
            ('E', (Letters, b"10000")),
            ('F', (Letters, b"10110")),
            ('G', (Letters, b"01011")),
            ('H', (Letters, b"00101")),
            ('I', (Letters, b"01100")),
            ('J', (Letters, b"11010")),
            ('K', (Letters, b"11110")),
            ('L', (Letters, b"01001")),
            ('M', (Letters, b"00111")),
            ('N', (Letters, b"00110")),
            ('O', (Letters, b"00011")),
            ('P', (Letters, b"01101")),
            ('Q', (Letters, b"11101")),
            ('R', (Letters, b"01010")),
            ('S', (Letters, b"10100")),
            ('T', (Letters, b"00001")),
            ('U', (Letters, b"11100")),
            ('V', (Letters, b"01111")),
            ('W', (Letters, b"11001")),
            ('X', (Letters, b"10111")),
            ('Y', (Letters, b"10101")),
            ('Z', (Letters, b"10001")),
            (' ', (Letters, b"00100")),
            // Figures
            ('0', (Figures, b"01101")),
            ('1', (Figures, b"10111")),
            ('2', (Figures, b"10011")),
            ('3', (Figures, b"10000")),
            ('4', (Figures, b"01010")),
            ('5', (Figures, b"00001")),
            ('6', (Figures, b"10101")),
            ('7', (Figures, b"11100")),
            ('8', (Figures, b"01100")),
            ('9', (Figures, b"00011")),
            ('.', (Figures, b"00111")),
        ]
        .into_iter()
        .collect();

        let mut case = None;

        {
            const PERIOD: f64 = 1.0 / 45.45;
            const SPACE: f32 = 1800.0;
            const MARK: f32 = 1400.0;

            let frequency = oscillator.frequency();

            for (i, &bits) in text
                .chars()
                .flat_map(|c| match table.get(&c) {
                    None => vec![],
                    Some(&(next_case, bits)) => {
                        if case != Some(next_case) {
                            let shift = match next_case {
                                Letters => b"11111",
                                Figures => b"11011",
                            };
                            case = Some(next_case);
                            vec![shift, bits]
                        } else {
                            vec![bits]
                        }
                    }
                })
                .enumerate()
            {
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
