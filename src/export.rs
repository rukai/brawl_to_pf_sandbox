use std::fs;

use pf_sandbox::package::Package;
use pf_sandbox::fighter::*;
use treeflection::context_vec::ContextVec;
use brawllib_rs::high_level_fighter::HighLevelFighter;
use brawllib_rs::fighter::Fighter as BrawlFighter;
use brawllib_rs::script_ast::EdgeSlide;
use noisy_float::prelude::*;

use cgmath::Matrix4;

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
                    fighter.weight = attributes.weight / 100.0;
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

                    let ledge_grab_box = if let Some(ledge_grab) = hl_fighter.ledge_grabs.get(0) {
                        Some(LedgeGrabBox {
                            x1: ledge_grab.x,
                            y1: ledge_grab.y,
                            x2: ledge_grab.x + ledge_grab.width,
                            y2: ledge_grab.y + ledge_grab.height,
                        })
                    } else {
                        None
                    };

                    // create fighter actions
                    for hl_action in hl_fighter.actions {
                        let mut frames = ContextVec::new();
                        for hl_frame in hl_action.frames {
                            // https://smashboards.com/threads/all-aboard-the-pain-train-hurtboxes.301220/
                            // Hurtboxes like hitboxes have a reference to a single bone that determines its position + an offset vector.
                            // However hurtboxes have radius and stretch values that give them the (usually) cylindrical shape.
                            // We create two linked colboxes for each hurtbox, this is not accurate but is the best we can do.
                            let mut colboxes = vec!();
                            let mut colbox_links = vec!();
                            let mut render_order = vec!();

                            for hurt_box in hl_frame.hurt_boxes {
                                let transform = hurt_box.bone_matrix * Matrix4::<f32>::from_translation(hurt_box.hurt_box.offset);

                                colboxes.push(CollisionBox {
                                    point: (transform.w.z, transform.w.y),
                                    radius: hurt_box.hurt_box.radius, // TODO: radius is not accurate, needs to take Z offset into account; however it certainly looks fine, so eh
                                    role: CollisionBoxRole::Hurt (HurtBox::default()),
                                });

                                // TODO: Works well when there are two stretch offsets, if there are three then it will likely be wonky.
                                //       Maybe use a heuristic for using a single colbox with custom radius if there is a large 3rd offset.
                                //       If this isnt actually an issue just delete comment
                                let s = hurt_box.hurt_box.stretch;
                                if s.x != 0.0 || s.y != 0.0 || s.z != 0.0 { // If there are no stretch offsets we only need one colbox
                                    let stretch_transform = transform * Matrix4::<f32>::from_translation(s);
                                    colboxes.push(CollisionBox {
                                        point: (stretch_transform.w.z, stretch_transform.w.y),
                                        radius: hurt_box.hurt_box.radius,
                                        role: CollisionBoxRole::Hurt (HurtBox::default()),
                                    });

                                    colbox_links.push(CollisionBoxLink {
                                        one: colboxes.len() - 2,
                                        two: colboxes.len() - 1,
                                        link_type: LinkType::MeldFirst,
                                    });

                                    render_order.push((
                                        RenderOrder::Link(colbox_links.len() - 1),
                                        (transform.w.x + stretch_transform.w.x) / 2.0, // average of the z values for both colboxes
                                    ));
                                }
                                else {
                                    render_order.push((
                                        RenderOrder::Colbox(colboxes.len() - 1),
                                        transform.w.x,
                                    ));
                                }
                            }

                            for hit_box in hl_frame.hit_boxes {
                                let hb = hit_box.hit_box;
                                colboxes.push(CollisionBox {
                                    point: (hit_box.position.z, hit_box.position.y),
                                    radius: hb.size,
                                    role: CollisionBoxRole::Hit (HitBox {
                                        shield_damage:     hb.shield_damage as f32,
                                        damage:            hb.damage as f32,
                                        bkb:               hb.bkb as f32,
                                        kbg:               hb.kbg as f32 / 100.0,
                                        angle:             hb.trajectory as f32,
                                        hitstun:           HitStun::default(), // TODO: tweak to brawl/pm values
                                        enable_clang:      hb.clang,
                                        enable_rebound:    hb.clang, // TODO: are these the same thing?
                                        effect:            HitboxEffect::None,
                                    }),
                                });
                            }

                            for hit_box in hl_frame.special_hit_boxes {
                                let hb = hit_box.hit_box.hitbox_args;
                                colboxes.push(CollisionBox {
                                    point: (hit_box.position.z, hit_box.position.y),
                                    radius: hb.size,
                                    role: CollisionBoxRole::Hit (HitBox {
                                        shield_damage:     hb.shield_damage as f32,
                                        damage:            hb.damage as f32,
                                        bkb:               hb.bkb as f32,
                                        kbg:               hb.kbg as f32 / 100.0,
                                        angle:             hb.trajectory as f32,
                                        hitstun:           HitStun::default(), // TODO: tweak to brawl/pm values
                                        enable_clang:      hb.clang,
                                        enable_rebound:    hb.clang, // TODO: are these the same thing?
                                        effect:            HitboxEffect::None,
                                    }),
                                });
                            }

                            render_order.sort_by_key(|x| n32(x.1));

                            // TODO: Hitboxes

                            // just modify a default frame because we are lazy
                            let mut frame = ActionFrame::default();
                            frame.colboxes = ContextVec::from_vec(colboxes);
                            frame.colbox_links = colbox_links;
                            frame.render_order = render_order.iter().map(|x| x.0.clone()).collect();
                            frame.ledge_grab_box = ledge_grab_box.clone(); // TODO: Only some frames have ledge_grab_boxes, they can also have different ledge_grab_box values. This should probably be handled by brawllib_rs
                            // TODO: The offset returned by apply_chr0_to_bones doesnt seem to change, figure out why
                            //frame.set_x_vel = hl_frame.animation_velocity.map(|vel| vel.z);
                            //frame.set_y_vel = hl_frame.animation_velocity.map(|vel| vel.y);
                            frame.ledge_cancel = match hl_frame.edge_slide {
                                | EdgeSlide::Airbourne
                                | EdgeSlide::SlideOff    => true,
                                | EdgeSlide::StayOn
                                | EdgeSlide::Unknown (_) => false
                            };

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
