//! This file contains auxiliary definitions for Flux

use crate::hash;
use crate::time;
use crate::unicode;

/// List of properties tracked for the result of primitive bitwise operations.
/// See the following link for more information on how extensible properties for primitive operations work:
/// <https://flux-rs.github.io/flux/guide/specifications.html#extensible-properties-for-primitive-ops>
#[flux::defs {
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

    fn is_ascii_uppercase(n: int) -> bool {
        cast('A') <= n && n <= cast('Z')
    }

    fn is_ascii_lowercase(n: int) -> bool {
        cast('a') <= n && n <= cast('z')
    }

    fn to_ascii_uppercase(n: int) -> int {
        n - (cast(is_ascii_lowercase(n)) * 32)
    }

    fn to_ascii_lowercase(n: int) -> int {
        n + (cast(is_ascii_uppercase(n)) * 32)
    }

    property BitXor0[^](x, y) {
        (y == 0) => [^](x, y) == x
    }

    property BitXor32[^](x, y) {
        (is_ascii_lowercase(x) && y == 32) => [^](x, y) == x - 32
    }

    property BitOr0[|](x, y) {
        (y == 0) => [|](x, y) == x
    }

    property BitOr32[|](x, y) {
        (is_ascii_uppercase(x) && y == 32) => [|](x, y) == x + 32
    }

}]
#[flux::specs {
    mod hash {
        mod sip {
            struct Hasher {
              k0: u64,
              k1: u64,
              length: usize, // how many bytes we've processed
              state: State,  // hash State
              tail: u64,     // unprocessed bytes le
              ntail: usize{v: v <= 8}, // how many bytes in tail are valid
              _marker: PhantomData<S>,
            }

            impl hash::Hasher for Hasher {
                fn write(self: &mut Self, msg: &[u8]) ensures self: Self; // mut-ref-unfolding
            }
        }

        impl BuildHasherDefault {
            #[trusted(reason="https://github.com/flux-rs/flux/issues/1185")]
            fn new() -> Self;
        }

        impl clone::Clone for BuildHasherDefault {
            #[trusted(reason="https://github.com/flux-rs/flux/issues/1185")]
            fn clone(self: &Self) -> Self;
        }
    }

    mod time {
        impl fmt::Debug for Duration {
            #[trusted(reason="modular arithmetic invariant inside nested fmt_decimal")]
            fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result;
        }
    }

    mod unicode {
        mod unicode_data {
            fn bitset_search(
               needle: _,
               chunk_idx_map:&[u8{v: v < N1};_],
               bitset_chunk_idx: &[[u8{v: v < CANONICAL + CANONICALIZED}; _]; _],
               bitset_canonical: _,
               bitset_canonicalized: &[(u8{v:v < CANONICAL}, u8); _],
            ) -> bool requires CHUNK_SIZE > 0;

            #[trusted(reason="TODO:LEAN:binary-search")]
            fn skip_search(needle: _, short_offset_runs: _, offsets: _ ) -> bool;

            mod lowercase {
               static BITSET_CHUNKS_MAP: [u8{v:v < 20}; 123];
               static BITSET_INDEX_CHUNKS: [[u8{v:v < 79}; 16]; 20];
               static BITSET_MAPPING: [(u8{v:v < 57}, u8); 22];
            }

            mod uppercase {
              static BITSET_CHUNKS_MAP: [u8{v:v < 17}; 125];
               static BITSET_INDEX_CHUNKS: [[u8{v:v < 69}; 16]; 17];
               static BITSET_MAPPING: [(u8{v : v < 44}, u8); 25];
            }
        }
    }
}]
const _: () = {};
