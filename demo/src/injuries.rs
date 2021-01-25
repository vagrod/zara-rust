use zara::health::{StageLevel};
use zara::health::injury::{StageBuilder};

pub struct Cut;
zara::injury!(Cut, "Cut", None,
    vec![
        StageBuilder::start()
            .build_for(StageLevel::InitialStage)
                .self_heal(20)
                .drains()
                    .stamina(0.2)
                    .blood_level(0.08)
                .deadly()
                    .with_chance_of_death(0)
                .will_reach_target_in(0.3)
                .will_end()
            .build(),

        StageBuilder::start()
            .build_for(StageLevel::Progressing)
                .self_heal(20)
                .no_drains()
                .no_death_probability()
                .will_reach_target_in(0.1)
                .will_end()
            .build()
    ]
);