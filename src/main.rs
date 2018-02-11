use std::fs::File;
use std::fs;
use std::io::Read;
use std::str;

extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};

fn main() {
    match fs::read_dir("data/brawl/fighter") {
        Ok(fighter_dir) => {
            for fighter_path in fighter_dir {
                let fighter_path = fighter_path.unwrap();
                if fighter_path.file_type().unwrap().is_dir() {
                    let fighter_path = fighter_path.path();

                    let folder_name = fighter_path.file_name().unwrap().to_str().unwrap().to_string();
                    let mut cased_fighter_name: Option<String> = None; // TODO: Use this to determine the names of the other *.pac *.pcs files
                    let mut moveset_name: Option<String> = None;

                    for data_path in fs::read_dir(&fighter_path).unwrap() {
                        let data_path = data_path.unwrap().path();
                        let data_name = data_path.file_name().unwrap().to_str().unwrap().to_string();
                        if data_name.to_lowercase() == format!("Fit{}.pac", folder_name).to_lowercase() {
                            cased_fighter_name = Some(String::from(data_name.trim_right_matches(".pac").trim_left_matches("Fit")));
                            moveset_name = Some(data_name);
                        }
                    }

                    println!("\n{:?}", folder_name);
                    println!("{:?}", cased_fighter_name);

                    // read
                    if let Some(moveset_name) = moveset_name {
                        let mut moveset_data: Vec<u8> = vec!();
                        File::open(fighter_path.join(moveset_name)).unwrap().read_to_end(&mut moveset_data).unwrap();
                        parse_arc(&moveset_data);
                    }
                }
            }
        }
        Err(_) => {
            println!("'data' directory incorrectly setup.");
        }
    }
}

fn parse_arc(data: &[u8]) {
    //read the main header
    let num_sub_headers = (&data[6..8]).read_u16::<BigEndian>().unwrap();

    // read the sub headers
    let mut header_index = ARC_MAIN_HEADER_SIZE;
    for i in 0..num_sub_headers {
        let sub_header = ArcSubHeader::new(&data[header_index ..]);
        println!("sub_header: {:?}", sub_header);

        if i == 0 {
            parse_arc_sakurai(&data[header_index + ARC_SUB_HEADER_SIZE ..]);
        }

        header_index += ARC_SUB_HEADER_SIZE + sub_header.size as usize;

        // align to the next ARC_SUB_HEADER_SIZE
        let offset = header_index % ARC_SUB_HEADER_SIZE;
        if offset != 0 {
            header_index += ARC_SUB_HEADER_SIZE - offset;
        }
    }
}

fn parse_arc_sakurai(data: &[u8]) {
    let header = ArcSakuraiHeader::new(data);
    println!("sakurai_header: {:?}", header);

    let lookup_entries_index = ARC_SAKURAI_HEADER_SIZE + header.lookup_offset as usize;
    let sections_index = lookup_entries_index + header.lookup_entry_count as usize * 4;
    let external_subroutines_index = sections_index + header.section_count as usize * 8;
    let string_table_index = external_subroutines_index + header.external_subroutine_count as usize * 8;

    for i in 0..header.section_count {
        let section_header = ArcSakuraiSectionHeader::new(&data[sections_index + i as usize * ARC_SAKURAI_SECTION_HEADER_SIZE ..]);

        let section_name_index = string_table_index + section_header.string_offset as usize;
        let section_name_length = &data[section_name_index..].iter().position(|x| *x == 0).unwrap();
        let section_name = str::from_utf8(&data[section_name_index .. section_name_index + section_name_length]).unwrap();

        println!("section: {:?}", section_name);
        if section_name == "data" {
            let fighter_data_header = ArcFighterDataHeader::new(&data[ARC_SAKURAI_HEADER_SIZE + section_header.data_offset as usize ..]);
            println!("fighter_data_header: {:?}", fighter_data_header);

            let attributes = &FighterAttributes::new(&data[ARC_SAKURAI_HEADER_SIZE + fighter_data_header.attribute_start as usize ..]);
            println!("attributes: {:?}", attributes);
        }
    }
}

// TODO:
// I might want to store the below structs for later retrieval instead of immediately processing
// To do that:
// *   add a vec to each struct to store their children
// *   delete the new() methods for each struct, put that logic in the parse functions

