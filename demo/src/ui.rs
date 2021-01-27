use crate::events::ZaraEventsListener;

use zara::utils::GameTimeC;

use std::io::{self, Write};

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
        cursor::MoveTo(1, 1)
    );

    let gt = person.environment.game_time.to_contract();

    // Game time
    execute!(w,
        cursor::MoveTo(0, 0),
        style::SetForegroundColor(style::Color::DarkCyan),
        style::Print("Game Time: "),
        style::SetForegroundColor(style::Color::Cyan),
        style::Print(format_gt(&gt)),
    );

    let vitals_h = 12;
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
    );

    // Inventory
    let inv_col_base = 40;
    execute!(w,
        cursor::MoveTo(inv_col_base, 0),
        style::SetForegroundColor(style::Color::DarkBlue),
        style::Print(format!("Inventory (total weight {:.1}g)", person.inventory.get_weight())),
        style::SetForegroundColor(style::Color::Blue),
        cursor::MoveToNextLine(1),
    );
    for (name, item) in person.inventory.items.borrow().iter() {
        execute!(w,
            cursor::MoveToColumn(inv_col_base+4),
            style::Print(format!("{} ({})", name, item.get_count())),
            cursor::MoveToNextLine(1),
            cursor::MoveToColumn(inv_col_base+6),
            style::Print(format!("Total weight: {}g", item.get_total_weight())),
            cursor::MoveToNextLine(1)
        );
        match item.consumable() {
            Some(c) => {
                execute!(w,
                    cursor::MoveToColumn(inv_col_base+6),
                    style::Print("Is a consumable")
                );
                if c.is_food() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(food)")
                    );
                }
                if c.is_water() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(water)")
                    );
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
                );
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
                        );
                    },
                    _ => {
                        execute!(w,
                            cursor::MoveToColumn(inv_col_base+8),
                            style::Print("cannot spoil"),
                            cursor::MoveToNextLine(1)
                        );
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
                );
                if a.is_body_appliance() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(body appliance)")
                    );
                }
                if a.is_injection() {
                    execute!(w,
                        cursor::MoveToColumn(inv_col_base+22),
                        style::Print("(injection)")
                    );
                }
                execute!(w,
                    cursor::MoveToNextLine(1)
                );
            },
            _ => { }
        }
    }

    // Misc stats
    execute!(w,
        cursor::MoveToNextLine(1),
        style::SetForegroundColor(style::Color::DarkYellow),
        cursor::MoveTo(0, vitals_h),
        style::Print("Stats"),
        style::SetForegroundColor(style::Color::Yellow),
        cursor::MoveToNextLine(1)
    );
    match person.body.last_sleep_time() {
        Some(t) => {
            execute!(w,
                cursor::MoveToColumn(3),
                style::Print(format!("Last time slept: {}", format_gt(&t))),
                cursor::MoveToNextLine(1)
            );
        },
        _ => {
            execute!(w,
                cursor::MoveToColumn(3),
                style::Print("Not slept yet"),
                cursor::MoveToNextLine(1)
            );
        }
    }
    execute!(w,
        cursor::MoveToColumn(3),
        style::Print(format!("Is sleeping now? {}", if person.body.is_sleeping() { "yes" } else { "no" })),
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(3),
        style::Print(format!("Blood loss? {}", if person.health.is_blood_loss() { "yes" } else { "no" })),
        cursor::MoveToNextLine(1)
    );

    // Diseases
    let dis_col_base = 90;
    execute!(w,
        cursor::MoveTo(dis_col_base, 0),
        style::SetForegroundColor(style::Color::DarkMagenta),
        style::Print("Diseases"),
        style::SetForegroundColor(style::Color::Magenta),
        cursor::MoveToNextLine(1),
    );
    for (name, item) in person.health.diseases.borrow().iter() {
        let active = item.is_active(&gt);
        let info = format!("{} ({})", name, if active { "active now" } else { "scheduled" });
        execute!(w,
            cursor::MoveToColumn(dis_col_base+3),
            style::Print(format!("{}", info)),
            cursor::MoveToNextLine(1),
        );
        if !active {
            let at = item.activation_time();
            execute!(w,
                cursor::MoveToColumn(dis_col_base+6),
                style::Print(format!("Will activate at {}", format_gt(&at))),
                cursor::MoveToNextLine(1),
            );
        } else {
            match item.get_active_stage(&gt) {
                Some(st) => {
                    execute!(w,
                        cursor::MoveToColumn(dis_col_base+6),
                        style::Print(format!("Is on level {:?} {}%", st.info.level, st.percent_active(&gt))),
                        cursor::MoveToNextLine(1)
                    );
                    match item.end_time() {
                        Some(t) => {
                            execute!(w,
                                cursor::MoveToColumn(dis_col_base+6),
                                style::Print(format!("Will end at {}", format_gt(&t))),
                                cursor::MoveToNextLine(1)
                            );
                        },
                        _ => {
                            execute!(w,
                                cursor::MoveToColumn(dis_col_base+6),
                                style::Print("Will not end"),
                                cursor::MoveToNextLine(1)
                            );
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
        );
    }

    // Injuries
    let inj_col_base = 90;
    execute!(w,
        cursor::MoveToNextLine(1),
        cursor::MoveToColumn(inj_col_base+1),
        style::SetForegroundColor(style::Color::DarkMagenta),
        style::Print("Injuries"),
        style::SetForegroundColor(style::Color::Magenta),
        cursor::MoveToNextLine(1),
    );
    for (key, item) in person.health.injuries.borrow().iter() {
        let active = item.is_active(&gt);
        let info = format!("{} on {:?} ({})", key.injury, key.body_part, if active { "active now" } else { "scheduled" });
        execute!(w,
            cursor::MoveToColumn(inj_col_base+3),
            style::Print(format!("{}", info)),
            cursor::MoveToNextLine(1),
        );
        if !active {
            let at = item.activation_time();
            execute!(w,
                cursor::MoveToColumn(inj_col_base+6),
                style::Print(format!("Will activate at {}", format_gt(&at))),
                cursor::MoveToNextLine(1),
            );
        } else {
            match item.get_active_stage(&gt) {
                Some(st) => {
                    execute!(w,
                        cursor::MoveToColumn(inj_col_base+6),
                        style::Print(format!("Is on level {:?} {}%", st.info.level, st.percent_active(&gt))),
                        cursor::MoveToNextLine(1)
                    );
                    match item.end_time() {
                        Some(t) => {
                            execute!(w,
                                cursor::MoveToColumn(inj_col_base+6),
                                style::Print(format!("Will end at {}", format_gt(&t))),
                                cursor::MoveToNextLine(1)
                            );
                        },
                        _ => {
                            execute!(w,
                                cursor::MoveToColumn(inj_col_base+6),
                                style::Print("Will not end"),
                                cursor::MoveToNextLine(1)
                            );
                        }
                    }
                },
                _ => { }
            }
        }
        execute!(w,
            cursor::MoveToColumn(inj_col_base+6),
            style::Print(format!("Needs treatment? {}", if item.needs_treatment { "yes" } else { "no (will self-heal)" })),
            cursor::MoveToNextLine(1),
        );
        execute!(w,
            cursor::MoveToColumn(inj_col_base+6),
            style::Print(format!("Blood forcibly stopped? {}", if item.is_blood_stopped() { "yes" } else { "no" })),
            cursor::MoveToNextLine(1),
        );
    }

    w.flush();
}

fn format_gt(t: &GameTimeC) -> String {
    format!("{}day {}hour {}min {:.0}sec", t.day,t.hour,t.minute, t.second)
}

fn read_char() -> Result<char> {
    loop {
        if let Ok(TermEvent::Key(KeyEvent {
                                     code: KeyCode::Char(c),
                                     ..
                                 })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}