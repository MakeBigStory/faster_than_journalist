// todo: 可能不需要，目前大家都用float
use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;
//
use format::*;
use attribute::Attribute;

#[derive(Debug)]
struct AttributeLayout {
    attributes: Vec<Attribute>,
    total_size: usize
}

impl AttributeLayout {
    pub fn new() -> Self {
        AttributeLayout {
            attributes: vec![],
            total_size: 0
        }
    }

    pub fn size(&self) -> usize {
        self.total_size
    }

    pub fn add_attribute(&mut self, attribute : Attribute) {
        self.total_size += attribute.size;
        self.attributes.push(attribute);
    }

    pub fn enable(&mut self) -> Result<(), String> {
        for attribute in &mut self.attributes {
            attribute.enable();
        }

        Ok(())
    }
}