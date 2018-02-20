use byteorder::{BigEndian, ReadBytesExt};

use util;
use resources;

pub(crate) fn bres(data: &[u8]) -> Bres {
    let root_offset = (&data[0xc..0xe]).read_u16::<BigEndian>().unwrap();
    bres_group(&data[root_offset as usize ..])
}

fn bres_group(data: &[u8]) -> Bres {
    let mut children = vec!();
    for resource in resources::resources(&data[ROOT_SIZE..]) {
        let child_data = &data[ROOT_SIZE + resource.data_offset as usize .. ];

        let tag = util::parse_tag(child_data);
        let child_data = match tag.as_ref() {
            "CHR0" => BresChildData::Chr0 (chr0(child_data)),
            "" => BresChildData::Bres (Box::new(bres_group(&data[resource.data_offset as usize ..]))),
            _  => BresChildData::Unknown (tag),
        };

        children.push(BresChild {
            string_offset: resource.string_offset,
            data_offset:   resource.data_offset,
            name:          resource.string,
            data:          child_data,
        });
    }

    Bres {
        children
    }
}

fn chr0(data: &[u8]) -> Chr0 {
    let size             = (&data[0x4..]).read_i32::<BigEndian>().unwrap();
    let version          = (&data[0x8..]).read_i32::<BigEndian>().unwrap();
    let bres_offset      = (&data[0xc..]).read_i32::<BigEndian>().unwrap();
    let resources_offset = (&data[0x10..]).read_i32::<BigEndian>().unwrap();
    let string_offset    = (&data[0x14..]).read_i32::<BigEndian>().unwrap();
    let orig_path_offset = (&data[0x18..]).read_i32::<BigEndian>().unwrap();
    let num_frames       = (&data[0x1c..]).read_u16::<BigEndian>().unwrap();
    let num_children     = (&data[0x1e..]).read_u16::<BigEndian>().unwrap();
    let loop_value       = (&data[0x20..]).read_i32::<BigEndian>().unwrap();
    let scaling_rule     = (&data[0x24..]).read_i32::<BigEndian>().unwrap();
    assert_eq!(version, 4);

    let name = String::from(util::parse_str(&data[string_offset as usize ..]).unwrap());

    let mut children = vec!();
    for resource in resources::resources(&data[resources_offset as usize ..]) {
        let child_data = &data[resources_offset as usize + resource.data_offset as usize .. ];
        children.push(Chr0Child {
            string_offset: resource.string_offset,
            data_offset:   resource.data_offset,
            name:          resource.string
        });
    }

    Chr0 {
        name,
        size,
        version,
        bres_offset,
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
    pub children: Vec<BresChild>
}

const ROOT_SIZE: usize = 0x8;
#[derive(Debug)]
pub struct BresChild {
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
