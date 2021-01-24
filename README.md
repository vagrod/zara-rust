This is my attempt to rewrite [Zara](https://github.com/vagrod/zara) in [Rust](https://www.rust-lang.org).

This code is **WIP** and changes all the time.

## What's done at the moment
- Game time

- Inventory:
  + Adding/Removing items
  + Inventory weight support
  + Describing and registering crafting combinations (via fluent interface)
  + Checking if recipe exist for a given set of items
  + Consumables: water and food, consuming items
  + Macros to write less code describing items (one-liners)
  
- Health:
  + Sleeping
  + "Side effects" monitors as traits
  + Vitals (body temperature, heart rate, blood pressure, stamina, fatigue). Most common "side effects" implemented like "fluctuating vitals", running (stamina and fatigue drain), fatgue based on sleepng time and sleepng duration, food drain, water drain
  + Diseases as traits (fully implemented, except appliance treatment [like injections]): disease montors as traits, inverting ("curing") a disease, "invertng back"
  + Disease treatment (by consumable)
  + Describng diseases via fluent interface
  + Spawning/removing a disease
 
- Player status
  + Walking/running/swimming/underwater states that can be used by "side effect monitors" and "disease monitors" to spawn a disease, affect vitals and other parameters

- Weather status
  + Rain intensty, temperature, wind speed that can be used by "side effect monitors" and "disease monitors" to spawn a disease, affect vitals and other parameters

- Game events system

## What's in progress
- Appliances as inventory item type
- Disease treatment with appliances
- Medical agents
- Clothes and body appliances
- Oxygen level (and optional "side effect" to control it)
- Warmth and wetness levels (and optional "side effects" to control them)
- Injuries (w/fluent), their treatment with appliances (like injections/bandages/splints/etc)
- More death events (vitals death, suffocation, starvation, thirst, etc)
- Checking crafting recipe for resources availability
- Actual getting new item from a crafting combination and spending resources on it
- Optional easing variants for lerping diseases/injuries/vitals

The demo is built with `termion` which seems like does not support Windows. Maybe later I'll try find another solution for the demo "UI".

![Zara Rust Demo](http://imw.su/zara_rust_001.png)
