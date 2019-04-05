use std::fs::File;
use std::fs;
use std::io::Write;

use handlebars::Handlebars;
use rayon::prelude::*;
use brawllib_rs::high_level_fighter::CollisionBoxValues;
use brawllib_rs::script_ast::{AngleFlip, HitBoxEffect};
use brawllib_rs::renderer;

use crate::brawl_data::{BrawlMods, SubactionLinks};
use crate::page::NavLink;
use crate::process_scripts;
use crate::assets::AssetPaths;

pub fn generate(handlebars: &Handlebars, brawl_mods: &BrawlMods, assets: &AssetPaths) {
    for brawl_mod in &brawl_mods.mods {
        let mod_links = brawl_mods.gen_mod_links(brawl_mod.name.clone());

        for fighter in &brawl_mod.fighters {
            fighter.fighter.subactions.par_iter().enumerate().for_each(|(index, subaction)| {
                let fighter_name = &fighter.fighter.name;
                // Originally tried to handle scripts as a table of frame,main,gfx,sfx,other but
                // that would require simulating the scripts and with what inputs???
                // How to handle infinite loops???
                //
                // It would be nice to simulate the whole script displaying the current line in browser but that would need wasm and stuff.
                // Im not rewriting ScriptRunner in javascript!!!
                //
                // So instead I just dump the scripts one by one, linking to other pages for external function calls
                // Then one day I can come and add script running via wasm.
                let script_main  = process_scripts::process_events(&subaction.scripts.script_main.block.events, false, brawl_mod, fighter);
                let script_gfx   = process_scripts::process_events(&subaction.scripts.script_gfx.block.events, false, brawl_mod, fighter);
                let script_sfx   = process_scripts::process_events(&subaction.scripts.script_sfx.block.events, false, brawl_mod, fighter);
                let script_other = process_scripts::process_events(&subaction.scripts.script_other.block.events, false, brawl_mod, fighter);

                let mut frame_buttons = vec!();
                for (mut index, frame) in subaction.frames.iter().enumerate() {
                    index += 1;
                    let class = if !frame.hit_boxes.is_empty() {
                        String::from("hitbox-frame-button")
                    } else if index > subaction.iasa {
                        String::from("iasa-frame-button")
                    } else {
                        String::from("standard-frame-button")
                    };
                    frame_buttons.push(FrameButton { index, class });
                }

                // generate auto cancel ranges
                let mut auto_cancel = String::new();
                let mut landing_lag_prev = true;
                let mut last_frame_change = 0;
                for (index, frame) in subaction.frames.iter().enumerate() {
                    if frame.landing_lag && !landing_lag_prev {
                        if auto_cancel.len() != 0 {
                            auto_cancel.push_str(", ");
                        }
                        auto_cancel.push_str(&format!("{}-{}", last_frame_change + 1, index));
                    }
                    if landing_lag_prev != frame.landing_lag {
                        last_frame_change = index;
                        landing_lag_prev = frame.landing_lag;
                    }
                }
                if !landing_lag_prev && last_frame_change != 0 {
                    if auto_cancel.len() != 0 {
                        auto_cancel.push_str(", ");
                    }
                    auto_cancel.push_str(&format!("{}-{}", last_frame_change + 1, subaction.frames.len()));
                }

                let mut invincible = String::new();
                let mut intangible = String::new();
                let mut partial_invincible = String::new();
                let mut partial_intangible = String::new();
                let mut start_invincible = None;
                let mut start_intangible = None;
                let mut start_partial_intangible = None;
                let mut start_partial_invincible = None;
                let mut reverse_direction = String::new();
                for (i, frame) in subaction.frames.iter().enumerate() {
                    let all_invincible = frame.hurt_boxes.iter().all(|x| x.state.is_invincible());
                    let any_invincible = frame.hurt_boxes.iter().any(|x| x.state.is_invincible());
                    let all_intangible = frame.hurt_boxes.iter().all(|x| x.state.is_intangible());
                    let any_intangible = frame.hurt_boxes.iter().any(|x| x.state.is_intangible());

                    // fullly invincible
                    if all_invincible && start_invincible.is_none() {
                        start_invincible = Some(i);
                    }
                    if !all_invincible {
                        if let Some(start) = start_invincible.take() {
                            if invincible.len() > 0 {
                                invincible.push_str(", ");
                            }
                            invincible.push_str(&format!("{}-{}", start + 1, i));
                        }
                    }

                    // partially invincible
                    if !all_invincible && any_invincible && start_partial_invincible.is_none() {
                        start_partial_invincible = Some(i);
                    }
                    if !any_invincible {
                        if let Some(start) = start_partial_invincible.take() {
                            if partial_invincible.len() > 0 {
                                partial_invincible.push_str(", ");
                            }
                            partial_invincible.push_str(&format!("{}-{}", start + 1, i));
                        }
                    }

                    // fully intangible
                    if all_intangible && start_intangible.is_none() {
                        start_intangible = Some(i);
                    }
                    if !all_intangible {
                        if let Some(start) = start_intangible.take() {
                            if intangible.len() > 0 {
                                intangible.push_str(", ");
                            }
                            intangible.push_str(&format!("{}-{}", start + 1, i));
                        }
                    }

                    // partially intangible
                    if !all_intangible && any_intangible && start_partial_intangible.is_none() {
                        start_partial_intangible = Some(i);
                    }
                    if !any_intangible {
                        if let Some(start) = start_partial_intangible.take() {
                            if partial_intangible.len() > 0 {
                                partial_intangible.push_str(", ");
                            }
                            partial_intangible.push_str(&format!("{}-{}", start + 1, i));
                        }
                    }

                    if frame.reverse_direction {
                        if reverse_direction.len() > 0 {
                            reverse_direction.push_str(", ");
                        }
                        reverse_direction.push_str(&format!("{}", i + 1));
                    }
                }

                // handle invincible/intangible states that were not turned off
                if let Some(start) = start_invincible.take() {
                    if invincible.len() > 0 {
                        invincible.push_str(", ");
                    }
                    invincible.push_str(&format!("{}-{}", start + 1, subaction.frames.len()));
                }
                if let Some(start) = start_partial_invincible.take() {
                    if partial_invincible.len() > 0 {
                        partial_invincible.push_str(", ");
                    }
                    partial_invincible.push_str(&format!("{}-{}", start + 1, subaction.frames.len()));
                }
                if let Some(start) = start_intangible.take() {
                    if intangible.len() > 0 {
                        intangible.push_str(", ");
                    }
                    intangible.push_str(&format!("{}-{}", start + 1, subaction.frames.len()));
                }
                if let Some(start) = start_partial_intangible.take() {
                    if partial_intangible.len() > 0 {
                        partial_intangible.push_str(", ");
                    }
                    partial_intangible.push_str(&format!("{}-{}", start + 1, subaction.frames.len()));
                }

                let mut attributes = vec!();
                attributes.push(Attribute {
                    name: r#"<abbr title="Interruptible As Soon As. The first frame the subaction can be interrupted with another subaction.">IASA</abbr>"#.into(),
                    value: (subaction.iasa + 1).to_string()
                });
                if auto_cancel.len() > 0 {
                    attributes.push(Attribute {
                        name: r#"<abbr title="The frames during which landing will auto cancel.">Auto Cancel Window</abbr>"#.into(),
                        value: auto_cancel.clone()
                    });

                    let auto_cancel_lag = fighter.fighter.attributes.light_landing_lag as i32 + 1;
                    attributes.push(Attribute {
                        name: r#"<abbr title="The amount of lag taken when auto cancelling. This is the same amount as when landing outside of an attack">Auto Cancel Lag</abbr>"#.into(),
                        value: auto_cancel_lag.to_string()
                    });
                }
                if let Some(landing_lag) = subaction.landing_lag {
                    attributes.push(Attribute {
                        name: r#"<abbr title="Number of frames of landing lag without l-cancelling">Landing Lag</abbr>"#.into(),
                        value: landing_lag.to_string()
                    });
                    attributes.push(Attribute {
                        name: r#"<abbr title="Number of frames of landing lag with l-cancelling. This is half of the regular landing lag, rounded down.">Landing Lag (L-Cancel)</abbr>"#.into(),
                        value: ((landing_lag / 2.0) as u32).to_string()
                    });
                }
                if invincible.len() > 0 {
                    attributes.push(Attribute {
                        name: r#"<abbr title="Frames where all of the characters hurtboxes are invincible.">Fully Invincible</abbr>"#.into(),
                        value: invincible.clone()
                    });
                }
                if intangible.len() > 0 {
                    attributes.push(Attribute {
                        name: r#"<abbr title="Frames where all of the characters hurtboxes are intangible.">Fully Intangible</abbr>"#.into(),
                        value: intangible.clone()
                    });
                }
                if partial_invincible.len() > 0 {
                    attributes.push(Attribute {
                        name: r#"<abbr title="Frames where some of the characters hurtboxes are invincible and some are vulnerable.">Partially Invincible</abbr>"#.into(),
                        value: partial_invincible.clone()
                    });
                }
                if partial_intangible.len() > 0 {
                    attributes.push(Attribute {
                        name: r#"<abbr title="Frames where some of the characters hurtboxes are intangible and some are vulnerable.">Partially Intangible</abbr>"#.into(),
                        value: partial_intangible.clone()
                    });
                }
                if reverse_direction.len() > 0 {
                    attributes.push(Attribute {
                        name: r#"<abbr title="Frames where the character turns to face the oppposite direction.">Direction Reverse Frames</abbr>"#.into(),
                        value: reverse_direction.clone()
                    });
                }


                let mut hitboxes_active = String::new();
                let mut start_hitboxes_active = None;
                for (i, frame) in subaction.frames.iter().enumerate() {
                    let has_hitboxes = frame.hit_boxes.len() > 0;

                    if has_hitboxes && start_hitboxes_active.is_none() {
                        start_hitboxes_active = Some(i);
                    }
                    if !has_hitboxes {
                        if let Some(start) = start_hitboxes_active.take() {
                            if hitboxes_active.len() > 0 {
                                hitboxes_active.push_str(", ");
                            }
                            hitboxes_active.push_str(&format!("{}-{}", start + 1, i));
                        }
                    }
                }

                // handle hitboxes that were not deleted
                if let Some(start) = start_hitboxes_active.take() {
                    if hitboxes_active.len() > 0 {
                        hitboxes_active.push_str(", ");
                    }
                    hitboxes_active.push_str(&format!("{}-{}", start + 1, subaction.frames.len()));
                }

                attributes.push(Attribute {
                    name: "Hitboxes active".into(),
                    value: hitboxes_active.clone(),
                });

                for set_id in 0..10 {
                    if subaction.frames.iter().any(|x| x.hitbox_sets_rehit[set_id]) {
                        let mut hitboxes_rehit = String::new();
                        for (i, frame) in subaction.frames.iter().enumerate() {
                            if frame.hitbox_sets_rehit[set_id] {
                                if hitboxes_rehit.len() > 0 {
                                    hitboxes_rehit.push_str(", ");
                                }
                                hitboxes_rehit.push_str(&format!("{}", i + 1));
                            }
                        }

                        // All hitboxes within the hitbox's set can hit the same enemy again.
                        attributes.push(Attribute {
                            name: format!(r#"<abbr title="All hitboxes within hitbox set {} can hit the same enemy again on these frames.">Hitbox set {} hits</abbr>"#, set_id, set_id),
                            value: hitboxes_rehit.clone(),
                        });
                    }
                }

                attributes.push(Attribute {
                    name: r#"<abbr title="Internal subaction index. Useful for modding with PSA.">Subaction Index</abbr>"#.into(),
                    value: format!("0x{:x}", index)
                });

                // generate hitbox tables
                let mut hitbox_tables = vec!();
                let mut twitter_hitboxes = String::new();
                let mut last_change_frame = None;
                for i in 0..subaction.frames.len() {
                    let prev_frame = if i == 0 {
                        None
                    } else {
                        Some(&subaction.frames[i - 1])
                    };
                    let frame = &subaction.frames[i];

                    // get the values of the previous and next hitboxes
                    let prev_values = if let Some(prev_frame) = prev_frame {
                        prev_frame.hit_boxes.iter().map(|x| &x.next_values).collect()
                    } else {
                        vec!()
                    };
                    let next_values: Vec<_> = frame.hit_boxes.iter().map(|x| &x.next_values).collect();

                    // start a new table when ((the hitbox values or number of hitboxes change) and there are hitboxes) or it is the last frame
                    // TODO: This comparison ignores hitbox_id, is this acceptable?
                    if prev_values != next_values || i + 1 == subaction.frames.len() {
                        if let Some(first_frame) = last_change_frame {
                            let frames = if first_frame + 1 == i {
                                format!("Frame: {}", i)
                            } else {
                                format!("Frames: {}-{}", first_frame+1, i)
                            };

                            let mut hitboxes = vec!();
                            for colbox in prev_frame.map(|x| &x.hit_boxes).unwrap_or(&frame.hit_boxes) {
                                match &colbox.next_values {
                                    CollisionBoxValues::Grab (_) => { }
                                    CollisionBoxValues::Hit (hit) => hitboxes.push(hit),
                                }
                            }

                            // TODO: determine what optional columns to use
                            let use_wdsk = hitboxes.iter().any(|x| x.weight_knockback != 0);
                            let use_angle_flipping = hitboxes.iter().any(|x| match x.angle_flipping {
                                AngleFlip::AwayFromAttacker => false,
                                _ => true,
                            });
                            let use_clang = hitboxes.iter().any(|x| !x.clang);
                            let use_direct = hitboxes.iter().any(|x| !x.direct);
                            let use_hitlag_mult = hitboxes.iter().any(|x| x.hitlag_mult != 1.0);
                            let use_sdi_mult = hitboxes.iter().any(|x| x.sdi_mult != 1.0);
                            let use_shield_damage = hitboxes.iter().any(|x| x.shield_damage != 0);
                            let use_tripping_rate = hitboxes.iter().any(|x| x.tripping_rate != 0.0);
                            let use_rehit_rate = hitboxes.iter().any(|x| x.rehit_rate != 0);
                            let use_can_be_shielded = hitboxes.iter().any(|x| !x.can_be_shielded);
                            let use_can_be_reflected = hitboxes.iter().any(|x| x.can_be_reflected);
                            let use_can_be_absorbed = hitboxes.iter().any(|x| x.can_be_absorbed);
                            let use_remain_grabbed = hitboxes.iter().any(|x| x.remain_grabbed);
                            let use_ignore_invincibility = hitboxes.iter().any(|x| x.ignore_invincibility);
                            let use_freeze_frame_disable = hitboxes.iter().any(|x| x.freeze_frame_disable);
                            let use_flinchless = hitboxes.iter().any(|x| x.flinchless);
                            let twitter_use_effect = hitboxes.iter().any(|x| match x.effect {
                                HitBoxEffect::Normal      => false,
                                HitBoxEffect::None        => false,
                                HitBoxEffect::Slash       => false,
                                HitBoxEffect::Coin        => false,
                                HitBoxEffect::Unknown (_) => false,
                                HitBoxEffect::Electric    => true,
                                HitBoxEffect::Freezing    => true,
                                HitBoxEffect::Flame       => true,
                                HitBoxEffect::Reverse     => true,
                                HitBoxEffect::Trip        => true,
                                HitBoxEffect::Sleep       => true,
                                HitBoxEffect::Bury        => true,
                                HitBoxEffect::Stun        => true,
                                HitBoxEffect::Flower      => true,
                                HitBoxEffect::Grass       => true,
                                HitBoxEffect::Water       => true,
                                HitBoxEffect::Darkness    => true,
                                HitBoxEffect::Paralyze    => true,
                                HitBoxEffect::Aura        => true,
                                HitBoxEffect::Plunge      => true,
                                HitBoxEffect::Down        => true,
                                HitBoxEffect::Flinchless  => true,
                            });

                            last_change_frame = Some(i);
                            let mut header = vec!();
                            header.push(r#"<abbr title="Hitboxes in different sets can rehit the same enemy across multiple frames. i.e. a multi-hit move. Hitboxes in different sets can also hit in the same frame, the damage is the total of all hit hitboxes however the knockback and angle is taken from the hitbox with the most knockback.">Set</abbr>"#);
                            header.push(r#"<abbr title="Lower hitbox IDs take priority over higher ones. i.e. if hitbox 0 and 1 are both hit, hitbox 0 is used">ID</abbr>"#);
                            header.push(r#"<abbr title="Damage">Dmg</abbr>"#);
                            if use_wdsk {
                                header.push(r#"<abbr title="Weight Dependent Set Knockback. When this value is not 0, an entirely different formula is used to calculate knockback. In this formula hitbox damage and current % are ignored.">WDSK</abbr>"#);
                            }
                            header.push(r#"<abbr title="Base knockback">BKB</abbr>"#);
                            header.push(r#"<abbr title="Knockback growth">KBG</abbr>"#);
                            header.push("Angle");
                            header.push("Effect");
                            header.push("Sound");
                            if use_angle_flipping {
                                header.push("Angle Flip");
                            }
                            if use_clang {
                                header.push("Clang");
                            }
                            if use_direct {
                                header.push("Direct");
                            }
                            if use_hitlag_mult {
                                header.push("Hitlag Mult");
                            }
                            if use_sdi_mult {
                                header.push("SDI Mult");
                            }
                            if use_shield_damage {
                                header.push(r#"<abbr title="Shield Damage">Shield Dmg</abbr>"#);
                            }
                            if use_tripping_rate {
                                header.push(r#"<abbr title="Trip Rate. Chance of trip occuring out of 100?">Trip Rate</abbr>"#);
                            }
                            if use_rehit_rate {
                                header.push(r#"<abbr title="Rehit rate. How many frames it takes for the hitbox to hit the same target again. e.g. value of 1 means it will hit every frame.">Rehit Rate</abbr>"#);
                            }
                            if use_can_be_shielded {
                                header.push(r#"<abbr title="Can be shielded">Shieldable</abbr>"#);
                            }
                            if use_can_be_reflected {
                                header.push(r#"<abbr title="Can be reflected">Reflectable</abbr>"#);
                            }
                            if use_can_be_absorbed {
                                header.push(r#"<abbr title="Can be absorbed">Absorbable</abbr>"#);
                            }
                            if use_remain_grabbed {
                                header.push(r#"Remain Grabbed"#);
                            }
                            if use_ignore_invincibility {
                                header.push(r#"Ignore Invincibility"#);
                            }
                            if use_freeze_frame_disable {
                                header.push(r#"Freeze frame disable"#);
                            }
                            if use_flinchless {
                                header.push(r#"<abbr title="TODO">Flinchless"#);
                            }
                            header.push("Targets");

                            // store twitter string values here
                            let mut damage = String::new();
                            let mut wdsk = String::new();
                            let mut bkb = String::new();
                            let mut kbg = String::new();
                            let mut angle = String::new();
                            let mut angle_flipping = String::new();
                            let mut effect = String::new();
                            let mut clang = String::new();
                            let mut direct = String::new();
                            let mut hitlag_mult = String::new();
                            let mut sdi_mult = String::new();
                            let mut shield_damage = String::new();
                            let mut tripping_rate = String::new();
                            let mut rehit_rate = String::new();
                            let mut can_be_shielded = String::new();
                            let mut can_be_reflected = String::new();
                            let mut can_be_absorbed = String::new();
                            let mut remain_grabbed = String::new();
                            let mut ignore_invincibility = String::new();
                            let mut freeze_frame_disable = String::new();
                            let mut flinchless = String::new();
                            let mut has_grab = false;

                            let mut rows = vec!();
                            for hitbox in prev_frame.map(|x| &x.hit_boxes).unwrap_or(&frame.hit_boxes) {
                                let mut row = vec!();

                                // everything else
                                match &hitbox.next_values {
                                    CollisionBoxValues::Hit(hit) => {
                                        if !hit.enabled {
                                            continue;
                                        }

                                        // generate the hitbox table structs for the webpage
                                        row.push(hit.set_id.to_string());
                                        row.push(hitbox.hitbox_id.to_string());
                                        row.push(hit.damage.to_string());
                                        if use_wdsk {
                                            row.push(hit.weight_knockback.to_string());
                                        }
                                        row.push(hit.bkb.to_string());
                                        row.push(hit.kbg.to_string());
                                        let angle_name = match hit.trajectory {
                                            0 => String::from(r#"<abbr title="">0</abbr>"#),
                                            361 => String::from(r#"<abbr title="Sakurai Angle: When hit in the air angle is 45. When hit on the ground, if knockback < 32 then angle is 0, otherwise angle is 44.">361</abbr>"#),
                                            363 => String::from(r#"<abbr title="Autolink Angle: Angle is the angle the attacker is travelling on the frame collision occurred.">363</abbr>"#),
                                            365 => String::from(r#"<abbr title="Speed Dependent Autolink angle: Angle is the angle the attacker is travelling on the frame collision occurred. The knockback of the move is solely determined by the attackers velocity. Higher velocity results in more knockback.">365</abbr>"#),
                                            a   => a.to_string(),
                                        };
                                        row.push(format!(r#"<canvas class="hitbox-angle-render" width="0" height="0" hitbox-id="{}" angle="{}"></canvas>{}"#, hitbox.hitbox_id, hit.trajectory, angle_name));
                                        row.push(format!("{:?}", hit.effect));
                                        row.push(format!("{:?}", hit.sound));
                                        if use_angle_flipping {
                                            match hit.angle_flipping {
                                                AngleFlip::AwayFromAttacker => row.push(String::from(r#"<abbr title="Reverse Hittable: If the victim is behind the attacker the angle is flipped.">RH</abbr>"#)),
                                                AngleFlip::AttackerDir => row.push(String::from(r#"<abbr title="Forwards: The launch angle is flipped if the attacker is facing left">F</abbr>"#)),
                                                AngleFlip::AttackerDirReverse => row.push(String::from(r#"<abbr title="Backwards: The launch angle is flipped if the attacker is facing right">B</abbr>"#)),
                                                AngleFlip::FaceZaxis => row.push(String::from(r#"<abbr title="tooltiptext">Face Z Axis: A buggy unused angle flip, makes the victim face the screen and other weird stuff">FZA</abbr>"#)),
                                                AngleFlip::Unknown (_) => row.push(format!("{:?}", hit.angle_flipping)),
                                            }
                                        }
                                        if use_clang {
                                            row.push(hit.clang.to_string());
                                        }
                                        if use_direct {
                                            row.push(hit.direct.to_string());
                                        }
                                        if use_hitlag_mult {
                                            row.push(hit.hitlag_mult.to_string());
                                        }
                                        if use_sdi_mult {
                                            row.push(hit.sdi_mult.to_string());
                                        }
                                        if use_shield_damage {
                                            row.push(hit.shield_damage.to_string());
                                        }
                                        if use_tripping_rate {
                                            row.push(hit.tripping_rate.to_string());
                                        }
                                        if use_rehit_rate {
                                            row.push(hit.rehit_rate.to_string());
                                        }
                                        if use_can_be_shielded {
                                            row.push(hit.can_be_shielded.to_string());
                                        }
                                        if use_can_be_reflected {
                                            row.push(hit.can_be_reflected.to_string());
                                        }
                                        if use_can_be_absorbed {
                                            row.push(hit.can_be_absorbed.to_string());
                                        }
                                        if use_remain_grabbed {
                                            row.push(hit.remain_grabbed.to_string());
                                        }
                                        if use_ignore_invincibility {
                                            row.push(hit.ignore_invincibility.to_string());
                                        }
                                        if use_freeze_frame_disable {
                                            row.push(hit.freeze_frame_disable.to_string());
                                        }
                                        if use_flinchless {
                                            row.push(hit.flinchless.to_string());
                                        }

                                        let mut can_hit = String::new();

                                        let enable_fighter_ground = if hit.can_hit_fighter() && hit.ground { "" } else { "icon-disable" };
                                        let enable_fighter_air    = if hit.can_hit_fighter() && hit.aerial { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Fighter on the ground" class="spritesheet-fighter-ground {}" src="{}" />"#, enable_fighter_ground, assets.spritesheet_png));
                                        can_hit.push_str(&format!(r#"<img title="Fighter in the air" class="spritesheet-fighter-air {}" src="{}" />"#, enable_fighter_air, assets.spritesheet_png));

                                        let enable_waddle = if hit.can_hit_waddle_dee_doo() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Waddle Dee and Waddle Doo" class="spritesheet-waddle {}" src="{}" />"#, enable_waddle, assets.spritesheet_png));

                                        let enable_pikmin = if hit.can_hit_pikmin() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Pikmin" class="spritesheet-pikmin {}" src="{}" />"#, enable_pikmin, assets.spritesheet_png));

                                        let enable_gyro = if hit.can_hit_gyro() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="ROB Gyro" class="spritesheet-gyro {}" src="{}" />"#, enable_gyro, assets.spritesheet_png));

                                        let enable_grenade = if hit.can_hit_snake_grenade() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Snake's Grenade" class="spritesheet-snake-grenade {}" src="{}" />"#, enable_grenade, assets.spritesheet_png));

                                        let enable_mr_saturn = if hit.can_hit_mr_saturn() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Mr Saturn" class="spritesheet-mr-saturn {}" src="{}" />"#, enable_mr_saturn, assets.spritesheet_png));

                                        let enable_stage_non_wall_ceiling_floor = if hit.can_hit_stage_non_wall_ceiling_floor() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Stage hurtboxes not including walls, ceilings and floors." class="spritesheet-stage-non-wall-ceiling-floor {}" src="{}" />"#, enable_stage_non_wall_ceiling_floor, assets.spritesheet_png));

                                        let enable_wall_ceiling_floor = if hit.can_hit_wall_ceiling_floor() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Walls, Ceilings and Floors" class="spritesheet-wall-ceiling-floor {}" src="{}" />"#, enable_wall_ceiling_floor, assets.spritesheet_png));

                                        let enable_link_bomb = if hit.can_hit_link_bomb() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Toon Link and Link's Bombs" class="spritesheet-link-bomb {}" src="{}" />"#, enable_link_bomb, assets.spritesheet_png));

                                        let enable_bobomb = if hit.can_hit_bobomb() { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Bob-ombs" class="spritesheet-bobomb {}" src="{}" />"#, enable_bobomb, assets.spritesheet_png));

                                        row.push(can_hit);


                                        // generate the hitbox string for the twitter description
                                        if damage.len() > 0 {
                                            damage.push_str(",");
                                        }
                                        if wdsk.len() > 0 {
                                            wdsk.push_str(",");
                                        }
                                        if bkb.len() > 0 {
                                            bkb.push_str(",");
                                        }
                                        if kbg.len() > 0 {
                                            kbg.push_str(",");
                                        }
                                        if angle.len() > 0 {
                                            angle.push_str(",");
                                        }
                                        if angle_flipping.len() > 0 {
                                            angle_flipping.push_str(",");
                                        }
                                        if effect.len() > 0 {
                                            effect.push_str(",");
                                        }
                                        if clang.len() > 0 {
                                            clang.push_str(",");
                                        }
                                        if direct.len() > 0 {
                                            direct.push_str(",");
                                        }
                                        if hitlag_mult.len() > 0 {
                                            hitlag_mult.push_str(",");
                                        }
                                        if sdi_mult.len() > 0 {
                                            sdi_mult.push_str(",");
                                        }
                                        if shield_damage.len() > 0 {
                                            shield_damage.push_str(",");
                                        }
                                        if tripping_rate.len() > 0 {
                                            tripping_rate.push_str(",");
                                        }
                                        if rehit_rate.len() > 0 {
                                            rehit_rate.push_str(",");
                                        }
                                        if can_be_shielded.len() > 0 {
                                            can_be_shielded.push_str(",");
                                        }
                                        if can_be_reflected.len() > 0 {
                                            can_be_reflected.push_str(",");
                                        }
                                        if can_be_absorbed.len() > 0 {
                                            can_be_absorbed.push_str(",");
                                        }
                                        if remain_grabbed.len() > 0 {
                                            remain_grabbed.push_str(",");
                                        }
                                        if ignore_invincibility.len() > 0 {
                                            ignore_invincibility.push_str(",");
                                        }
                                        if freeze_frame_disable.len() > 0 {
                                            freeze_frame_disable.push_str(",");
                                        }
                                        if flinchless.len() > 0 {
                                            flinchless.push_str(",");
                                        }

                                        damage.push_str(&hit.damage.to_string());
                                        wdsk.push_str(&hit.weight_knockback.to_string());
                                        bkb.push_str(&hit.bkb.to_string());
                                        kbg.push_str(&hit.kbg.to_string());
                                        angle.push_str(&hit.trajectory.to_string());
                                        angle_flipping.push_str(&match hit.angle_flipping {
                                            AngleFlip::AwayFromAttacker => "RH".into(),
                                            AngleFlip::AttackerDir => "F".into(),
                                            AngleFlip::AttackerDirReverse => "B".into(),
                                            AngleFlip::FaceZaxis => "FZA".into(),
                                            AngleFlip::Unknown (_) => format!("{:?}", hit.angle_flipping),
                                        });
                                        effect.push_str(&format!("{:?}", hit.effect));
                                        clang.push_str(if hit.clang { "y" } else { "n" });
                                        direct.push_str(if hit.direct { "y" } else { "n" });
                                        hitlag_mult.push_str(&hit.hitlag_mult.to_string());
                                        sdi_mult.push_str(&hit.sdi_mult.to_string());
                                        shield_damage.push_str(&hit.shield_damage.to_string());
                                        tripping_rate.push_str(&hit.tripping_rate.to_string());
                                        rehit_rate.push_str(&hit.rehit_rate.to_string());
                                        can_be_shielded.push_str(if hit.can_be_shielded { "y" } else { "n" });
                                        can_be_reflected.push_str(if hit.can_be_reflected { "y" } else { "n" });
                                        can_be_absorbed.push_str(if hit.can_be_absorbed { "y" } else { "n" });
                                        remain_grabbed.push_str(if hit.remain_grabbed { "y" } else { "n" });
                                        ignore_invincibility.push_str(if hit.ignore_invincibility { "y" } else { "n" });
                                        freeze_frame_disable.push_str(if hit.freeze_frame_disable { "y" } else { "n" });
                                        flinchless.push_str(if hit.flinchless { "y" } else { "n" });
                                    }
                                    CollisionBoxValues::Grab(grab) => {
                                        row.push(String::from("0"));
                                        row.push(hitbox.hitbox_id.to_string());
                                        row.push(String::from("Grab"));
                                        row.push(format!("set action: 0x{:x}", grab.set_action));
                                        row.push(String::new());
                                        row.push(String::new());
                                        row.push(String::new());
                                        row.push(String::new());

                                        let mut can_hit = String::new();

                                        let enable_fighter_ground = if grab.target.grounded() { "" } else { "icon-disable" };
                                        let enable_fighter_air    = if grab.target.aerial()   { "" } else { "icon-disable" };
                                        can_hit.push_str(&format!(r#"<img title="Fighter on the ground" class="spritesheet-fighter-ground {}" src="{}" />"#, enable_fighter_air, assets.spritesheet_png));
                                        can_hit.push_str(&format!(r#"<img title="Fighter in the air" class="spritesheet-fighter-air {}" src="{}" />"#, enable_fighter_ground, assets.spritesheet_png));

                                        row.push(can_hit);
                                        has_grab = true;
                                    }
                                }
                                rows.push(row);
                            }

                            if rows.len() > 0 {
                                twitter_hitboxes.push_str(&format!("\n\n{}", frames));
                                twitter_hitboxes.push_str(&format!("\nDamage: {}", damage));
                                if use_wdsk {
                                    twitter_hitboxes.push_str(&format!("\nWDSK: {}", wdsk));
                                }
                                twitter_hitboxes.push_str(&format!("\nBKB: {}", bkb));
                                twitter_hitboxes.push_str(&format!("\nKBG: {}", kbg));
                                twitter_hitboxes.push_str(&format!("\nAngle: {}", angle));
                                if use_angle_flipping {
                                    twitter_hitboxes.push_str(&format!("\nAngle Flip: {}", angle_flipping));
                                }
                                if twitter_use_effect {
                                    twitter_hitboxes.push_str(&format!("\nEffect: {}", effect));
                                }
                                if use_clang {
                                    twitter_hitboxes.push_str(&format!("\nClang: {}", clang));
                                }
                                if use_direct {
                                    twitter_hitboxes.push_str(&format!("\nDirect: {}", direct));
                                }
                                if use_hitlag_mult {
                                    twitter_hitboxes.push_str(&format!("\nHitlag Mult: {}", hitlag_mult));
                                }
                                if use_sdi_mult {
                                    twitter_hitboxes.push_str(&format!("\nSDI Mult: {}", sdi_mult));
                                }
                                if use_shield_damage {
                                    twitter_hitboxes.push_str(&format!("\nShield Damage: {}", shield_damage));
                                }
                                if use_tripping_rate {
                                    twitter_hitboxes.push_str(&format!("\nTripping Rate: {}", tripping_rate));
                                }
                                if use_rehit_rate {
                                    twitter_hitboxes.push_str(&format!("\nRehit Rate: {}", rehit_rate));
                                }
                                if use_can_be_shielded {
                                    twitter_hitboxes.push_str(&format!("\nCan Be Shielded: {}", can_be_shielded));
                                }
                                if use_can_be_reflected {
                                    twitter_hitboxes.push_str(&format!("\nCan Be Reflected: {}", can_be_reflected));
                                }
                                if use_can_be_absorbed {
                                    twitter_hitboxes.push_str(&format!("\nCan Be Absorbed: {}", can_be_absorbed));
                                }
                                if use_remain_grabbed {
                                    twitter_hitboxes.push_str(&format!("\nRemain Grabbed: {}", remain_grabbed));
                                }
                                if use_ignore_invincibility {
                                    twitter_hitboxes.push_str(&format!("\nIgnore Invincibility: {}", ignore_invincibility));
                                }
                                if use_freeze_frame_disable {
                                    twitter_hitboxes.push_str(&format!("\nDisable Freeze Frame: {}", freeze_frame_disable));
                                }
                                if use_flinchless {
                                    twitter_hitboxes.push_str(&format!("\nFlinchless: {}", flinchless));
                                }
                                if has_grab {
                                    twitter_hitboxes.push_str("\nHas Grabbox");
                                }

                                hitbox_tables.push(HitBoxTable { frames, header, rows });
                            }
                        }
                    }

                    // set initial last_change_frame
                    if prev_values != next_values && frame.hit_boxes.len() > 0 && last_change_frame.is_none() {
                        last_change_frame = Some(i);
                    }

                    // no more hitboxes, clear last_change_frame
                    if prev_values != next_values && frame.hit_boxes.len() == 0 {
                        last_change_frame = None;
                    }
                }

                // Despite being a twitter description, this is designed for use on discord instead of twitter.
                // We make use of the 78 lines or 276 characters that discord displays.
                // Ignoring that twitter only displays the first 3 lines.
                //
                // We can't reuse the `attributes` vec as we have different values here e.g. frame count and no subaction index
                let mut twitter_description = String::new();
                twitter_description.push_str(&format!("Frames: {}", subaction.frames.len()));
                twitter_description.push_str(&format!("\nIASA: {}", subaction.iasa + 1));
                if auto_cancel.len() > 0 {
                    twitter_description.push_str(&format!("\nAuto Cancel: {}", auto_cancel));
                }
                if let Some(landing_lag) = subaction.landing_lag {
                    twitter_description.push_str(&format!("\nLanding Lag: {}", landing_lag));
                }
                if intangible.len() > 0 {
                    twitter_description.push_str(&format!("\nFully Intangible: {}", intangible));
                }
                if invincible.len() > 0 {
                    twitter_description.push_str(&format!("\nFully Invincible: {}", invincible));
                }
                if partial_intangible.len() > 0 {
                    twitter_description.push_str(&format!("\nPartially Intangible: {}", partial_intangible));
                }
                if partial_invincible.len() > 0 {
                    twitter_description.push_str(&format!("\nPartially Invincible: {}", partial_invincible));
                }
                if reverse_direction.len() > 0 {
                    twitter_description.push_str(&format!("\nDirection Reverse Frames: {}", reverse_direction));
                }

                twitter_description.push_str(&twitter_hitboxes);

                // generate fighter links
                let mut fighter_links = vec!();
                for nav_fighter in &brawl_mod.fighters {
                    let nav_fighter = &nav_fighter.fighter;
                    let subaction_name = if nav_fighter.subactions.iter().find(|x| x.name == subaction.name).is_some() {
                        subaction.name.clone()
                    } else {
                        String::from("Wait1")
                    };
                    fighter_links.push(NavLink {
                        name:    nav_fighter.name.clone(),
                        link:    format!("/{}/{}/subactions/{}.html", brawl_mod.name, nav_fighter.name, subaction_name),
                        current: fighter_name == &nav_fighter.name,
                    });
                }

                let mut subaction_extent = subaction.hurt_box_extent();
                subaction_extent.extend(&subaction.hit_box_extent());

                let twitter_image = format!("/{}/{}/subactions/{}.gif", brawl_mod.name, fighter_name, subaction.name);

                let page = SubactionPage {
                    assets,
                    fighter_link:     format!("/{}/{}", brawl_mod.name, fighter_name),
                    mod_links:        &mod_links,
                    title:            format!("{} - {} - Subaction - {}", brawl_mod.name, fighter_name, subaction.name),
                    subaction_links:  brawl_mod.gen_subaction_links(&fighter.fighter, subaction.name.clone()),
                    subaction:        serde_json::to_string(&subaction).unwrap(),
                    subaction_extent: serde_json::to_string(&subaction_extent).unwrap(),
                    attributes,
                    hitbox_tables,
                    fighter_links,
                    script_main,
                    script_gfx,
                    script_sfx,
                    script_other,
                    frame_buttons,
                    twitter_image: twitter_image.clone(),
                    twitter_description,
                };

                fs::create_dir_all(format!("../root/{}/{}/subactions", brawl_mod.name, fighter_name)).unwrap();
                {
                    let path = format!("../root/{}/{}/subactions/{}.html", brawl_mod.name, fighter_name, subaction.name);
                    let file = File::create(path).unwrap();
                    handlebars.render_to_write("subaction", &page, file).unwrap();
                }

                if subaction.name == "AttackAirF" && false {
                    let gif = renderer::render_gif(&fighter.fighter, index);
                    let path = format!("../root{}", twitter_image);
                    let mut file = File::create(path).unwrap();
                    file.write_all(&gif).unwrap();
                }
                info!("{} {} {}", brawl_mod.name, fighter_name, subaction.name);
            });
        }
    }
}

#[derive(Serialize)]
pub struct SubactionPage<'a> {
    assets:              &'a AssetPaths,
    mod_links:           &'a [NavLink],
    fighter_links:       Vec<NavLink>,
    subaction_links:     SubactionLinks,
    fighter_link:        String,
    title:               String,
    attributes:          Vec<Attribute>,
    hitbox_tables:       Vec<HitBoxTable>,
    subaction:           String,
    subaction_extent:    String,
    script_main:         String,
    script_gfx:          String,
    script_sfx:          String,
    script_other:        String,
    frame_buttons:       Vec<FrameButton>,
    twitter_description: String,
    twitter_image:       String,
}

#[derive(Serialize)]
pub struct FrameButton {
    index: usize,
    class: String,
}

#[derive(Serialize)]
struct Attribute {
    name:  String,
    value: String,
}

#[derive(Serialize)]
struct HitBoxTable {
    frames: String,
    header: Vec<&'static str>,
    rows:   Vec<Vec<String>>,
}
