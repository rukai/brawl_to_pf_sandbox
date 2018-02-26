use byteorder::{BigEndian, ReadBytesExt};
use cgmath::Vector3;

use resources::*;
use resources;

pub(crate) fn mdl0(data: &[u8]) -> Mdl0 {
    let _size        = (&data[0x4..]).read_i32::<BigEndian>().unwrap();
    let version      = (&data[0x8..]).read_i32::<BigEndian>().unwrap();
    let _bres_offset = (&data[0xc..]).read_i32::<BigEndian>().unwrap();

    //let string_offset = match version {
    //    0xA => 0x44,
    //    0xB => 0x48,
    //    _   => 0x3C
    //};

    //let data_offset = match version {
    //    0xA => 0x40,
    //    0xB => 0x44,
    //    _   => 0 // no data
    //};

    //let name = String::from(util::parse_str(&data[string_offset .. ]).unwrap());
    //println!("{} {}", version, util::hex_dump(&data[string_offset - 40 .. string_offset + 40]));

    let props_offset = match version {
        0x08 => 0x40,
        0x09 => 0x40,
        0x0A => 0x48,
        0x0B => 0x4C,
        _    => 0 // no data
    };

    let props = if props_offset == 0 {
        None
    } else {
        Some(Mdl0Props {
            header_len:         (&data[props_offset + 0x00 ..]).read_u32::<BigEndian>().unwrap(),
            mdl0offset:         (&data[props_offset + 0x04 ..]).read_i32::<BigEndian>().unwrap(),
            scaling_rule:       (&data[props_offset + 0x08 ..]).read_i32::<BigEndian>().unwrap(),
            tex_matrix_mode:    (&data[props_offset + 0x0c ..]).read_i32::<BigEndian>().unwrap(),
            num_vertices:       (&data[props_offset + 0x10 ..]).read_i32::<BigEndian>().unwrap(),
            num_triangles:      (&data[props_offset + 0x14 ..]).read_i32::<BigEndian>().unwrap(),
            orig_path_offset:   (&data[props_offset + 0x18 ..]).read_i32::<BigEndian>().unwrap(),
            num_nodes:          (&data[props_offset + 0x1c ..]).read_i32::<BigEndian>().unwrap(),
            need_nrm_mtx_array: (&data[props_offset + 0x20 ..]).read_u8().unwrap(),
            need_tex_mtx_array: (&data[props_offset + 0x21 ..]).read_u8().unwrap(),
            enable_extents:     (&data[props_offset + 0x22 ..]).read_u8().unwrap(),
            env_mtx_mode:       (&data[props_offset + 0x23 ..]).read_u8().unwrap(),
            data_offset:        (&data[props_offset + 0x24 ..]).read_i32::<BigEndian>().unwrap(),
            extents:   read_mbox(&data[props_offset + 0x28 ..]),
        })
    };

    let mut definitions = None;
    let mut bones = None;
    let mut vertices = None;
    let mut normals = None;
    let mut colors = None;
    let mut uv = None;
    let mut fur_vectors = None;
    let mut fur_layer_coords = None;
    let mut materials = None;
    let mut shaders = None;
    let mut objects = None;
    let mut textures = None;
    let mut palettes = None;

    let fur_version = version >= 10;
    let num_children = if fur_version { 13 } else { 11 };
    for i in 0..num_children {
        let offset = 0x10 + i * 0x4;

        let resources_offset = (&data[offset..]).read_i32::<BigEndian>().unwrap();
        if resources_offset != 0 {
            let resources = resources::resources(&data[resources_offset as usize .. ]);
            match i {
                6  if fur_version => { fur_vectors = Some(resources) }
                7  if fur_version => { fur_layer_coords = Some(resources) }
                8  if fur_version => { materials = Some(resources) }
                9  if fur_version => { shaders = Some(resources) }
                10 if fur_version => { objects = Some(resources) }
                11 if fur_version => { textures = Some(resources) }
                12 if fur_version => { palettes = Some(resources) }
                0 => { definitions = Some(Mdl0Definitions { resources }) }
                1 => {
                    bones = Some(Mdl0Bones {
                        resources
                    })
                }
                2 => { vertices = Some(resources) }
                3 => { normals = Some(resources) }
                4 => { colors = Some(resources) }
                5 => { uv = Some(resources) }
                6 => { materials = Some(resources) }
                7 => { shaders = Some(resources) }
                8 => { objects = Some(resources) }
                9 => { textures = Some(resources) }
                10 => { palettes = Some(resources) }
                _ => { unreachable!() }
            }
        }
    }

    Mdl0 {
        version,
        props,
        definitions,
        bones,
        vertices,
        normals,
        colors,
        uv,
        fur_vectors,
        fur_layer_coords,
        materials,
        shaders,
        objects,
        textures,
        palettes,
    }
}

fn read_mbox(data: &[u8]) -> MBox {
    MBox {
        min: Vector3::<f32>::new(
            (&data[0x00..]).read_f32::<BigEndian>().unwrap(),
            (&data[0x04..]).read_f32::<BigEndian>().unwrap(),
            (&data[0x08..]).read_f32::<BigEndian>().unwrap(),
        ),
        max: Vector3::<f32>::new(
            (&data[0x0c..]).read_f32::<BigEndian>().unwrap(),
            (&data[0x10..]).read_f32::<BigEndian>().unwrap(),
            (&data[0x14..]).read_f32::<BigEndian>().unwrap(),
        )
    }
}

#[derive(Debug)]
pub struct Mdl0 {
    version: i32,
    pub props: Option<Mdl0Props>,
    definitions: Option<Mdl0Definitions>,
    bones: Option<Mdl0Bones>,
    vertices: Option<Vec<Resource>>,
    normals: Option<Vec<Resource>>,
    colors: Option<Vec<Resource>>,
    uv: Option<Vec<Resource>>,
    fur_vectors: Option<Vec<Resource>>,
    fur_layer_coords: Option<Vec<Resource>>,
    materials: Option<Vec<Resource>>,
    shaders: Option<Vec<Resource>>,
    objects: Option<Vec<Resource>>,
    textures: Option<Vec<Resource>>,
    palettes: Option<Vec<Resource>>,
}

#[derive(Debug)]
pub struct Mdl0Bones {
    resources: Vec<Resource>,
}

#[derive(Debug)]
pub struct Mdl0Definitions {
    resources: Vec<Resource>,
}

#[derive(Debug)]
pub struct Mdl0Props {
    header_len: u32,
    mdl0offset: i32,
    scaling_rule: i32,
    tex_matrix_mode: i32,
    num_vertices: i32,
    num_triangles: i32,
    orig_path_offset: i32,
    num_nodes: i32,
    need_nrm_mtx_array: u8,
    need_tex_mtx_array: u8,
    enable_extents: u8,
    env_mtx_mode: u8,
    data_offset: i32,
    extents: MBox,
}

// TODO: move into seperate file
// named mbox because box is used in std lib
#[derive(Debug)]
pub struct MBox {
    min: Vector3<f32>,
    max: Vector3<f32>,
}
