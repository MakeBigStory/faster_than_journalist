#[derive(Debug, Clone, Hash)]
pub struct Attribute {
    kind: AttributeKind,
    count: usize,
    item_count: usize,
    item_kind: DataKind,
    location: usize,
}

impl Attribute {
    #[inline]
    pub fn new(kind: AttributeKind, count: usize, location: usize) -> Self {
        let (item_count, item_kind) = kind.item_data();

        Attribute {
            kind: kind,
            count: count,
            item_count: item_count,
            item_kind: item_kind,
            location: location,
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