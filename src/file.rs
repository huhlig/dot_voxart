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

#![allow(non_snake_case)]

use nom::{
    IResult,
    bytes::complete::{
        take,
        tag,
    },
    combinator::{
        map,
        map_res,
    },
    multi::{
        count,
        many0,
    },
    number::complete::{
        le_u8,
        le_u32,
        le_f32,
        le_i32,
    },
    sequence::tuple,
};
use std::collections::HashMap;

/// BUFFER type
pub struct VOXBuffer(Vec<u8>);

/// STRING Type
pub struct VOXString(String);

/// ATTRIB Type
pub struct VOXAtributes(HashMap<String, VOXValue>);

/// VALUE Type
pub enum VOXValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    BUFFER(BUFFER),
    STRING(STRING),
    ATTRIB(ATTRIB),
}

/// Color Type
pub struct VOXColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

/// Chunk
pub enum Chunk {
    INFO {
        attribs: HashMap<String, String>,
    },
    MODL {
        id: u32,
        name: String,
        size: (u8, u8),
        palette: u8,
    },
    RGBA {
        id: u32,
        name: String,
        colors: [(u8,u8,u8,u8); 256],
    },
    SCEN {
        id: u32,
        name: STRING,
        nodes: Vec<SceneNode>,
    },
    MATL {
        id: u32,
        name: String,
    },
}