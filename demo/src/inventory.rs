use zara::utils::{GameTimeC};

use std::cell::Cell;

pub struct Knife { pub count: Cell<usize> }
pub struct Rope { pub count: Cell<usize> }

zara::inv_item!(Knife, "Knife", 432.);
zara::inv_item!(Rope, "Rope", 328.);

pub struct Meat { pub count: Cell<usize> }
pub struct MeatConsumableOption;
pub struct MeatSpoilOption;
zara::inv_cons_item!(Meat, "Meat", 351., Some(&MeatConsumableOption));
zara::inv_spoil!(MeatSpoilOption, 2, 15, GameTimeC::new(0,4,30,0.));
zara::inv_food!(MeatConsumableOption, 10., 30., Some(&MeatSpoilOption));
