use std::cmp::Ordering::Equal;
use std::cell::RefCell;

/// Provides ability to chain different lerp segments together to form
/// something like "lerp curve" (very simple analog to the Unity's AnimationCurve)
#[derive(Default, Debug, Clone)]
pub struct MultiKeyedLerp {
    segments: Vec<(KeyFrame, KeyFrame)>,
    last_segment: RefCell<Option<(KeyFrame, KeyFrame)>>,
    pub(crate) keyframes: Vec<KeyFrame>
}

impl MultiKeyedLerp {
    /// Constructs new lerp curve using a list of keyframes
    /// 
    /// # Examples
    /// use zara::health;
    /// ```
    /// let o = health::MultiKeyedLerp::new(vec![
    ///     health::KeyFrame::new(0., 0.),
    ///     health::KeyFrame::new(0.5, 7.),
    ///     health::KeyFrame::new(1., 12.3)
    /// ]);
    /// ```
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
            last_segment: RefCell::new(None),
            keyframes
        }
    }

    /// Evaluates curve at a given time. Returns `None` if a given time is not on a
    /// curve time scale.
    /// 
    /// # Parameters
    /// - `time`: time value to evaluate at
    /// 
    /// # Examples
    /// ```
    /// if let Some(value) = curve.evaluate(0.75) {
    ///     // ...
    /// }
    /// ```
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

/// Keyframe of the lerp curve (`MultiKeyedLerp`). It consists of a time marker and a value
/// at this time point
#[derive(Copy, Clone, Default, Debug)]
pub struct KeyFrame {
    /// Time value
    pub time: f32,
    /// Value at this time point
    pub value: f32
}

impl KeyFrame {
    /// Constructs a new keyframe
    /// 
    /// # Parameters
    /// - `time`: time value of the curve keyframe
    /// - `value`: value for the given time
    /// 
    /// # Examples
    /// ```
    /// use zara::health;
    /// 
    /// let o = health::KeyFrame::new(0.5, 7.6);
    /// ```
    pub fn new(time: f32, value: f32) -> Self { KeyFrame { time, value } }
}