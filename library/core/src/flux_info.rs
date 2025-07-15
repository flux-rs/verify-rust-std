//! This file contains auxiliary definitions for Flux

/// List of properties tracked for the result of primitive bitwise operations.
/// See the following link for more information on how extensible properties for primitive operations work:
/// <https://flux-rs.github.io/flux/guide/specifications.html#extensible-properties-for-primitive-ops>
#[flux::defs {

    fn as_int(x:int) -> int { 
        x 
    }

    fn char_to_int(x:char) -> int { 
        cast(x) 
    } 

    property ShiftRightByFour[>>](x, y) {
        16 * [>>](x, 4) == x
    }

    property MaskLess[&](x, y) {
        [&](x, y) <= x && [&](x, y) <= y
    }

    property ShiftLeft[<<](n, k) {
        0 < k => n <= [<<](n, k)
    }
}]
const _: () = {};
