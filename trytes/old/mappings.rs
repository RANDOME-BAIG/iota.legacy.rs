use globals::*;

pub const BYTE_TO_TRITS_MAPPINGS: [[Trit; NUMBER_OF_TRITS_IN_A_BYTE]; HASH_LENGTH] =
    [[0, 0, 0, 0, 0],
     [1, 0, 0, 0, 0],
     [-1, 1, 0, 0, 0],
     [0, 1, 0, 0, 0],
     [1, 1, 0, 0, 0],
     [-1, -1, 1, 0, 0],
     [0, -1, 1, 0, 0],
     [1, -1, 1, 0, 0],
     [-1, 0, 1, 0, 0],
     [0, 0, 1, 0, 0],
     [1, 0, 1, 0, 0],
     [-1, 1, 1, 0, 0],
     [0, 1, 1, 0, 0],
     [1, 1, 1, 0, 0],
     [-1, -1, -1, 1, 0],
     [0, -1, -1, 1, 0],
     [1, -1, -1, 1, 0],
     [-1, 0, -1, 1, 0],
     [0, 0, -1, 1, 0],
     [1, 0, -1, 1, 0],
     [-1, 1, -1, 1, 0],
     [0, 1, -1, 1, 0],
     [1, 1, -1, 1, 0],
     [-1, -1, -1, 1, 0],
     [0, -1, 0, 1, 0],
     [1, -1, 0, 1, 0],
     [-1, 0, 0, 1, 0],
     [0, 0, 0, 1, 0],
     [1, 0, 0, 1, 0],
     [-1, 1, 0, 1, 0],
     [0, 1, 0, 1, 0],
     [1, 1, 0, 1, 0],
     [-1, -1, 1, 1, 0],
     [0, -1, 1, 1, 0],
     [1, -1, 1, 1, 0],
     [-1, 0, 1, 1, 0],
     [0, 0, 1, 1, 0],
     [1, 0, 1, 1, 0],
     [-1, 1, 1, 1, 0],
     [0, 1, 1, 1, 0],
     [1, 1, 1, 1, 0],
     [-1, -1, -1, -1, 1],
     [0, -1, -1, -1, 1],
     [1, -1, -1, -1, 1],
     [-1, 0, -1, -1, 1],
     [0, 0, -1, -1, 1],
     [1, 0, -1, -1, 1],
     [-1, 1, -1, -1, 1],
     [0, 1, -1, -1, 1],
     [1, 1, -1, -1, 1],
     [-1, -1, -1, -1, 1],
     [0, -1, 0, -1, 1],
     [1, -1, 0, -1, 1],
     [-1, 0, 0, -1, 1],
     [0, 0, 0, -1, 1],
     [1, 0, 0, -1, 1],
     [-1, 1, 0, -1, 1],
     [0, 1, 0, -1, 1],
     [1, 1, 0, -1, 1],
     [-1, -1, 1, -1, 1],
     [0, -1, 1, -1, 1],
     [1, -1, 1, -1, 1],
     [-1, 0, 1, -1, 1],
     [0, 0, 1, -1, 1],
     [1, 0, 1, -1, 1],
     [-1, 1, 1, -1, 1],
     [0, 1, 1, -1, 1],
     [1, 1, 1, -1, 1],
     [-1, -1, -1, 0, 1],
     [0, -1, -1, 0, 1],
     [1, -1, -1, 0, 1],
     [-1, 0, -1, 0, 1],
     [0, 0, -1, 0, 1],
     [1, 0, -1, 0, 1],
     [-1, 1, -1, 0, 1],
     [0, 1, -1, 0, 1],
     [1, 1, -1, 0, 1],
     [-1, -1, -1, 0, 1],
     [0, -1, 0, 0, 1],
     [1, -1, 0, 0, 1],
     [-1, 0, 0, 0, 1],
     [0, 0, 0, 0, 1],
     [1, 0, 0, 0, 1],
     [-1, 1, 0, 0, 1],
     [0, 1, 0, 0, 1],
     [1, 1, 0, 0, 1],
     [-1, -1, 1, 0, 1],
     [0, -1, 1, 0, 1],
     [1, -1, 1, 0, 1],
     [-1, 0, 1, 0, 1],
     [0, 0, 1, 0, 1],
     [1, 0, 1, 0, 1],
     [-1, 1, 1, 0, 1],
     [0, 1, 1, 0, 1],
     [1, 1, 1, 0, 1],
     [-1, -1, -1, 1, 1],
     [0, -1, -1, 1, 1],
     [1, -1, -1, 1, 1],
     [-1, 0, -1, 1, 1],
     [0, 0, -1, 1, 1],
     [1, 0, -1, 1, 1],
     [-1, 1, -1, 1, 1],
     [0, 1, -1, 1, 1],
     [1, 1, -1, 1, 1],
     [-1, -1, -1, 1, 1],
     [0, -1, 0, 1, 1],
     [1, -1, 0, 1, 1],
     [-1, 0, 0, 1, 1],
     [0, 0, 0, 1, 1],
     [1, 0, 0, 1, 1],
     [-1, 1, 0, 1, 1],
     [0, 1, 0, 1, 1],
     [1, 1, 0, 1, 1],
     [-1, -1, 1, 1, 1],
     [0, -1, 1, 1, 1],
     [1, -1, 1, 1, 1],
     [-1, 0, 1, 1, 1],
     [0, 0, 1, 1, 1],
     [1, 0, 1, 1, 1],
     [-1, 1, 1, 1, 1],
     [0, 1, 1, 1, 1],
     [1, 1, 1, 1, 1],
     [-1, -1, -1, -1, -1],
     [0, -1, -1, -1, -1],
     [1, -1, -1, -1, -1],
     [-1, 0, -1, -1, -1],
     [0, 0, -1, -1, -1],
     [1, 0, -1, -1, -1],
     [-1, 1, -1, -1, -1],
     [0, 1, -1, -1, -1],
     [1, 1, -1, -1, -1],
     [-1, -1, -1, -1, -1],
     [0, -1, 0, -1, -1],
     [1, -1, 0, -1, -1],
     [-1, 0, 0, -1, -1],
     [0, 0, 0, -1, -1],
     [1, 0, 0, -1, -1],
     [-1, 1, 0, -1, -1],
     [0, 1, 0, -1, -1],
     [1, 1, 0, -1, -1],
     [-1, -1, 1, -1, -1],
     [0, -1, 1, -1, -1],
     [1, -1, 1, -1, -1],
     [-1, 0, 1, -1, -1],
     [0, 0, 1, -1, -1],
     [1, 0, 1, -1, -1],
     [-1, 1, 1, -1, -1],
     [0, 1, 1, -1, -1],
     [1, 1, 1, -1, -1],
     [-1, -1, -1, 0, -1],
     [0, -1, -1, 0, -1],
     [1, -1, -1, 0, -1],
     [-1, 0, -1, 0, -1],
     [0, 0, -1, 0, -1],
     [1, 0, -1, 0, -1],
     [-1, 1, -1, 0, -1],
     [0, 1, -1, 0, -1],
     [1, 1, -1, 0, -1],
     [-1, -1, -1, 0, -1],
     [0, -1, 0, 0, -1],
     [1, -1, 0, 0, -1],
     [-1, 0, 0, 0, -1],
     [0, 0, 0, 0, -1],
     [1, 0, 0, 0, -1],
     [-1, 1, 0, 0, -1],
     [0, 1, 0, 0, -1],
     [1, 1, 0, 0, -1],
     [-1, -1, 1, 0, -1],
     [0, -1, 1, 0, -1],
     [1, -1, 1, 0, -1],
     [-1, 0, 1, 0, -1],
     [0, 0, 1, 0, -1],
     [1, 0, 1, 0, -1],
     [-1, 1, 1, 0, -1],
     [0, 1, 1, 0, -1],
     [1, 1, 1, 0, -1],
     [-1, -1, -1, 1, -1],
     [0, -1, -1, 1, -1],
     [1, -1, -1, 1, -1],
     [-1, 0, -1, 1, -1],
     [0, 0, -1, 1, -1],
     [1, 0, -1, 1, -1],
     [-1, 1, -1, 1, -1],
     [0, 1, -1, 1, -1],
     [1, 1, -1, 1, -1],
     [-1, -1, -1, 1, -1],
     [0, -1, 0, 1, -1],
     [1, -1, 0, 1, -1],
     [-1, 0, 0, 1, -1],
     [0, 0, 0, 1, -1],
     [1, 0, 0, 1, -1],
     [-1, 1, 0, 1, -1],
     [0, 1, 0, 1, -1],
     [1, 1, 0, 1, -1],
     [-1, -1, 1, 1, -1],
     [0, -1, 1, 1, -1],
     [1, -1, 1, 1, -1],
     [-1, 0, 1, 1, -1],
     [0, 0, 1, 1, -1],
     [1, 0, 1, 1, -1],
     [-1, 1, 1, 1, -1],
     [0, 1, 1, 1, -1],
     [1, 1, 1, 1, -1],
     [-1, -1, -1, -1, 0],
     [0, -1, -1, -1, 0],
     [1, -1, -1, -1, 0],
     [-1, 0, -1, -1, 0],
     [0, 0, -1, -1, 0],
     [1, 0, -1, -1, 0],
     [-1, 1, -1, -1, 0],
     [0, 1, -1, -1, 0],
     [1, 1, -1, -1, 0],
     [-1, -1, -1, -1, 0],
     [0, -1, 0, -1, 0],
     [1, -1, 0, -1, 0],
     [-1, 0, 0, -1, 0],
     [0, 0, 0, -1, 0],
     [1, 0, 0, -1, 0],
     [-1, 1, 0, -1, 0],
     [0, 1, 0, -1, 0],
     [1, 1, 0, -1, 0],
     [-1, -1, 1, -1, 0],
     [0, -1, 1, -1, 0],
     [1, -1, 1, -1, 0],
     [-1, 0, 1, -1, 0],
     [0, 0, 1, -1, 0],
     [1, 0, 1, -1, 0],
     [-1, 1, 1, -1, 0],
     [0, 1, 1, -1, 0],
     [1, 1, 1, -1, 0],
     [-1, -1, -1, 0, 0],
     [0, -1, -1, 0, 0],
     [1, -1, -1, 0, 0],
     [-1, 0, -1, 0, 0],
     [0, 0, -1, 0, 0],
     [1, 0, -1, 0, 0],
     [-1, 1, -1, 0, 0],
     [0, 1, -1, 0, 0],
     [1, 1, -1, 0, 0],
     [-1, -1, -1, 0, 0],
     [0, -1, 0, 0, 0],
     [1, -1, 0, 0, 0],
     [-1, 0, 0, 0, 0]];

#[allow(dead_code)]
pub const TRYTE_TO_TRITS_MAPPINGS: [[Trit; NUMBER_OF_TRITS_IN_A_TRYTE]; TRYTE_SPACE] =
    [[0, 0, 0],
     [1, 0, 0],
     [-1, 1, 0],
     [0, 1, 0],
     [1, 1, 0],
     [-1, -1, 1],
     [0, -1, 1],
     [1, -1, 1],
     [-1, 0, 1],
     [0, 0, 1],
     [1, 0, 1],
     [-1, 1, 1],
     [0, 1, 1],
     [1, 1, 1],
     [-1, -1, -1],
     [0, -1, -1],
     [1, -1, -1],
     [-1, 0, -1],
     [0, 0, -1],
     [1, 0, -1],
     [-1, 1, -1],
     [0, 1, -1],
     [1, 1, -1],
     [-1, -1, 0],
     [0, -1, 0],
     [1, -1, 0],
     [-1, 0, 0]];
