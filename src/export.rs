use std::fs;

use pf_sandbox::package::Package;
use pf_sandbox::fighter::*;
use treeflection::context_vec::ContextVec;
use brawllib_rs;
use brawllib_rs::parse::{SectionData, ArcChildData};
use brawllib_rs::bres::BresChildData;
use brawllib_rs::chr0::Chr0;
use brawllib_rs::mdl0::bones::Bone;
use brawllib_rs::misc_section::HurtBox as BrawlHurtBox;
use cgmath::{Matrix4, Zero};

use action_map::action_name_to_indexes;

pub(crate) fn export(mod_path: Option<String>, export_fighters: &[String]) {
    // TODO: replace files with mod_path
    // TODO: use mod path as package name
    match fs::read_dir("data/brawl/fighter") {
        Ok(fighter_dir) => {
            let brawl_fighters = brawllib_rs::fighters(fighter_dir);

            let mut package = Package::open_or_generate("brawl").unwrap();
            package.fighters.clear();

            for brawl_fighter in brawl_fighters {
                if export_fighters.contains(&brawl_fighter.cased_fighter_name.to_lowercase()) || export_fighters.contains(&String::from("all")) {
                    let mut fighter = Fighter::default();
                    fighter.name = brawl_fighter.cased_fighter_name.clone();

                    let mut hurt_boxes = vec!();

                    for sub_arc in brawl_fighter.moveset.children {
                        match sub_arc.data {
                            ArcChildData::Sakurai (data) => {
                                for section in data.sections {
                                    if let SectionData::FighterData { attributes, misc, .. } = section.data {
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

                                        hurt_boxes = misc.hurt_boxes;
                                    }
                                }
                            }
                            _ => { }
                        }
                    }

                    // locate bones
                    let mut first_bone: Option<&Bone> = None;
                    if let Some(model) = brawl_fighter.models.get(0) {
                        println!("{:#?}", model.name);
                        for sub_arc in model.children.iter() {
                            match &sub_arc.data {
                                &ArcChildData::Arc (_) => {
                                    panic!("Not expecting arc at this level")
                                }
                                &ArcChildData::Bres (ref bres) => {
                                    for bres_child in bres.children.iter() {
                                        match &bres_child.data {
                                            &BresChildData::Bres (ref model) => {
                                                for model_child in model.children.iter() {
                                                    if model_child.name == format!("Fit{}00", brawl_fighter.cased_fighter_name) {
                                                        match &model_child.data {
                                                            &BresChildData::Mdl0 (ref model) => {
                                                                first_bone = model.bones.as_ref();
                                                            }
                                                            _ => { }
                                                        }
                                                    }
                                                }
                                            }
                                            &BresChildData::Mdl0 (_) => {
                                                panic!("Not expecting Mdl at this level");
                                            }
                                            _ => { }
                                        }
                                    }
                                }
                                _ => { }
                            }
                        }
                    }

                    // locate animations
                    let mut chr0s: Vec<&Chr0> = vec!();
                    for sub_arc in &brawl_fighter.motion.children {
                        match &sub_arc.data {
                            &ArcChildData::Arc (ref arc) => {
                                for sub_arc in &arc.children {
                                    match &sub_arc.data {
                                        &ArcChildData::Bres (ref bres) => {
                                            for bres_child in bres.children.iter() {
                                                match &bres_child.data {
                                                    &BresChildData::Bres (ref bres) => {
                                                        for bres_child in bres.children.iter() {
                                                            match &bres_child.data {
                                                                &BresChildData::Bres (_) => {
                                                                    panic!("Not expecting bres at this level");
                                                                }
                                                                &BresChildData::Chr0 (ref chr0) => {
                                                                    chr0s.push(chr0);
                                                                }
                                                                _ => { }
                                                            }
                                                        }
                                                    }
                                                    &BresChildData::Chr0 (_) => {
                                                        panic!("Not expecting Chr0 at this level");
                                                    }
                                                    _ => { }
                                                }
                                            }
                                        }
                                        &ArcChildData::Arc (_) => {
                                            //panic!("Not expecting arc at this level"); // TODO: Whats here
                                        }
                                        _ => { }
                                    }
                                }
                            }
                            &ArcChildData::Bres (_) => {
                                panic!("Not expecting bres at this level");
                            }
                            _ => { }
                        }
                    }

                    // create fighter actions
                    if let Some(first_bone) = first_bone {
                        for chr0 in chr0s {
                            let mut frames = ContextVec::new();
                            for i in 0..chr0.num_frames {
                                let mut frame = ActionFrame::default();

                                // TODO: use frame to create animation data, parse to colboxes or something
                                let (colboxes, links) = gen_colboxes(
                                    first_bone,
                                    Matrix4::<f32>::zero(),
                                    -1, // starts with no parent
                                    &hurt_boxes
                                );
                                frame.colboxes = ContextVec::from_vec(colboxes);
                                frame.colbox_links = links;

                                frames.push(frame);
                            }

                            let action = ActionDef {
                                iasa: 0,
                                frames
                            };

                            for index in action_name_to_indexes(&chr0.name) {
                                fighter.actions[index] = action.clone();
                            }
                        }
                    }

                    package.fighters.push(brawl_fighter.cased_fighter_name, fighter);
                }
            }

            package.meta.title = String::from("Brawl");
            package.save();
        }
        Err(_) => {
            println!("'data' directory incorrectly setup.");
        }
    }
}