const ARC_MAIN_HEADER_SIZE: usize = 0x40;

const ARC_SUB_HEADER_SIZE: usize = 0x20;
#[derive(Debug)]
struct ArcSubHeader {
    ty: i16,
    index: i16,
    size: i32,
    group_index: u8,
    redirect_index: i16 // The index of a different file to read
}

impl ArcSubHeader {
    pub fn new(data: &[u8]) -> ArcSubHeader {
        ArcSubHeader {
            ty:             (&data[0..2]).read_i16::<BigEndian>().unwrap(),
            index:          (&data[2..4]).read_i16::<BigEndian>().unwrap(),
            size:           (&data[4..8]).read_i32::<BigEndian>().unwrap(),
            group_index:    data[8],
            redirect_index: (&data[9..11]).read_i16::<BigEndian>().unwrap(),
        }
    }
}

const ARC_SAKURAI_HEADER_SIZE: usize = 0x20;
#[derive(Debug)]
struct ArcSakuraiHeader {
    size: i32,
    lookup_offset: i32,
    lookup_entry_count: i32,
    section_count: i32,
    external_subroutine_count: i32,
}

impl ArcSakuraiHeader {
    pub fn new(data: &[u8]) -> ArcSakuraiHeader {
        ArcSakuraiHeader {
            size:                      (&data[0..4]).read_i32::<BigEndian>().unwrap(),
            lookup_offset:             (&data[4..8]).read_i32::<BigEndian>().unwrap(),
            lookup_entry_count:        (&data[8..12]).read_i32::<BigEndian>().unwrap(),
            section_count:             (&data[12..16]).read_i32::<BigEndian>().unwrap(),
            external_subroutine_count: (&data[16..20]).read_i32::<BigEndian>().unwrap(),
        }
    }
}

const ARC_SAKURAI_SECTION_HEADER_SIZE: usize = 0x8;
#[derive(Debug)]
struct ArcSakuraiSectionHeader {
    data_offset: i32,
    string_offset: i32,
}

impl ArcSakuraiSectionHeader {
    pub fn new(data: &[u8]) -> ArcSakuraiSectionHeader {
        ArcSakuraiSectionHeader {
            data_offset:   (&data[0..4]).read_i32::<BigEndian>().unwrap(),
            string_offset: (&data[4..8]).read_i32::<BigEndian>().unwrap(),
        }
    }
}

const _ARC_FIGHTER_DATA_HEADER_SIZE: usize = 0x7c;
#[derive(Debug)]
struct ArcFighterDataHeader {
    subaction_flags_start: i32,
    model_visibility_start: i32,
    attribute_start: i32,
    sse_attribute_start: i32,
    misc_section_offset: i32,
    common_action_flags_start: i32,
    action_flags_start: i32,
    action_interrupts: i32,
    entry_actions_start: i32,
    exit_actions_start: i32,
    action_pre_start: i32,
    subaction_main_start: i32,
    subaction_gfx_start: i32,
    subaction_sfx_start: i32,
    subaction_other_start: i32,
    anchored_item_positions: i32,
    gooey_bomb_positions: i32,
    bone_ref1: i32,
    bone_ref2: i32,
    entry_action_overrides: i32,
    exit_action_overrides: i32,
    samus_arm_cannon_positions: i32,
    static_articles_start: i32,
    entry_articles_start: i32,
    flags1: u32,
    flags2: i32,
}

