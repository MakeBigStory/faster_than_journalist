trait ResourcePool<R> {
    /// max number of resources in a pool
    const MAX_NUM_POOL_RESOURCES: i32 = 100;

    /// discard the resource pool
    fn discard(mut self);

    /// return true if the pool has been setup
    fn is_valid(mut self);

    /// update the pool, call once per frame
    fn update(mut self);

    // todo: refine it
    /// allocate a resource id
    fn alloc_resource();
    /// free a resource id
    fn free_resource();
}
