use byteorder::{BigEndian, ReadBytesExt};

use util;

pub(crate) fn bres(data: &[u8]) -> Bres {
    let root_offset = (&data[0xc..0xe]).read_u16::<BigEndian>().unwrap();
    bres_group(&data[root_offset as usize ..])
}

// TODO: Pull ResourceGroup + ResourceEntries into their own struct, then store that struct in Bres and Chr0
//Resource {
//    string_offset
//    string
//    data_offset
//}

fn bres_group(data: &[u8]) -> Bres {
    let total_size = (&data[0x8 ..]).read_i32::<BigEndian>().unwrap();
    let num_children = (&data[0xc ..]).read_i32::<BigEndian>().unwrap();

    let mut children = vec!();
    for i in 1..num_children+1 { // the first child is a dummy so we skip it.
        let child_index = 0x10 + BRES_CHILD_SIZE * i as usize;

        let string_offset = (&data[child_index as usize + 8 .. ]).read_i32::<BigEndian>().unwrap();
        let string_data = &data[ROOT_SIZE + string_offset as usize .. ];
        let name = String::from(util::parse_str(string_data).unwrap());

        let data_offset = (&data[child_index as usize + 0xc .. ]).read_i32::<BigEndian>().unwrap();
        let child_data = &data[ROOT_SIZE + data_offset as usize .. ];

        let tag = util::parse_tag(child_data);
        let child_data = match tag.as_ref() {
            "CHR0" => BresChildData::Chr0 (chr0(child_data)),
            "" => BresChildData::Bres (Box::new(bres_group(&data[data_offset as usize ..]))),
            _  => BresChildData::Unknown (tag),
        };

        children.push(BresChild {
            id:          (&data[child_index as usize       .. ]).read_u16::<BigEndian>().unwrap(),
            flag:        (&data[child_index as usize + 0x2 .. ]).read_u16::<BigEndian>().unwrap(),
            left_index:  (&data[child_index as usize + 0x4 .. ]).read_u16::<BigEndian>().unwrap(),
            right_index: (&data[child_index as usize + 0x6 .. ]).read_u16::<BigEndian>().unwrap(),
            string_offset,
            data_offset,
            data: child_data,
            name,
        });
    }

    Bres {
        total_size,
        children
    }
}

fn chr0(data: &[u8]) -> Chr0 {
    let size             = (&data[0x4..]).read_i32::<BigEndian>().unwrap();
    let version          = (&data[0x8..]).read_i32::<BigEndian>().unwrap();
    let bres_offset      = (&data[0xc..]).read_i32::<BigEndian>().unwrap();
    let group_offset     = (&data[0x10..]).read_i32::<BigEndian>().unwrap();
    let string_offset    = (&data[0x14..]).read_i32::<BigEndian>().unwrap();
    let orig_path_offset = (&data[0x18..]).read_i32::<BigEndian>().unwrap();
    let num_frames       = (&data[0x1c..]).read_u16::<BigEndian>().unwrap();
    let num_children     = (&data[0x1e..]).read_u16::<BigEndian>().unwrap();
    let loop_value       = (&data[0x20..]).read_i32::<BigEndian>().unwrap();
    let scaling_rule     = (&data[0x24..]).read_i32::<BigEndian>().unwrap();
    assert_eq!(version, 4);

    let name = String::from(util::parse_str(&data[string_offset as usize ..]).unwrap());
    let _group_total_size = (&data[group_offset as usize ..]).read_i32::<BigEndian>().unwrap();
    let group_num_children = (&data[group_offset as usize + 4 ..]).read_i32::<BigEndian>().unwrap();

    let mut children = vec!();
    for i in 1..group_num_children+1 { // the first child is a dummy so we skip it.
        let child_index = group_offset as usize + BRES_GROUP_SIZE + BRES_CHILD_SIZE * i as usize;

        let string_offset = (&data[child_index + 8 .. ]).read_i32::<BigEndian>().unwrap();
        let string_data = &data[group_offset as usize + string_offset as usize .. ]; // -0x10 correctly aligns snakes first group
        let name = String::from(util::parse_str(string_data).unwrap());

        let data_offset = (&data[child_index + 0xc .. ]).read_i32::<BigEndian>().unwrap();
        let child_data = &data[group_offset as usize + data_offset as usize .. ];

        children.push(Chr0Child {
            string_offset,
            data_offset,
            name
        });
    }

    Chr0 {
        name,
        size,
        version,
        bres_offset,
        group_offset,
        string_offset,
        orig_path_offset,
        num_frames,
        num_children,
        loop_value: loop_value != 0,
        scaling_rule,
        children,
    }
}

// Brawlbox has this split into three structs: BRESHeader, BRESEntry and ROOTHeader
// BRESEntry is commented out, so that appears wrong
// BRESHeader and RootHeader are combined because without BRESEntry they appear to be sequential
#[derive(Debug)]
pub struct Bres {
    total_size: i32,
    pub children: Vec<BresChild>
}

const ROOT_SIZE: usize = 0x8;
const BRES_GROUP_SIZE: usize = 0x8;

const BRES_CHILD_SIZE: usize = 0x10;
#[derive(Debug)]
pub struct BresChild {
    id: u16,
    flag: u16,
    left_index: u16,
    right_index: u16,
    string_offset: i32,
    data_offset: i32,
    pub name: String,
    pub data: BresChildData
}

#[derive(Debug)]
pub enum BresChildData {
    Chr0 (Chr0),
    Bres (Box<Bres>),
    Unknown (String)
}

#[derive(Debug)]
pub struct Chr0 {
    pub name: String,
    size: i32,
    version: i32,
    bres_offset: i32,
    group_offset: i32,
    string_offset: i32,
    orig_path_offset: i32,
    pub num_frames: u16,
    num_children: u16,
    loop_value: bool,
    scaling_rule: i32,
    children: Vec<Chr0Child>
}

#[derive(Debug)]
pub struct Chr0Child {
    string_offset: i32,
    data_offset: i32,
    pub name: String
}
