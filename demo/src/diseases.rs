use zara::health::disease::{Disease, StageLevel, StageBuilder};

pub struct Flu;
zara::disease!(Flu, "Flu",
    vec![
        StageBuilder::start()
            .build_for(StageLevel::InitialStage)
                .self_heal(0.5)
                .vitals()
                    .with_target_body_temp(37.3)
                    .with_target_heart_rate(85.)
                    .with_target_blood_pressure(130., 90.)
                    .will_reach_target_in(0.1)
                    .will_end()
            .build()
    ]
);