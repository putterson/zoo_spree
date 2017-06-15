void Shape_drop_virtual(b2Shape* self) {
    delete self;
}
/*b2Shape* Shape_clone_virtual(const b2Shape* self, 
                             b2BlockAllocator* allocator) {
    return self->Clone(allocator);
}*/

i32 Shape_get_type(const b2Shape* self) {
    return self->GetType();
}

i32 Shape_get_child_count_virtual(const b2Shape* self) {
    return self->GetChildCount();
}

bool Shape_test_point_virtual(const b2Shape* self,
                              const b2Transform* xf,
                              const b2Vec2* p) {
    return self->TestPoint(*xf, *p);
}

bool Shape_ray_cast_virtual(const b2Shape* self,
                            b2RayCastOutput* output,
                            const b2RayCastInput* input,
                            const b2Transform* transform,
                            i32 child_id) {
    return self->RayCast(output, *input, *transform, child_id);
}

void Shape_compute_aabb_virtual(const b2Shape* self,
                                b2AABB* aabb,
                                const b2Transform* xf,
                                i32 child_id) {
    self->ComputeAABB(aabb, *xf, child_id);
}

void Shape_compute_mass_virtual(const b2Shape* self,
                                b2MassData* data,
                                f32 density) {
    self->ComputeMass(data, density);
}

f32 Shape_get_radius(const b2Shape* self) {
    return self->m_radius;
}

void Shape_set_radius(b2Shape* self, f32 radius) {
    self->m_radius = radius;
}
