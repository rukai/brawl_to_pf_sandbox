extern crate cgmath;
extern crate pf_sandbox;
extern crate brawllib_rs;

use std::fs;
use std::env;

use pf_sandbox::package::Package;
use pf_sandbox::fighter::*;

use brawllib_rs::parse::{SectionData, ArcChildData, Arc};

fn main() {
    let mut args = env::args();
    args.next();
    let arg = args.next().unwrap_or(String::from(""));

    match fs::read_dir("data/brawl/fighter") {
        Ok(fighter_dir) => {
            let brawl_fighters = brawllib_rs::fighters(fighter_dir);

            let mut package = Package::open_or_generate("brawl").unwrap();
            package.fighters.clear();

            if "all" == arg {
                println!("{:#?}", brawl_fighters);
            }

            for brawl_fighter in brawl_fighters.iter() {
                if brawl_fighter.cased_fighter_name == arg {
                    println!("{:#?}", brawl_fighter);
                    return;
                }
            }

            for brawl_fighter in brawl_fighters {
                let mut fighter = Fighter::default();
                fighter.name = brawl_fighter.cased_fighter_name.clone();

                for sub_arc in brawl_fighter.moveset.children {
                    match sub_arc.data {
                        ArcChildData::Sakurai (data) => {
                            for section in data.sections {
                                if let SectionData::FighterData { attributes, .. } = section.data {
                                    fighter.air_jumps = attributes.num_jumps as u64 - 1;
                                    fighter.weight = attributes.weight;
                                    fighter.gravity = -attributes.gravity;
                                    fighter.terminal_vel = -attributes.term_vel;
                                    fighter.fastfall_terminal_vel = -attributes.fastfall_velocity;
                                    fighter.jump_y_init_vel = attributes.jump_y_init_vel;
                                    fighter.jump_y_init_vel_short = attributes.jump_y_init_vel_short;
                                    fighter.jump_x_init_vel = attributes.jump_x_init_vel;
                                    fighter.jump_x_term_vel = attributes.jump_x_init_term_vel;
                                    fighter.jump_x_vel_ground_mult = attributes.jump_x_vel_ground_mult;
                                    fighter.air_mobility_a = attributes.air_mobility_a;
                                    fighter.air_mobility_b = attributes.air_mobility_b;
                                    fighter.air_x_term_vel = attributes.air_x_term_vel;
                                    fighter.air_friction = attributes.air_friction_x;
                                    fighter.air_jump_x_vel = 1.0; // attributes.air_jump_x_mult; // TODO: store air_jump in an enum to allow both brawl and melee physics
                                    fighter.air_jump_y_vel = 2.0; // attributes.air_jump_y_mult;
                                    fighter.walk_init_vel = attributes.walk_init_vel;
                                    fighter.walk_acc = attributes.walk_acc;
                                    fighter.walk_max_vel = attributes.walk_max_vel;
                                    fighter.slow_walk_max_vel = 0.0_f32.max(attributes.walk_max_vel - 0.5);
                                    fighter.dash_init_vel = attributes.dash_init_vel;
                                    fighter.dash_run_acc_a = attributes.dash_run_acc_a;
                                    fighter.dash_run_acc_b = attributes.dash_run_acc_b;
                                    fighter.dash_run_term_vel = attributes.dash_run_term_vel;
                                    fighter.friction = attributes.ground_friction;
                                    // fighter.aerialdodge_mult = // TODO: PM custom?
                                    // fighter.aerialdodge_drift_frame = attributes. // TODO: PM custom?
                                    fighter.forward_roll = true;
                                    fighter.backward_roll = true;
                                    fighter.spot_dodge = true;
                                    fighter.lcancel = None; // TODO: handle for PM
                                    fighter.shield = Some(Shield {
                                        // TODO: shield_strength !??!?!
                                        stick_lock: false,
                                        stick_mult: 3.0,
                                        offset_x:   1.0,
                                        offset_y:   8.0,
                                        break_vel:  attributes.shield_break_vel,
                                        scaling:    attributes.shield_size,
                                        hp_scaling: 1.15,
                                        hp_max:     60.0,
                                        hp_regen:   0.1,
                                        hp_cost:    0.28,
                                    });
                                    fighter.power_shield = Some(PowerShield {
                                        reflect_window: Some(2),
                                        parry: Some(PowerShieldEffect {
                                            window: 4,
                                            duration: 4,
                                        }),
                                        enemy_stun: None,
                                    });
                                    fighter.tech = Some(Tech::default());
                                    fighter.missed_tech_forced_getup = Some(220); // TODO
                                    fighter.run_turn_flip_dir_frame = attributes.flip_dir_frame as u64; // TODO
                                    fighter.tilt_turn_flip_dir_frame = attributes.flip_dir_frame as u64;
                                    fighter.tilt_turn_into_dash_iasa = attributes.flip_dir_frame as u64;
                                }
                            }
                        }
                        _ => { }
                    }
                }

                process_motion(&brawl_fighter.motion);

                for action in fighter.actions.iter_mut() {
                    for frame in action.frames.iter_mut() {
                        frame.colboxes.insert(0, CollisionBox {
                            point: (0.0, 4.0),
                            radius: 4.0,
                            role: CollisionBoxRole::Hurt (HurtBox::default()),
                        });
                    }
                }

                package.fighters.push(brawl_fighter.cased_fighter_name, fighter);
            }

            package.meta.title = String::from("Brawl");
            package.save();
        }
        Err(_) => {
            println!("'data' directory incorrectly setup.");
        }
    }

    fn process_motion(arc: &Arc) {
        for sub_arc in &arc.children {
            match &sub_arc.data {
                &ArcChildData::Arc (ref arc) => {
                    process_motion(arc);
                }
                &ArcChildData::Bres (ref _bres) => {
                }
                _ => { }
            }
        }
    }
}

