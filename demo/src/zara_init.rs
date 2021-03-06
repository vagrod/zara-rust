use super::events::ZaraEventsListener;
use super::inventory;
use crate::inventory::Knife;
use super::inventory::{PantsClothes, JacketClothes};

use zara::health::medagent::{CurveType};
use zara::health::{MedicalAgentBuilder};
use zara::body::{ClothesGroupBuilder};
use zara::inventory::crafting;

pub fn init_zara_instance() -> zara::ZaraController<ZaraEventsListener>{
    // Instantiate our events listener
    let events_listener = ZaraEventsListener;

    // Describe environment conditions
    let environment = zara::utils::EnvironmentC::new(24., 2., 0.12);

    // Initialize Zara instance
    let person =
        zara::ZaraController::with_environment (
            events_listener, environment
        );

    person.health.register_medical_agents (
        vec![
            MedicalAgentBuilder::start()
                .for_agent("Aspirin")
                    .activates(CurveType::Immediately)
                    .and_lasts_for_minutes(23.)
                    .includes(
                        vec![
                            "Aspirin Pills",
                            "Big Green Leaves",
                            "Syringe With Aspirin",
                            "This Strange Glowy Pink Goop That I Found In Thaaaaat Very Cave Yesterday When I Was Wandering Here At Night And..."
                        ]
                    )
                .build()
        ]
    );

    person.body.register_clothes_groups(
        vec![
            ClothesGroupBuilder::start()
                .with_name("Water Resistant Suit")
                    .bonus_cold_resistance(2)
                    .bonus_water_resistance(7)
                    .includes(
                        vec![
                            ("Pants", Box::new(PantsClothes)),
                            ("Jacket", Box::new(JacketClothes)),
                        ]
                    )
                .build()
        ]
    );

    populate_inventory(&person);
    dress(&person);

    person
}

fn dress(person: &zara::ZaraController<ZaraEventsListener>) {
    person.put_on_clothes(&format!("Pants")).ok();
    person.put_on_clothes(&format!("Jacket")).ok();
}

fn populate_inventory(person: &zara::ZaraController<ZaraEventsListener>) {
    let meat = inventory::Meat{ count: 2 };
    //let knife = inventory::Knife{ count: 1 };
    let rope = inventory::Rope{ count: 2 };
    let stone = inventory::SharpStone{ count: 2 };
    let stick = inventory::Stick{ count: 4 };
    let mre = inventory::MRE{ count: 2 };
    let aspirin = inventory::AspirinPills{ count: 2 };
    let pants = inventory::Pants{ count: 1 };
    let jacket = inventory::Jacket{ count: 1 };
    let bandage = inventory::Bandage{ count: 5 };

    person.inventory.add_item(Box::new(aspirin));
    person.inventory.add_item(Box::new(meat));
   // person.inventory.add_item(Box::new(knife));
    person.inventory.add_item(Box::new(rope));
    person.inventory.add_item(Box::new(stone));
    person.inventory.add_item(Box::new(stick));
    person.inventory.add_item(Box::new(mre));
    person.inventory.add_item(Box::new(pants));
    person.inventory.add_item(Box::new(jacket));
    person.inventory.add_item(Box::new(bandage));

    person.inventory.register_crafting_combinations(
        vec![
            crafting::Builder::start()
                .build_for("Knife")
                    .is("SharpStone", 1)
                    .plus("Stick", 1)
                    .and("Rope", 2)
                .build(zara::inv_result!(Knife { count: 1 }))
        ]
    );

    // Crafting test
    let ids = person.inventory.get_suitable_combinations_for(
        vec![
            &format!("Stick"),
            &format!("Rope"),
            &format!("SharpStone")
        ]);

    person.inventory.execute_combination(&ids[0]).ok();
}