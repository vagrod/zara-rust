use zara::utils::{GameTimeC};

pub struct Knife { pub count: usize }
zara::inv_item!(Knife, "Knife", 432.);

pub struct Rope { pub count: usize }
zara::inv_item!(Rope, "Rope", 328.);

pub struct SharpStone { pub count: usize }
zara::inv_item!(SharpStone, "SharpStone", 318.);

pub struct Stick { pub count: usize }
zara::inv_item!(Stick, "Stick", 159.);

pub struct Meat { pub count: usize }
pub struct MeatConsumableOption;
pub struct MeatSpoilOption;
zara::inv_item_cons!(Meat, "Meat", 351., Some(&MeatConsumableOption));
zara::inv_spoil!(
    MeatSpoilOption,
    /* fresh poisoning chance, 0..100% probability */ 2,
    /* spoiled poisoning chance, 0..100% probability */ 15,
    /* spoil time */ GameTimeC::new(0,4,30,0.)
);
zara::inv_food!(
    MeatConsumableOption,
    /* water gain, 0..100% */ 10.,
    /* food gain, 0..100% */ 30.,
    /* spoil option */ Some(&MeatSpoilOption)
);

pub struct MRE { pub count: usize }
pub struct MREConsumableOption;
zara::inv_item_cons!(MRE, "MRE", 269., Some(&MREConsumableOption));
zara::inv_food!(
    MREConsumableOption,
    /* water gain, 0..100% */ 0.,
    /* food gain, 0..100% */ 18.,
    /* spoil option */ None
);

pub struct AspirinPills { pub count: usize }
pub struct AspirinPillsConsumableOption;
zara::inv_item_cons!(AspirinPills, "Aspirin Pills", 27., Some(&AspirinPillsConsumableOption));
zara::inv_food!(
    AspirinPillsConsumableOption,
    /* water gain, 0..100% */ 0.,
    /* food gain, 0..100% */ 0.,
    /* spoil option */ None
);

pub struct Pants { pub count: usize }
pub struct PantsClothes;
zara::inv_item_clothes!(Pants, "Pants", 1622., Some(&PantsClothes));
zara::inv_clothes!(
    PantsClothes,
    /* cold resistance, 0..100% */ 1.,
    /* water resistance, 0..100% */ 14.
);

pub struct Jacket { pub count: usize }
pub struct JacketClothes;
zara::inv_item_clothes!(Jacket, "Jacket", 1874., Some(&JacketClothes));
zara::inv_clothes!(
    JacketClothes,
    /* cold resistance, 0..100% */ 2.,
    /* water resistance, 0..100% */ 38.
);

pub struct Bandage {pub count: usize }
pub struct BandageAppliance;
zara::inv_item_appl!(
    Bandage,
    "Bandage",
    /* weight per unit */ 59.,
    Some(&BandageAppliance)
);
zara::inv_body_appliance!(BandageAppliance);