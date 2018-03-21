use std::fs;

use pf_sandbox::package::Package;
use pf_sandbox::fighter::*;
use treeflection::context_vec::ContextVec;
use brawllib_rs::high_level_fighter::HighLevelFighter;
use brawllib_rs::fighter::Fighter as BrawlFighter;
use noisy_float::prelude::*;

use action_map::action_name_to_indexes;

pub(crate) fn export(mod_path: Option<String>, export_fighters: &[String]) {
    let mod_fighter_dir = if let &Some(ref mod_path) = &mod_path {
        if let Ok(dir) = fs::read_dir(format!("data/{}/fighter", mod_path)) {
            Some(dir)
        } else {
            println!("Mod directory '{}' does not exist.", mod_path);
            return;
        }
    } else {
        None
    };

    match fs::read_dir("data/brawl/fighter") {
        Ok(fighter_dir) => {
            let brawl_fighters = BrawlFighter::load(fighter_dir, mod_fighter_dir, true);

            let mut package = if let Some(name) = mod_path.clone() {
                Package::open_or_generate(&name).unwrap()
            } else {
                Package::open_or_generate("brawl").unwrap()
            };
            package.fighters.clear();

            for brawl_fighter in brawl_fighters {
                if export_fighters.contains(&brawl_fighter.cased_name.to_lowercase()) || export_fighters.contains(&String::from("all")) {
                    let hl_fighter = HighLevelFighter::new(&brawl_fighter);
                    info!("starting export fighter: {}", brawl_fighter.cased_name);
                    let mut fighter = Fighter::default();
                    fighter.name = hl_fighter.name.clone();

                    let attributes = hl_fighter.attributes;
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

                    // create fighter actions
                    for hl_action in hl_fighter.actions {
                        let mut frames = ContextVec::new();
                        for hl_frame in hl_action.frames {
                            // Hurtboxes are long, starting at the referenced bone stretched to the child bone (appears to be the last child?)
                            // We create two linked colboxes for each hurtbox
                            let mut colboxes = vec!();
                            let mut colbox_links = vec!();
                            let mut hurt_boxes = hl_frame.hurt_boxes.clone();
                            hurt_boxes.sort_by_key(|hurt_box|
                                // TODO: might be able to get more exact, by individually checking both start and end
                                // But I want to fix the wonky animation bug first, so I can properly evaluate if this works.
                                n32(
                                    if let Some(end) = hurt_box.end {
                                        (hurt_box.start.z + end.z) / 2.0
                                    } else {
                                        hurt_box.start.z
                                    }
                                )
                            );
                            for hurt_box in hurt_boxes {
                                colboxes.push(CollisionBox {
                                    point: (hurt_box.start.x, hurt_box.start.y),
                                    radius: hurt_box.radius,
                                    role: CollisionBoxRole::Hurt (HurtBox::default()),
                                });

                                if let Some(end) = hurt_box.end {
                                    colboxes.push(CollisionBox {
                                        point: (end.x, end.y),
                                        radius: hurt_box.radius,
                                        role: CollisionBoxRole::Hurt (HurtBox::default()),
                                    });

                                    colbox_links.push(CollisionBoxLink {
                                        one: colboxes.len() - 2,
                                        two: colboxes.len() - 1,
                                        link_type: LinkType::MeldFirst,
                                    });
                                }
                            }

                            // TODO: Hitboxes

                            // just modify a default frame because we are lazy
                            let mut frame = ActionFrame::default();
                            frame.colboxes = ContextVec::from_vec(colboxes);
                            frame.colbox_links = colbox_links;

                            frames.push(frame);
                        }

                        let action = ActionDef {
                            iasa: hl_action.iasa as i64,
                            frames
                        };

                        for index in action_name_to_indexes(&hl_action.name) {
                            fighter.actions[index] = action.clone();
                        }
                    }

                    package.fighters.push(brawl_fighter.cased_name, fighter);
                }
            }

            package.meta.title = mod_path.unwrap_or(String::from("Brawl"));
            package.save();
        }
        Err(_) => {
            println!("'data' directory incorrectly setup.");
        }
    }
}