impl ArcFighterDataHeader {
    pub fn new(data: &[u8]) -> ArcFighterDataHeader {
        ArcFighterDataHeader {
            subaction_flags_start:      (&data[0..4]).read_i32::<BigEndian>().unwrap(),
            model_visibility_start:     (&data[4..8]).read_i32::<BigEndian>().unwrap(),
            attribute_start:            (&data[8..12]).read_i32::<BigEndian>().unwrap(),
            sse_attribute_start:        (&data[12..16]).read_i32::<BigEndian>().unwrap(),
            misc_section_offset:        (&data[16..20]).read_i32::<BigEndian>().unwrap(),
            common_action_flags_start:  (&data[20..24]).read_i32::<BigEndian>().unwrap(),
            action_flags_start:         (&data[24..28]).read_i32::<BigEndian>().unwrap(),
            action_interrupts:          (&data[32..36]).read_i32::<BigEndian>().unwrap(),
            entry_actions_start:        (&data[36..40]).read_i32::<BigEndian>().unwrap(),
            exit_actions_start:         (&data[40..44]).read_i32::<BigEndian>().unwrap(),
            action_pre_start:           (&data[44..48]).read_i32::<BigEndian>().unwrap(),
            subaction_main_start:       (&data[48..52]).read_i32::<BigEndian>().unwrap(),
            subaction_gfx_start:        (&data[52..56]).read_i32::<BigEndian>().unwrap(),
            subaction_sfx_start:        (&data[56..60]).read_i32::<BigEndian>().unwrap(),
            subaction_other_start:      (&data[60..64]).read_i32::<BigEndian>().unwrap(),
            anchored_item_positions:    (&data[64..68]).read_i32::<BigEndian>().unwrap(),
            gooey_bomb_positions:       (&data[58..72]).read_i32::<BigEndian>().unwrap(),
            bone_ref1:                  (&data[72..76]).read_i32::<BigEndian>().unwrap(),
            bone_ref2:                  (&data[76..80]).read_i32::<BigEndian>().unwrap(),
            entry_action_overrides:     (&data[80..84]).read_i32::<BigEndian>().unwrap(),
            exit_action_overrides:      (&data[84..88]).read_i32::<BigEndian>().unwrap(),
            samus_arm_cannon_positions: (&data[92..96]).read_i32::<BigEndian>().unwrap(),
            static_articles_start:      (&data[100..104]).read_i32::<BigEndian>().unwrap(),
            entry_articles_start:       (&data[104..108]).read_i32::<BigEndian>().unwrap(),
            flags1:                     (&data[116..120]).read_u32::<BigEndian>().unwrap(),
            flags2:                     (&data[120..124]).read_i32::<BigEndian>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct FighterAttributes {
    walk_init_vel: f32,
    walk_acc: f32,
    walk_max_vel: f32,
    ground_friction: f32,
    dash_init_vel: f32,
    dash_run_accel_a: f32,
    dash_run_accel_b: f32,
    dash_run_term_vel: f32,
    grounded_max_x_vel: f32,
    dash_cancel_frame_window: i32, // spreadsheet is unsure
    guard_on_max_momentum: f32,
    jump_squat_frames: i32,
    jump_x_init_vel: f32,
    jump_y_init_vel: f32,
}

impl FighterAttributes {
    pub fn new(data: &[u8]) -> FighterAttributes {
        FighterAttributes {
            walk_init_vel:            (&data[0x00..0x04]).read_f32::<BigEndian>().unwrap(),
            walk_acc:                 (&data[0x04..0x08]).read_f32::<BigEndian>().unwrap(),
            walk_max_vel:             (&data[0x08..0x0c]).read_f32::<BigEndian>().unwrap(),
            ground_friction:          (&data[0x0c..0x10]).read_f32::<BigEndian>().unwrap(),
            dash_init_vel:            (&data[0x10..0x14]).read_f32::<BigEndian>().unwrap(),
            dash_run_accel_a:         (&data[0x14..0x18]).read_f32::<BigEndian>().unwrap(),
            dash_run_accel_b:         (&data[0x18..0x1c]).read_f32::<BigEndian>().unwrap(),
            dash_run_term_vel:        (&data[0x1c..0x20]).read_f32::<BigEndian>().unwrap(),
            grounded_max_x_vel:       (&data[0x24..0x28]).read_f32::<BigEndian>().unwrap(),
            dash_cancel_frame_window: (&data[0x28..0x2c]).read_i32::<BigEndian>().unwrap(),
            guard_on_max_momentum:    (&data[0x2c..0x30]).read_f32::<BigEndian>().unwrap(),
            jump_squat_frames:        (&data[0x30..0x34]).read_i32::<BigEndian>().unwrap(),
            jump_x_init_vel:          (&data[0x34..0x38]).read_f32::<BigEndian>().unwrap(),
            jump_y_init_vel:          (&data[0x38..0x3c]).read_f32::<BigEndian>().unwrap(),
        }
    }
}
