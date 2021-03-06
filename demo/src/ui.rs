use crate::events::ZaraEventsListener;

use zara::utils::GameTimeC;

use std::io::{Write};

pub use crossterm::{
    cursor,
    event::{self, Event as TermEvent, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

pub fn ui_frame<W>(w: &mut W, person: &zara::ZaraController<ZaraEventsListener>)
    where W: Write,
{
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    ).ok();

    let gt = person.environment.game_time.to_contract();

    // Game time
    execute!(w,
        style::SetForegroundColor(style::Color::DarkCyan),
        style::Print("Game Time: "),
        style::SetForegroundColor(style::Color::Cyan),
        style::Print(format_gt(&gt)),
    ).ok();

    let vitals_h = 13;
    // Vitals
    execute!(w,
        cursor::MoveTo(0, 2),
        style::SetForegroundColor(style::Color::DarkGreen),
        style::Print("Vitals"),
        style::SetForegroundColor(style::Color::Green),
        cursor::MoveToNextLine(1),
        style::Print("  Heart rate: "),
        style::Print(format!("{:.0} bpm", person.health.heart_rate())),
        cursor::MoveToNextLine(1),
        style::Print("  Body temperature: "),
        style::Print(format!("{:.1}°C", person.health.body_temperature())),
        cursor::MoveToNextLine(1),
        style::Print("  Blood pressure: "),
        style::Print(format!("{:.0}/{:.0} mmHg", person.health.top_pressure(), person.health.bottom_pressure())),
        cursor::MoveToNextLine(1),
        style::Print("  Food level: "),
        style::Print(format!("{:.0}%", person.health.food_level())),
        cursor::MoveToNextLine(1),
        style::Print("  Water level: "),
        style::Print(format!("{:.0}%", person.health.water_level())),
        cursor::MoveToNextLine(1),
        style::Print("  Stamina: "),
        style::Print(format!("{:.0}%", person.health.stamina_level())),
        cursor::MoveToNextLine(1),
        style::Print("  Fatigue: "),
        style::Print(format!("{:.1}%", person.health.fatigue_level())),
        cursor::MoveToNextLine(1),
        style::Print("  Blood level: "),
        style::Print(format!("{:.1}%", person.health.blood_level())),
        cursor::MoveToNextLine(1),
        style::Print("  Oxygen level: "),
        style::Print(format!("{:.1}%", person.health.oxygen_level())),
        cursor::MoveToNextLine(1),
    ).ok();

    // Misc stats
    execute!(w,
        cursor::MoveToNextLine(1),
        style::SetForegroundColor(style::Color::DarkYellow),
        cursor::MoveTo(0, vitals_h),
        style::Print("Stats"),
        style::SetForegroundColor(style::Color::Yellow),
        cursor::MoveToNextLine(1)
    ).ok();
    match person.body.last_sleep_time() {
        Some(t) => {
            execute!(w,
                cursor::MoveToColumn(3),
                style::Print(format!("Last time slept: {}", format_gt(&t))),
                cursor::MoveToNextLine(1)
            ).ok();
        },
        _ => {
            execute!(w,
                cursor::MoveToColumn(3),
                style::Print("Not slept yet"),
                cursor::MoveToNextLine(1)
            ).ok();
        }
    }
    execute!(w,
        cursor::MoveToColumn(3),
        style::Print(format!("Is sleeping now? {}", if person.body.is_sleeping() { "yes" } else { "no" })),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Blood loss? {}", if person.health.is_blood_loss() { "yes" } else { "no" })),
        cursor::MoveToNextLine(1)
    ).ok();
    execute!(w,
        cursor::MoveToColumn(3),
        style::Print("Warmth level: "),
        style::Print(format!("{:.1} (-5..+5 is a sweet spot)", person.body.warmth_level())),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print("Wetness level: "),
        style::Print(format!("{:.1}%", person.body.wetness_level())),
        cursor::MoveToNextLine(1)
    ).ok();
    execute!(w,
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Walking?: {}", if person.player_state.is_walking.get() {"yes"} else {"no"})),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Running?: {}", if person.player_state.is_running.get() {"yes"} else {"no"})),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Swimming?: {}", if person.player_state.is_swimming.get() {"yes"} else {"no"})),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Under water?: {}", if person.player_state.is_underwater.get() {"yes"} else {"no"})),
        cursor::MoveToNextLine(1),
    ).ok();


    // Weather
    execute!(w,
        cursor::MoveToNextLine(1),
        style::SetForegroundColor(style::Color::DarkCyan),
        style::Print("Weather"),
        style::SetForegroundColor(style::Color::Cyan),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Temperature: {}°C", person.environment.temperature.get())),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Wind speed: {:.1} m/s", person.environment.wind_speed.get())),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Rain intensity: {:.2} (0..1)", person.environment.rain_intensity.get())),
        cursor::MoveToNextLine(1)
    ).ok();

    // Inventory
    let inv_col_base = 43;
    execute!(w,
        cursor::MoveTo(inv_col_base, 0),
        style::SetForegroundColor(style::Color::DarkBlue),
        style::Print(format!("Inventory (total weight {:.1}g)", person.inventory.get_weight())),
        style::SetForegroundColor(style::Color::Blue),
        cursor::MoveToNextLine(1),
    ).ok();
    for (name, item) in person.inventory.items.borrow().iter() {
        execute!(w,
            cursor::MoveToColumn(inv_col_base+4),
            style::Print(format!("{} ({})", name, item.get_count())),
            cursor::MoveToNextLine(1),
            cursor::MoveToColumn(inv_col_base+6),
            style::Print(format!("Total weight: {}g", item.get_total_weight())),
            cursor::MoveToNextLine(1)
        ).ok();
        match item.consumable() {
            Some(c) => {
                execute!(w,
                    cursor::MoveToColumn(inv_col_base+6),
                    style::Print("Is a consumable")
                ).ok();
                if c.is_food() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(food)")
                    ).ok();
                }
                if c.is_water() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(water)")
                    ).ok();
                }
                execute!(w,
                    cursor::MoveToColumn(inv_col_base),
                    cursor::MoveToNextLine(1),
                    cursor::MoveToColumn(inv_col_base+8),
                    style::Print(format!("+food per dose: {:.1}%", c.food_gain_per_dose())),
                    cursor::MoveToNextLine(1),
                    cursor::MoveToColumn(inv_col_base+8),
                    style::Print(format!("+water per dose: {:.1}%", c.water_gain_per_dose())),
                    cursor::MoveToNextLine(1),
                ).ok();
                match c.spoiling() {
                    Some(sp) => {
                        execute!(w,
                            cursor::MoveToColumn(inv_col_base+8),
                            style::Print(format!("can spoil in {}", format_gt(&sp.spoil_time()))),
                            cursor::MoveToNextLine(1),
                            cursor::MoveToColumn(inv_col_base+10),
                            style::Print(format!("fresh poison chance {:.0}%", sp.fresh_poisoning_chance())),
                            cursor::MoveToNextLine(1),
                            cursor::MoveToColumn(inv_col_base+10),
                            style::Print(format!("spoiled poison chance {:.0}%", sp.spoil_poisoning_chance())),
                            cursor::MoveToNextLine(1),
                        ).ok();
                    },
                    _ => {
                        execute!(w,
                            cursor::MoveToColumn(inv_col_base+8),
                            style::Print("cannot spoil"),
                            cursor::MoveToNextLine(1)
                        ).ok();
                    }
                }
            },
            _ => { }
        }
        match item.appliance() {
            Some(a) => {
                execute!(w,
                    cursor::MoveToColumn(inv_col_base+6),
                    style::Print("Is an appliance")
                ).ok();
                if a.is_body_appliance() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(body appliance)")
                    ).ok();
                }
                if a.is_injection() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(injection)")
                    ).ok();
                }
                execute!(w,
                    cursor::MoveToNextLine(1)
                ).ok();
            },
            _ => { }
        }
    }

    // Diseases
    let dis_col_base = 86;
    execute!(w,
        cursor::MoveTo(dis_col_base, 0),
        style::SetForegroundColor(style::Color::DarkMagenta),
        style::Print("Diseases"),
        style::SetForegroundColor(style::Color::Magenta),
        cursor::MoveToNextLine(1),
    ).ok();
    for (name, item) in person.health.diseases.borrow().iter() {
        let active = item.is_active(&gt);
        let is_healing = item.is_healing();
        let active_string = if is_healing { "active [healing]" } else { "active" };
        let info = format!("{} ({})", name, if active { active_string } else { "scheduled" });
        execute!(w,
            cursor::MoveToColumn(dis_col_base+3),
            style::Print(format!("{}", info)),
            cursor::MoveToNextLine(1),
        ).ok();
        if !active {
            let at = item.activation_time();
            execute!(w,
                cursor::MoveToColumn(dis_col_base+6),
                style::Print(format!("Will activate at {}", format_gt(&at))),
                cursor::MoveToNextLine(1),
            ).ok();
        } else {
            match item.get_active_stage(&gt) {
                Some(st) => {
                    execute!(w,
                        cursor::MoveToColumn(dis_col_base+6),
                        style::Print(format!("Is on level {:?} {}%", st.info.level, st.percent_active(&gt))),
                        cursor::MoveToNextLine(1)
                    ).ok();
                    match item.end_time() {
                        Some(t) => {
                            execute!(w,
                                cursor::MoveToColumn(dis_col_base+6),
                                style::Print(format!("Will end at {}", format_gt(&t))),
                                cursor::MoveToNextLine(1)
                            ).ok();
                        },
                        _ => {
                            execute!(w,
                                cursor::MoveToColumn(dis_col_base+6),
                                style::Print("Will not end"),
                                cursor::MoveToNextLine(1)
                            ).ok();
                        }
                    }
                },
                _ => { }
            }
        }
        execute!(w,
            cursor::MoveToColumn(dis_col_base+6),
            style::Print(format!("Needs treatment? {}", if item.needs_treatment { "yes" } else { "no (will self-heal)" })),
            cursor::MoveToNextLine(1),
        ).ok();
    }

    // Injuries
    let inj_col_base = 86;
    execute!(w,
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(inj_col_base+1),
        style::SetForegroundColor(style::Color::DarkMagenta),
        style::Print("Injuries"),
        style::SetForegroundColor(style::Color::Magenta),
        cursor::MoveToNextLine(1),
    ).ok();
    for (key, item) in person.health.injuries.borrow().iter() {
        let active = item.is_active(&gt);
        let is_healing = item.is_healing();
        let active_string = if is_healing { "active [healing]" } else { "active" };
        let info = format!("{} on {:?} ({})", key.injury, key.body_part, if active { active_string } else { "scheduled" });
        execute!(w,
            cursor::MoveToColumn(inj_col_base+3),
            style::Print(format!("{}", info)),
            cursor::MoveToNextLine(1),
        ).ok();
        if !active {
            let at = item.activation_time();
            execute!(w,
                cursor::MoveToColumn(inj_col_base+6),
                style::Print(format!("Will activate at {}", format_gt(&at))),
                cursor::MoveToNextLine(1),
            ).ok();
        } else {
            match item.get_active_stage(&gt) {
                Some(st) => {
                    execute!(w,
                        cursor::MoveToColumn(inj_col_base+6),
                        style::Print(format!("Is on level {:?} {}%", st.info.level, st.percent_active(&gt))),
                        cursor::MoveToNextLine(1)
                    ).ok();
                    match item.end_time() {
                        Some(t) => {
                            execute!(w,
                                cursor::MoveToColumn(inj_col_base+6),
                                style::Print(format!("Will end at {}", format_gt(&t))),
                                cursor::MoveToNextLine(1)
                            ).ok();
                        },
                        _ => {
                            execute!(w,
                                cursor::MoveToColumn(inj_col_base+6),
                                style::Print("Will not end"),
                                cursor::MoveToNextLine(1)
                            ).ok();
                        }
                    }
                },
                _ => { }
            }
        }
        execute!(w,
            cursor::MoveToColumn(inj_col_base+6),
            style::Print(format!("Fracture? {}", if item.is_fracture { "yes" } else { "no" })),
            cursor::MoveToNextLine(1),
        ).ok();
        execute!(w,
            cursor::MoveToColumn(inj_col_base+6),
            style::Print(format!("Needs treatment? {}", if item.needs_treatment { "yes" } else { "no (will self-heal)" })),
            cursor::MoveToNextLine(1),
        ).ok();
        execute!(w,
            cursor::MoveToColumn(inj_col_base+6),
            style::Print(format!("Blood forcibly stopped? {}", if item.is_blood_stopped() { "yes" } else { "no" })),
            cursor::MoveToNextLine(1),
        ).ok();
    }

    // Medical Agents
    let medagent_col_base = 133;
    execute!(w,
        cursor::MoveTo(medagent_col_base, 0),
        style::SetForegroundColor(style::Color::DarkRed),
        style::Print(format!("Medical Agents ({} active now)", person.health.medical_agents.active_count())),
        style::SetForegroundColor(style::Color::Red),
        cursor::MoveToNextLine(1),
    ).ok();
    for (name, agent) in person.health.medical_agents.agents.borrow().iter() {
        execute!(w,
            cursor::MoveToColumn(medagent_col_base + 3),
            style::Print(format!("{}", name)),
            cursor::MoveToNextLine(1),
        ).ok();
        execute!(w,
            cursor::MoveToColumn(medagent_col_base + 6),
            style::Print(format!("Percent of activity: {:.1}%", agent.percent_of_activity())),
            cursor::MoveToNextLine(1),
            cursor::MoveToColumn(medagent_col_base + 6),
            style::Print(format!("Percent of presence: {:.1}%", agent.percent_of_presence())),
            cursor::MoveToNextLine(1),
        ).ok();
        match agent.last_dose_end_time() {
            Some(t) => {
                execute!(w,
                cursor::MoveToColumn(medagent_col_base + 6),
                style::Print(format!("Will last until {}", format_gt(&t))),
                cursor::MoveToNextLine(1),
            ).ok();
            },
            _ => { }
        }
    }

    // Clothes
    let cl_col_base = 178;
    execute!(w,
        cursor::MoveTo(cl_col_base, 0),
        style::SetForegroundColor(style::Color::DarkBlue),
        style::Print("Clothes"),
        style::SetForegroundColor(style::Color::Blue),
        cursor::MoveToNextLine(1),
    ).ok();
    for item_name in person.body.clothes.borrow().iter() {
        execute!(w,
            cursor::MoveToColumn(cl_col_base + 3),
            style::Print(format!("{}", item_name)),
            cursor::MoveToNextLine(1),
        ).ok();
        match person.inventory.items.borrow().get(item_name) {
            Some(item) => {
                match item.clothes() {
                    Some(c) => {
                        execute!(w,
                            cursor::MoveToColumn(cl_col_base + 6),
                            style::Print(format!("Cold protection: {}%", c.cold_resistance())),
                            cursor::MoveToNextLine(1),
                            cursor::MoveToColumn(cl_col_base + 6),
                            style::Print(format!("Water protection: {}%", c.water_resistance())),
                            cursor::MoveToNextLine(1),
                        ).ok();
                    },
                    _ => { }
                }
            },
            _ => { }
        }
    }
    match person.body.clothes_group() {
        Some(group) => {
            execute!(w,
                cursor::MoveToColumn(cl_col_base + 3),
                style::Print(format!("Has complete clothes set: {}", group.name)),
                cursor::MoveToNextLine(1),
                cursor::MoveToColumn(cl_col_base + 6),
                style::Print(format!("Bonus water protection: {}%", group.bonus_water_resistance)),
                cursor::MoveToNextLine(1),
                cursor::MoveToColumn(cl_col_base + 6),
                style::Print(format!("Bonus cold protection: {}%", group.bonus_cold_resistance)),
                cursor::MoveToNextLine(1),
            ).ok();
        },
        _ => {
            execute!(w,
                cursor::MoveToColumn(cl_col_base + 3),
                style::Print("No complete clothes set"),
                cursor::MoveToNextLine(1)
            ).ok();
        }
    }
    execute!(w,
        cursor::MoveToColumn(cl_col_base + 3),
        style::Print(format!("Total water protection: {}%", person.body.total_water_resistance())),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(cl_col_base + 3),
        style::Print(format!("Total cold protection: {}%", person.body.total_cold_resistance())),
        cursor::MoveToNextLine(1),
    ).ok();

    // Body appliances
    let appl_col_base = 178;
    execute!(w,
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(appl_col_base),
        style::SetForegroundColor(style::Color::DarkBlue),
        style::Print("Body Appliances"),
        style::SetForegroundColor(style::Color::Blue),
        cursor::MoveToNextLine(1),
    ).ok();
    for appliance in person.body.appliances.borrow().iter() {
        execute!(w,
            cursor::MoveToColumn(appl_col_base + 3),
            style::Print(format!("{} on {:?}", appliance.item_name, appliance.body_part)),
            cursor::MoveToNextLine(1),
        ).ok();
    }

    w.flush().ok();
}

fn format_gt(t: &GameTimeC) -> String {
    format!("{}day {}hour {}min {:.0}sec", t.day,t.hour,t.minute, t.second)
}