This is my attempt to rewrite [Zara](https://github.com/vagrod/zara) in [Rust](https://www.rust-lang.org).

This code is **WIP** and changes all the time.

## What's done at the moment
- Game time

- Inventory:
  + Adding/Removing items
  + Inventory weight support
  + Describing and registering crafting combinations (via fluent interface)
  + Getting recipes available for a given set of items
  + Consumables: water and food, consuming items
  + Macros to write less code describing items (one-liners)
  + Appliances as inventory item type
  
- Health:
  + Sleeping
  + "Side effects" monitors as traits
  + Vitals (body temperature, heart rate, blood pressure, stamina, fatigue). 
  + Most common "side effects" implemented like "fluctuating vitals", running (stamina and fatigue drain), fatgue based on sleepng time and sleepng duration, food drain, water drain
  + Describng diseases via fluent interface
  + Diseases as traits (fully implemented). 
  + Disease montors as traits
  + Inverting ("curing") a disease, "invertng back"
  + Disease treatment (with consumables and appliances)
  + Spawning/removing a disease
  + Injuries (w/fluent)
  + Injury treatment with appliances (like injections/bandages/splints/etc)
  + Spawning/removing an injury
  + Inverting ("curing") an injury, "invertng back"
  + Body parts for injuries and treatment with appliances
 
- Player status
  + Walking/running/swimming/underwater states that can be used by "side effect monitors" and "disease monitors" to spawn a disease, affect vitals and other parameters

- Weather status
  + Rain intensity, temperature, wind speed that can be used by "side effect monitors" and "disease monitors" to spawn a disease, affect vitals and other parameters

- Game events system

## What's in progress
- Medical agents
- Clothes and body appliances
- Oxygen level (and optional "side effect" to control it)
- Warmth and wetness levels (and optional "side effects" to control them)
- More death events (vitals death, suffocation, starvation, thirst, etc)
- Checking crafting recipe for resources availability
- Actual getting new item from a crafting combination and spending resources on it
- Trait to get and restore state of every engine node
- Optional easing variants for lerping diseases/injuries/vitals

The demo is built with `termion` which seems like does not support Windows. Maybe later I'll try find another solution for the demo "UI".

![Zara Rust Demo](http://imw.su/zara_rust_001.png)
