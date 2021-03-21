//
// Copyright 2021 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::collections::HashMap;

pub struct FileInfo(HashMap<String,String>);

/// Model Id
pub struct ModelId(u32);

/// Voxel Model
pub struct Model {
    id: ModelId,
    name: String,
    palette: PaletteId,
    voxels: Vec<PaletteColorId>,
}

/// Voxel Model Size
pub struct ModelSize {
    /// X Dimension
    width: u8,
    /// Y Dimension
    depth: u8,
    /// Z Dimension (-Z is gravity direction)
    height: u8,
}

pub struct DenseVoxels {
    size: ModelSize,
    voxels: Vec<PaletteColorId>
}

pub struct SparseVoxels {
    size: ModelSize,
    voxels: HashMap<(u8,u8,u8), PaletteColorId>,
}


/// Palette Id
pub struct PaletteId(u8);

/// Palette
pub struct Palette {
    id: PaletteId,
    name: String,
    colors: [PaletteColor; 256],
}

/// Color Id
pub struct PaletteColorId(u8);

/// RGBA Color
pub struct PaletteColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Material Id
pub struct MaterialId(u32);

/// Material
pub struct Material {
    // Surface
    roughness: f32,
    ior: f32,
    specular: f32,
    metallic: f32,
    transparency: f32,

    // Emission
    emission: f32,
    power: f32,
    ldr: f32,

    // Media
    density: f32,
}


pub struct SceneId(u32);

pub struct Scene {
    id: SceneId,
    name: String,
    nodes: Vec<SceneNode>,
}

pub struct SceneNodeId(u32);

pub enum SceneNode {
    Group {
        id: SceneNodeId,
        name: String,
        children: Vec<SceneNodeId>,
    },
    Model {
        id: SceneNodeId,
        name: String,
        model: ModelId,
    },
    Transform {
        id: SceneNodeId,
        name: String,
        rotation: [f32; 3],
        translation: [f32; 3],
    },
    Light {},
}