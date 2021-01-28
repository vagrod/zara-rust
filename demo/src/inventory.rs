use zara::utils::{GameTimeC};
use std::cell::Cell;

pub struct Knife { pub count: Cell<usize> }
zara::inv_item!(Knife, "Knife", 432.);

pub struct Rope { pub count: Cell<usize> }
zara::inv_item!(Rope, "Rope", 328.);

pub struct Meat { pub count: Cell<usize> }
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

pub struct MRE { pub count: Cell<usize> }
pub struct MREConsumableOption;
zara::inv_item_cons!(MRE, "MRE", 269., Some(&MREConsumableOption));
zara::inv_food!(
    MREConsumableOption,
    /* water gain, 0..100% */ 0.,
    /* food gain, 0..100% */ 18.,
    /* spoil option */ None
);

pub struct AspirinPills { pub count: Cell<usize> }
pub struct AspirinPillsConsumableOption;
zara::inv_item_cons!(AspirinPills, "Aspirin Pills", 27., Some(&AspirinPillsConsumableOption));
zara::inv_food!(
    AspirinPillsConsumableOption,
    /* water gain, 0..100% */ 0.,
    /* food gain, 0..100% */ 0.,
    /* spoil option */ None
);

pub struct Pants { pub count: Cell<usize> }
pub struct PantsClothes;
zara::inv_item_clothes!(Pants, "Pants", 1622., Some(&PantsClothes));
zara::inv_clothes!(
    PantsClothes,
    /* cold resistance, 0..100% */ 5.,
    /* water resistance, 0..100% */ 14.
);

pub struct Jacket { pub count: Cell<usize> }
pub struct JacketClothes;
zara::inv_item_clothes!(Jacket, "Jacket", 1874., Some(&JacketClothes));
zara::inv_clothes!(
    JacketClothes,
    /* cold resistance, 0..100% */ 9.,
    /* water resistance, 0..100% */ 38.
);