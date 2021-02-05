use std::cmp::Ordering::Equal;
use std::cell::RefCell;

pub struct MultiKeyedLerp {
    segments: Vec<(KeyFrame, KeyFrame)>,
    last_segment: RefCell<Option<(KeyFrame, KeyFrame)>>
}

impl MultiKeyedLerp {
    pub fn new(mut keyframes: Vec<KeyFrame>) -> Self {
        let mut segments = Vec::new();

        keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Equal));

        if keyframes.len() >= 2 {
            for i in 1..keyframes.len() {
                segments.push((keyframes[i-1], keyframes[i]));
            }
        }

        MultiKeyedLerp {
            segments,
            last_segment: RefCell::new(None)
        }
    }

    pub fn evaluate(&self, time: f32) -> Option<f32> {
        let rescan_segments = || {
            match self.find_segment(time) {
                Some(seg) => {
                    self.last_segment.replace(Some(seg))
                },
                _ => self.last_segment.replace(None)
            }
        };

        let mut need_rescan = false;
        match self.last_segment.borrow().as_ref() {
            Some(seg) => {
                if !(time >= seg.0.time && time <= seg.1.time) {
                    need_rescan = true;
                }
            },
            None => {
                need_rescan = true;
            }
        }

        if need_rescan { rescan_segments(); }

        match self.last_segment.borrow().as_ref() {
            Some(seg) => {
                // sanity check
                if !(seg.0.time..=seg.1.time).contains(&time) { return None; }

                let p = crate::utils::clamp_01((time - seg.0.time) / (seg.1.time - seg.0.time));

                Some(crate::utils::lerp(seg.0.value, seg.1.value, p))
            },
            _ => None
        }
    }

    fn find_segment(&self, time: f32) -> Option<(KeyFrame, KeyFrame)> {
        for seg in &self.segments {
            if time >= seg.0.time && time <= seg.1.time {
                return Some(seg.clone());
            }
        }

        None
    }
}

#[derive(Copy, Clone)]
pub struct KeyFrame {
    pub time: f32,
    pub value: f32
}

impl KeyFrame {
    pub fn new(time: f32, value: f32) -> Self { KeyFrame { time, value } }
}