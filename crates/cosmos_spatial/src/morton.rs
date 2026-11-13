use glam::Vec3;

/// Spreads the low 21 bits of `v` so each bit occupies position `3 * i`.
#[inline]
const fn split_by_3_u21(v: u32) -> u64 {
    let mut x = (v as u64) & 0x001F_FFFF;
    x = (x | (x << 32)) & 0x001F_0000_0000_FFFF;
    x = (x | (x << 16)) & 0x001F_0000_FF00_00FF;
    x = (x | (x << 8)) & 0x100F_00F0_0F00_F00F;
    x = (x | (x << 4)) & 0x10C3_0C30_C30C_30C3;
    x = (x | (x << 2)) & 0x1249_2492_4924_9249;
    x
}

/// Compacts every third bit (`3 * i`) back into a 21-bit integer.
#[inline]
const fn compact_by_3_u21(v: u64) -> u32 {
    let mut x = v & 0x1249_2492_4924_9249;
    x = (x ^ (x >> 2)) & 0x10C3_0C30_C30C_30C3;
    x = (x ^ (x >> 4)) & 0x100F_00F0_0F00_F00F;
    x = (x ^ (x >> 8)) & 0x001F_0000_FF00_00FF;
    x = (x ^ (x >> 16)) & 0x001F_0000_0000_FFFF;
    x = (x ^ (x >> 32)) & 0x001F_FFFF;
    x as u32
}

/// Encodes 21-bit `(x, y, z)` integer grid coordinates into a 63-bit Z-order Morton code.
#[inline]
pub const fn morton3_encode(x: u32, y: u32, z: u32) -> u64 {
    split_by_3_u21(x) | (split_by_3_u21(y) << 1) | (split_by_3_u21(z) << 2)
}

/// Decodes a 63-bit Z-order Morton code back into 21-bit `(x, y, z)` coordinates.
#[inline]
pub const fn morton3_decode(code: u64) -> (u32, u32, u32) {
    (
        compact_by_3_u21(code),
        compact_by_3_u21(code >> 1),
        compact_by_3_u21(code >> 2),
    )
}

/// Maps a local position inside `[bounds_min, bounds_max]` to a 21-bit Morton code.
#[inline]
pub fn morton_for_local_pos(pos: Vec3, bounds_min: Vec3, bounds_extent: Vec3) -> u64 {
    const MAX_COORD: f32 = ((1u32 << 21) - 1) as f32;
    let norm = ((pos - bounds_min) / bounds_extent).clamp(Vec3::ZERO, Vec3::ONE);
    let qx = (norm.x * MAX_COORD) as u32;
    let qy = (norm.y * MAX_COORD) as u32;
    let qz = (norm.z * MAX_COORD) as u32;
    morton3_encode(qx, qy, qz)
}
