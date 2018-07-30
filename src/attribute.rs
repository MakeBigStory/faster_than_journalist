use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;

use format::*;

//todo: 目前根据一个attribute对应一个buffer
#[derive(Debug, Clone, Hash)]
pub struct Attribute {
    pub kind: AttributeKind,
    pub count: usize,
    pub item_count: usize,
    pub item_kind: DataKind,
    pub location: usize,
}

impl Attribute {
    #[inline]
    pub fn new(kind: AttributeKind, count: usize, location: usize) -> Self {
        let (item_count, item_kind) = kind.item_data();

        Attribute {
            kind,
            count,
            item_count,
            item_kind,
            location,
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline(always)]
    pub fn item_count(&self) -> usize {
        self.item_count
    }
    #[inline(always)]
    pub fn item_kind(&self) -> DataKind {
        self.item_kind
    }
    #[inline(always)]
    pub fn location(&self) -> usize {
        self.location
    }
}