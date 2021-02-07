Full-featured [Zara Survival Engine](https://github.com/vagrod/zara) rewritten from scratch in [Rust](https://www.rust-lang.org).

The code is stable.

![Zara Rust Demo](http://imw.su/zara_rust_008.png)

## Description
Zara will be useful for you if you want your game to have weather-aware health control with ton of intertwined parameters, sleeping, fatigue, diseases, injuries (cuts, fractures), food, water, inventory with crafting, clothes with different water/cold resistance levels and more.

## Features
- Health engine with support for diseases, injuries (cuts, fractures, etc.), their treatment (with pills, injections and/or appliances)
- Ability to affect vitals based on any imaginable condition (weather, health, clothes, inventory,...)
- Inventory with crafting (any number of items in a crafting recipe)
- Support for weather (temperature, wind speed, rain intensity) and player status (running, walking, swimming and so on)
- Water, food, pills, injections
- Clothes with different water- and cold-resistances; body appliances (like bandages)
- Warmth and wetness levels built-in
- Sleep mechanics; fatigue mechanics
- Dozen of vital parameters like heart rate, blood pressure, oxygen, food, water levels and more
- Game events support
- Every complex entity can be constructed using simple fluent interface
- Medical agents, side effects, inventory monitors (to control spoiling for example), disease monitors
- Number of built-in side effects like running effects, underwater effects and such

Saving and restoring engine state is supported: everything except inventory. Inventory is very custom to every use case, and you must handle it in the way your project structure/logic demands.

## What's in progress
- Wiki; preparing for a release

The demo is using `crossterm`.