// Hurtboxes are long, starting at the referenced bones parent stretched to the referenced bone.
// Hitboxes are circle at the bone point (appear long because PM debug mode uses interpolation with the previous frames hitbox)
// Currently creates a single colbox for every bone, creating links between them. (This is nice as
// it means we have a nicer 'model' to tweak/animate with.
// TODO: Need to take hitbox from previous frame and interpolate into this frame as an extra ColBox.
//       Can probably call gen_colboxes on current_frame and prev_frame then add in the interpolation of the previous frames hitboxes.

fn gen_colboxes(
    bone: &Bone,
    parent_transform: Matrix4<f32>,
    parent_colbox_index: i64,
    hurtboxes: &[BrawlHurtBox]
    /* TODO: animation_data */
) -> (Vec<CollisionBox>, Vec<CollisionBoxLink>) {
    let mut colbox_index = parent_colbox_index;
    let mut colboxes = vec!();
    let mut colbox_links = vec!();

    // TODO: Might need this for handling animations? Otherwise just delete it
    // transform position
    let _transform = if parent_colbox_index == -1 {
        bone.gen_transform()
    } else {
        parent_transform * bone.gen_transform()
    };

    for hurtbox in hurtboxes {
        // create hurtbox
        if bone.index == hurtbox.bone_index as i32 {
            colboxes.push(CollisionBox {
                //point: (transform.w.x, transform.w.y),
                point: (bone.transform.w.x + hurtbox.offset.x, bone.transform.w.y + hurtbox.offset.y), // TODO: pretty sure these should be applied via a matrix, so the translations are orientated correctly
                radius: hurtbox.radius,
                role: CollisionBoxRole::Hurt (HurtBox::default()),
            });

            colbox_index += 1;

            if let Some(first_child_bone) = bone.children.get(bone.children.len()-1) { // TODO: This is weird but seems to work, maybe I need to do it for all children instead? Maybe there is a value that says which child to use.
                colboxes.push(CollisionBox {
                    //point: (transform.w.x, transform.w.y),
                    point: (first_child_bone.transform.w.x + hurtbox.offset.x, first_child_bone.transform.w.y + hurtbox.offset.y),
                    radius: hurtbox.radius,
                    role: CollisionBoxRole::Hurt (HurtBox::default()),
                });

                colbox_links.push(CollisionBoxLink {
                    one: colbox_index as usize,
                    two: colbox_index as usize + 1,
                    link_type: LinkType::MeldFirst,
                });
                colbox_index += 1;
            }

            break;
        }

        // create hitbox
        // TODO
    }

    for child in bone.children.iter() {
        let (descendents, links) = gen_colboxes(
            child,
            bone.transform.clone(), // transform,
            colbox_index,
            hurtboxes
        );
        colbox_index += descendents.len() as i64;
        colboxes.extend(descendents);
        colbox_links.extend(links);
    }

    (colboxes, colbox_links)
}

// TODO: In order to take into account the z position for ordering, maybe use a temp struct like:
// Holder {
//      colbox: CollisionBox,
//      z:      f32,
// }
