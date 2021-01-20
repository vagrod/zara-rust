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
                    .will_reach_target_in(0.7)
                    .will_end()
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Progressing)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(37.9)
                    .with_target_heart_rate(89.)
                    .with_target_blood_pressure(126., 84.)
                    .will_reach_target_in(1.2)
                    .will_last_forever()
            .build()
    ]
);

pub struct Angina;
zara::disease!(Angina, "Angina",
    vec![
        StageBuilder::start()
            .build_for(StageLevel::InitialStage)
                .self_heal(0.5)
                .vitals()
                    .with_target_body_temp(37.5)
                    .with_target_heart_rate(85.)
                    .with_target_blood_pressure(130., 90.)
                    .will_reach_target_in(0.7)
                    .will_end()
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Progressing)
                .no_self_heal()
                .vitals()
                    .with_target_body_temp(38.2)
                    .with_target_heart_rate(89.)
                    .with_target_blood_pressure(126., 84.)
                    .will_reach_target_in(1.2)
                    .will_end()
            .build()
    ]
);