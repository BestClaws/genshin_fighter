use bevy::prelude::*;
use std::time::Duration;

#[derive(Debug)]
struct Animation {
    current_frame: usize,
    /// frame delays in (ms)
    frame_delays: Vec<i32>,
    elapsed: u32,
    total_duration: u32,
}

impl Animation {
    fn from(frame_delays_ms: Vec<i32>) -> Self {
        let current_frame = 0;
        let elapsed = 0;

        let total_duration = frame_delays_ms
            .iter()
            .map(|e| e.abs())
            .sum::<i32>() as u32;

        Self {
            current_frame,
            frame_delays: frame_delays_ms,
            elapsed,
            total_duration,
        }
    }

    fn advance(&mut self, adv_ms: u32) {

        let abs_advance = self.elapsed + adv_ms;
        let mut offset_adv = abs_advance % self.total_duration;

        self.elapsed = offset_adv;
        self.current_frame = 0;

        let mut frame_delays_itr = self.frame_delays.iter();

        while offset_adv > 0 {
            let next = *frame_delays_itr.next().unwrap() as u32;
            if offset_adv > next {
                offset_adv -= next;
                self.current_frame+=1;
            } else {
                break;
            }
        }
    }
}


#[derive(Bundle)]
pub struct AnimatedSpriteSheetBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

fn anim(anim: Animation) {}
