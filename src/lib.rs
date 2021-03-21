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

//!
//!

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
// TODO: Remove These
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::Read;
use std::fs::File;
use std::path::Path;

mod file;
mod vox;

pub fn load<P: AsRef<Path>>(filename: P) -> Result<VoxartData, &'static str> {
    match File::open(filename) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).expect("Unable to read file");
            load_bytes(&buffer)
        }
        Err(_) => Err("Unable to load file"),
    }
}

pub fn load_bytes(bytes: &[u8]) -> Result<VoxartData, &'static str> {
    match parse_vox_file(CompleteByteSlice(bytes)) {
        Ok((_, parsed)) => Ok(parsed),
        Err(_) => Err("Not a valid Voxart .vox file"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
