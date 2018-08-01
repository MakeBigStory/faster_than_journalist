use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;

use format::*;

#[derive(Debug, Clone, Hash)]
pub struct Attribute {
    pub name: String,
    pub kind: AttributeKind,
    pub item_count: usize,
    pub item_kind: DataKind,
    pub size: usize,
    location: isize
}

impl Attribute {
    #[inline]
    pub fn new(name: &str, kind: AttributeKind) -> Self {
        let (item_count, item_kind, size) = kind.item_data();

        Attribute {
            name: name.to_string(),
            kind,
            item_count,
            item_kind,
            location: -1,
            size
        }
    }

    fn is_valid(&self) -> bool {
        self.location >= 0
    }

    pub fn set_location(&mut self, location : isize) {
        self.location = location
    }

    pub fn transfer_data(&mut self, stride : isize, offset : usize) -> Result<(), String> {
        match self.is_valid() {
            true => {
                    // TODO: wrapper param should be more precise
                    let offset_array = [offset];
                    es20::wrapper::vertex_attrib_pointer(self.location as u32,
                                                         self.item_count as i32,
                                                         self.item_kind as u32,
                                                         false, stride as i32, &offset_array);
                    Ok(())
            },

            false => Err("transfer_data fail, Attribute location is invalid !!!".to_string())
        }
    }

    // TODO: gl get error
    pub fn enable(&self) -> Result<(), String> {
        if self.is_valid() {
            // TODO: wrapper param should be more precise
            es20::wrapper::enable_vertex_attrib_array(self.location as u32);
            Ok(())
        } else {
            Err("Enable attribute fail, Attribute location is invalid !!!".to_string())
        }
    }

    // TODO: gl get error
    pub fn disable(&self) -> Result<(), String> {
        if self.is_valid() {
            // TODO: wrapper param should be more precise
            es20::wrapper::disable_vertex_attrib_array(self.location as u32);
            Ok(())
        } else {
            Err("Disable attribute fail, Attribute location is invalid !!!".to_string())
        }
    }
}