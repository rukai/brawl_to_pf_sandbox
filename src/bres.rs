use byteorder::{BigEndian, ReadBytesExt};

use util;

pub(crate) fn bres(data: &[u8]) -> Bres {
    let root_offset = (&data[0xc..0xe]).read_u16::<BigEndian>().unwrap();
    let num_children = (&data[root_offset as usize + 0xc ..]).read_i32::<BigEndian>().unwrap();

    let mut children = vec!();
    let child_index = root_offset as usize + 0x10;
    for i in 0..num_children {
        let child_index = child_index + BRES_CHILD_SIZE * i as usize;

        let string_offset = root_offset as usize + BRES_GROUP_SIZE + (&data[child_index as usize + 8 .. ]).read_i32::<BigEndian>().unwrap() as usize;
        let name = String::from(util::parse_str(&data[string_offset..]).unwrap());

        let child_data = match name.as_ref() {
            "Textures(NW4R)"  => BresChildData::Textures,
            "Palettes(NW4R)"  => BresChildData::Palettes,
            "3DModels(NW4R)"  => BresChildData::X3DModels,
            "AnmChr(NW4R)"    => BresChildData::AnmChr,
            "AnmClr(NW4R)"    => BresChildData::AnmClr,
            "AnmTexSrt(NW4R)" => BresChildData::AnmTexSrt,
            "AnmTexPat(NW4R)" => BresChildData::AnmTexPat,
            "AnmShp(NW4R)"    => BresChildData::AnmShp,
            "AnmVis(NW4R)"    => BresChildData::AnmVis,
            "AnmScn(NW4R)"    => BresChildData::AnmScn,
            "AnmPat(NW4R)"    => BresChildData::AnmPat,
            _                 => BresChildData::Unknown
        };

        children.push(BresChild {
            id:            (&data[child_index as usize       .. ]).read_u16::<BigEndian>().unwrap(),
            flag:          (&data[child_index as usize + 0x2 .. ]).read_u16::<BigEndian>().unwrap(),
            data_offset:   (&data[child_index as usize + 0xc .. ]).read_i32::<BigEndian>().unwrap(),
            data:          child_data,
            name,
        });
    }

    Bres { children }
}

// Brawlbox has this split into three structs: BRESHeader, BRESEntry and ROOTHeader
// BRESEntry is commented out, so that appears wrong
// BRESHeader and RootHeader are combined because without BRESEntry they appear to be sequential
#[derive(Debug)]
pub struct Bres {
    pub children: Vec<BresChild>
}

const BRES_GROUP_SIZE: usize = 0x8;

const BRES_CHILD_SIZE: usize = 0x10;
#[derive(Debug)]
pub struct BresChild {
    id: u16,
    flag: u16,
    data_offset: i32,
    name: String,
    data: BresChildData
}

#[derive(Debug)]
enum BresChildData {
    Textures,
    Palettes,
    X3DModels,
    AnmChr,
    AnmClr,
    AnmTexSrt,
    AnmTexPat,
    AnmShp,
    AnmVis,
    AnmScn,
    AnmPat,
    Unknown
}

#[derive(Debug)]
pub struct AnmChr {
    num_frames:   u16,
    num_entries:  u16,
    loop_value:   i32,
    scaling_rule: i32,
}
