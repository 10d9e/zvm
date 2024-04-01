use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use tfhe::prelude::*;
use tfhe::{FheBool, FheUint128, FheUint16, FheUint32, FheUint64, FheUint8};

macro_rules! binary_op {
    ($($op:ident, $op_method:ident, $op_token:tt);*) => {
        impl Value {
            $(
                pub fn $op(self, other: Self) -> Self {
                    match (self, other) {
                        // Plaintext, Plaintext
                        (Value::Bool(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Bool(a), Value::Uint8(b)) => Value::Uint8((a as u8) $op_token b),
                        (Value::Bool(a), Value::Uint16(b)) => Value::Uint16((a as u16) $op_token b),
                        (Value::Bool(a), Value::Uint32(b)) => Value::Uint32((a as u32) $op_token b),
                        (Value::Bool(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Bool(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint8(a), Value::Bool(b)) => Value::Uint8(a $op_token (b as u8)),
                        (Value::Uint8(a), Value::Uint8(b)) => Value::Uint8(a $op_token b),
                        (Value::Uint8(a), Value::Uint16(b)) => Value::Uint16((a as u16) $op_token b),
                        (Value::Uint8(a), Value::Uint32(b)) => Value::Uint32((a as u32) $op_token b),
                        (Value::Uint8(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Uint8(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint16(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint16(a), Value::Uint8(b)) => Value::Uint16(a $op_token (b as u16)),
                        (Value::Uint16(a), Value::Uint16(b)) => Value::Uint16(a $op_token b),
                        (Value::Uint16(a), Value::Uint32(b)) => Value::Uint32((a as u32) $op_token b),
                        (Value::Uint16(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Uint16(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint32(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint32(a), Value::Uint8(b)) => Value::Uint32(a $op_token (b as u32)),
                        (Value::Uint32(a), Value::Uint16(b)) => Value::Uint32(a $op_token (b as u32)),
                        (Value::Uint32(a), Value::Uint32(b)) => Value::Uint32(a $op_token b),
                        (Value::Uint32(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Uint32(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint64(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint64(a), Value::Uint8(b)) => Value::Uint64(a $op_token (b as u64)),
                        (Value::Uint64(a), Value::Uint16(b)) => Value::Uint64(a $op_token (b as u64)),
                        (Value::Uint64(a), Value::Uint32(b)) => Value::Uint64(a $op_token (b as u64)),
                        (Value::Uint64(a), Value::Uint64(b)) => Value::Uint64(a $op_token b),
                        (Value::Uint64(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint128(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint128(a), Value::Uint8(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint16(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint32(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint64(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint128(b)) => Value::Uint128(a $op_token b),

                        // Plaintext, Encrypted
                        (Value::Bool(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint8(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint16(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint32(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint64(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint128(_b)) => unimplemented!(),

                        (Value::Uint8(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint8(a), Value::Euint8(b)) => Value::Euint8(a $op_token b),
                        (Value::Uint8(a), Value::Euint16(b)) => Value::Euint16((a as u16) $op_token b),
                        (Value::Uint8(a), Value::Euint32(b)) => Value::Euint32((a as u32) $op_token b),
                        (Value::Uint8(a), Value::Euint64(b)) => Value::Euint64((a as u64) $op_token b),
                        (Value::Uint8(a), Value::Euint128(b)) => Value::Euint128((a as u128) $op_token b),

                        (Value::Uint16(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint16(a), Value::Euint8(b)) => Value::Euint8((a as u8) $op_token FheUint8::cast_from(b)),
                        (Value::Uint16(a), Value::Euint16(b)) => Value::Euint16(a $op_token b),
                        (Value::Uint16(a), Value::Euint32(b)) => Value::Euint32((a as u32) $op_token FheUint32::cast_from(b)),
                        (Value::Uint16(a), Value::Euint64(b)) => Value::Euint64((a as u64) $op_token FheUint64::cast_from(b)),
                        (Value::Uint16(a), Value::Euint128(b)) => Value::Euint128((a as u128) $op_token FheUint128::cast_from(b)),

                        (Value::Uint32(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint32(a), Value::Euint8(b)) => Value::Euint8((a as u8) $op_token FheUint8::cast_from(b)),
                        (Value::Uint32(a), Value::Euint16(b)) => Value::Euint16((a as u16) $op_token FheUint16::cast_from(b)),
                        (Value::Uint32(a), Value::Euint32(b)) => Value::Euint32(a $op_token b),
                        (Value::Uint32(a), Value::Euint64(b)) => Value::Euint64((a as u64) $op_token FheUint64::cast_from(b)),
                        (Value::Uint32(a), Value::Euint128(b)) => Value::Euint128((a as u128) $op_token FheUint128::cast_from(b)),

                        (Value::Uint64(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint64(a), Value::Euint8(b)) => Value::Euint8((a as u8) $op_token FheUint8::cast_from(b)),
                        (Value::Uint64(a), Value::Euint16(b)) => Value::Euint16((a as u16) $op_token FheUint16::cast_from(b)),
                        (Value::Uint64(a), Value::Euint32(b)) => Value::Euint32((a as u32) $op_token FheUint32::cast_from(b)),
                        (Value::Uint64(a), Value::Euint64(b)) => Value::Euint64(a $op_token b),
                        (Value::Uint64(a), Value::Euint128(b)) => Value::Euint128((a as u128) $op_token FheUint128::cast_from(b)),

                        (Value::Uint128(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint128(a), Value::Euint8(b)) => Value::Euint8((a as u8) $op_token FheUint8::cast_from(b)),
                        (Value::Uint128(a), Value::Euint16(b)) => Value::Euint16((a as u16) $op_token FheUint16::cast_from(b)),
                        (Value::Uint128(a), Value::Euint32(b)) => Value::Euint32((a as u32) $op_token FheUint32::cast_from(b)),
                        (Value::Uint128(a), Value::Euint64(b)) => Value::Euint64((a as u64) $op_token FheUint64::cast_from(b)),
                        (Value::Uint128(a), Value::Euint128(b)) => Value::Euint128(a $op_token b),

                        // Encrypted, Plaintext
                        (Value::Ebool(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint8(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint16(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint32(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint64(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint128(_a), Value::Bool(_b)) => unimplemented!(),

                        (Value::Ebool(_a), Value::Uint8(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint8(b)) => Value::Euint8(a $op_token b),
                        (Value::Euint16(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint16(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint16(b)) => Value::Euint16(a $op_token b),
                        (Value::Euint32(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint32(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint32(b)) => Value::Euint32(a $op_token b),
                        (Value::Euint64(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint64(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint64(b)) => Value::Euint64(a $op_token b),
                        (Value::Euint128(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint128(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint128(b)) => Value::Euint128(a $op_token b),

                        // Encrypted, Encrypted
                        (Value::Ebool(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint8(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint16(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint32(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint64(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint128(_b)) => unimplemented!(),

                        (Value::Euint8(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Euint8(b)) => Value::Euint8(a $op_token b),
                        (Value::Euint8(a), Value::Euint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint8(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint8(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint8(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint16(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint16(a), Value::Euint8(b)) => Value::Euint16(a $op_token FheUint16::cast_from(b)),
                        (Value::Euint16(a), Value::Euint16(b)) => Value::Euint16(a $op_token b),
                        (Value::Euint16(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint32(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint32(a), Value::Euint8(b)) => Value::Euint32(a $op_token FheUint32::cast_from(b)),
                        (Value::Euint32(a), Value::Euint16(b)) => Value::Euint32(a $op_token FheUint32::cast_from(b)),
                        (Value::Euint32(a), Value::Euint32(b)) => Value::Euint32(a $op_token b),
                        (Value::Euint32(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint64(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint64(a), Value::Euint8(b)) => Value::Euint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Euint64(a), Value::Euint16(b)) => Value::Euint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Euint64(a), Value::Euint32(b)) => Value::Euint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Euint64(a), Value::Euint64(b)) => Value::Euint64(a $op_token b),
                        (Value::Euint64(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint128(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint128(a), Value::Euint8(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint16(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint32(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint64(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint128(b)) => Value::Euint128(a $op_token b),

                    }
                }
            )*
        }
    };
}

binary_op! {
    add, add_method, +;
    sub, sub_method, -;
    mul, mul_method, *;
    and, and_method, &;
    or, or_method, |;
    xor, xor_method, ^
}

impl Value {
    pub fn div(self, other: Self) -> Self {
        match (self, other) {
            // Plaintext, Plaintext
            (Value::Bool(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Bool(a), Value::Uint8(b)) => Value::Uint8((a as u8) / b),
            (Value::Bool(a), Value::Uint16(b)) => Value::Uint16((a as u16) / b),
            (Value::Bool(a), Value::Uint32(b)) => Value::Uint32((a as u32) / b),
            (Value::Bool(a), Value::Uint64(b)) => Value::Uint64((a as u64) / b),
            (Value::Bool(a), Value::Uint128(b)) => Value::Uint128((a as u128) / b),

            (Value::Uint8(a), Value::Bool(b)) => Value::Uint8(a / (b as u8)),
            (Value::Uint8(a), Value::Uint8(b)) => Value::Uint8(a / b),
            (Value::Uint8(a), Value::Uint16(b)) => Value::Uint16((a as u16) / b),
            (Value::Uint8(a), Value::Uint32(b)) => Value::Uint32((a as u32) / b),
            (Value::Uint8(a), Value::Uint64(b)) => Value::Uint64((a as u64) / b),
            (Value::Uint8(a), Value::Uint128(b)) => Value::Uint128((a as u128) / b),

            (Value::Uint16(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Uint16(a), Value::Uint8(b)) => Value::Uint16(a / (b as u16)),
            (Value::Uint16(a), Value::Uint16(b)) => Value::Uint16(a / b),
            (Value::Uint16(a), Value::Uint32(b)) => Value::Uint32((a as u32) / b),
            (Value::Uint16(a), Value::Uint64(b)) => Value::Uint64((a as u64) / b),
            (Value::Uint16(a), Value::Uint128(b)) => Value::Uint128((a as u128) / b),

            (Value::Uint32(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Uint32(a), Value::Uint8(b)) => Value::Uint32(a / (b as u32)),
            (Value::Uint32(a), Value::Uint16(b)) => Value::Uint32(a / (b as u32)),
            (Value::Uint32(a), Value::Uint32(b)) => Value::Uint32(a / b),
            (Value::Uint32(a), Value::Uint64(b)) => Value::Uint64((a as u64) / b),
            (Value::Uint32(a), Value::Uint128(b)) => Value::Uint128((a as u128) / b),

            (Value::Uint64(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Uint64(a), Value::Uint8(b)) => Value::Uint64(a / (b as u64)),
            (Value::Uint64(a), Value::Uint16(b)) => Value::Uint64(a / (b as u64)),
            (Value::Uint64(a), Value::Uint32(b)) => Value::Uint64(a / (b as u64)),
            (Value::Uint64(a), Value::Uint64(b)) => Value::Uint64(a / b),
            (Value::Uint64(a), Value::Uint128(b)) => Value::Uint128((a as u128) / b),

            (Value::Uint128(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Uint128(a), Value::Uint8(b)) => Value::Uint128(a / (b as u128)),
            (Value::Uint128(a), Value::Uint16(b)) => Value::Uint128(a / (b as u128)),
            (Value::Uint128(a), Value::Uint32(b)) => Value::Uint128(a / (b as u128)),
            (Value::Uint128(a), Value::Uint64(b)) => Value::Uint128(a / (b as u128)),
            (Value::Uint128(a), Value::Uint128(b)) => Value::Uint128(a / b),

            // Plaintext, Encrypted
            (Value::Bool(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Bool(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Bool(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Bool(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Bool(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Bool(_a), Value::Euint128(_b)) => unimplemented!(),

            (Value::Uint8(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Uint8(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Uint8(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Uint8(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Uint8(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Uint8(_a), Value::Euint128(_b)) => unimplemented!(),

            (Value::Uint16(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Uint16(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Uint16(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Uint16(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Uint16(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Uint16(_a), Value::Euint128(_b)) => unimplemented!(),

            (Value::Uint32(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Uint32(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Uint32(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Uint32(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Uint32(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Uint32(_a), Value::Euint128(_b)) => unimplemented!(),

            (Value::Uint64(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Uint64(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Uint64(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Uint64(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Uint64(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Uint64(_a), Value::Euint128(_b)) => unimplemented!(),

            (Value::Uint128(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Uint128(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Uint128(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Uint128(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Uint128(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Uint128(_a), Value::Euint128(_b)) => unimplemented!(),

            // Encrypted, Plaintext
            (Value::Ebool(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Euint8(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Euint16(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Euint32(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Euint64(_a), Value::Bool(_b)) => unimplemented!(),
            (Value::Euint128(_a), Value::Bool(_b)) => unimplemented!(),

            (Value::Ebool(_a), Value::Uint8(_b)) => unimplemented!(),
            (Value::Euint8(a), Value::Uint8(b)) => Value::Euint8(a / b),
            (Value::Euint16(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) / b),
            (Value::Euint32(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) / b),
            (Value::Euint64(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) / b),
            (Value::Euint128(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) / b),

            (Value::Ebool(_a), Value::Uint16(_b)) => unimplemented!(),
            (Value::Euint8(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) / b),
            (Value::Euint16(a), Value::Uint16(b)) => Value::Euint16(a / b),
            (Value::Euint32(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) / b),
            (Value::Euint64(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) / b),
            (Value::Euint128(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) / b),

            (Value::Ebool(_a), Value::Uint32(_b)) => unimplemented!(),
            (Value::Euint8(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) / b),
            (Value::Euint16(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) / b),
            (Value::Euint32(a), Value::Uint32(b)) => Value::Euint32(a / b),
            (Value::Euint64(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) / b),
            (Value::Euint128(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) / b),

            (Value::Ebool(_a), Value::Uint64(_b)) => unimplemented!(),
            (Value::Euint8(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),
            (Value::Euint16(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),
            (Value::Euint32(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),
            (Value::Euint64(a), Value::Uint64(b)) => Value::Euint64(a / b),
            (Value::Euint128(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),

            (Value::Ebool(_a), Value::Uint128(_b)) => unimplemented!(),
            (Value::Euint8(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) / b),
            (Value::Euint16(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) / b),
            (Value::Euint32(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) / b),
            (Value::Euint64(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) / b),
            (Value::Euint128(a), Value::Uint128(b)) => Value::Euint128(a / b),

            // Encrypted, Encrypted
            (Value::Ebool(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Ebool(_a), Value::Euint8(_b)) => unimplemented!(),
            (Value::Ebool(_a), Value::Euint16(_b)) => unimplemented!(),
            (Value::Ebool(_a), Value::Euint32(_b)) => unimplemented!(),
            (Value::Ebool(_a), Value::Euint64(_b)) => unimplemented!(),
            (Value::Ebool(_a), Value::Euint128(_b)) => unimplemented!(),

            (Value::Euint8(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Euint8(a), Value::Euint8(b)) => Value::Euint8(a / b),
            (Value::Euint8(a), Value::Euint16(b)) => Value::Euint16(FheUint16::cast_from(a) / b),
            (Value::Euint8(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(a) / b),
            (Value::Euint8(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),
            (Value::Euint8(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) / b),

            (Value::Euint16(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Euint16(a), Value::Euint8(b)) => Value::Euint16(a / FheUint16::cast_from(b)),
            (Value::Euint16(a), Value::Euint16(b)) => Value::Euint16(a / b),
            (Value::Euint16(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(a) / b),
            (Value::Euint16(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),
            (Value::Euint16(a), Value::Euint128(b)) => {
                Value::Euint128(FheUint128::cast_from(a) / b)
            }

            (Value::Euint32(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Euint32(a), Value::Euint8(b)) => Value::Euint32(a / FheUint32::cast_from(b)),
            (Value::Euint32(a), Value::Euint16(b)) => Value::Euint32(a / FheUint32::cast_from(b)),
            (Value::Euint32(a), Value::Euint32(b)) => Value::Euint32(a / b),
            (Value::Euint32(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) / b),
            (Value::Euint32(a), Value::Euint128(b)) => {
                Value::Euint128(FheUint128::cast_from(a) / b)
            }

            (Value::Euint64(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Euint64(a), Value::Euint8(b)) => Value::Euint64(a / FheUint64::cast_from(b)),
            (Value::Euint64(a), Value::Euint16(b)) => Value::Euint64(a / FheUint64::cast_from(b)),
            (Value::Euint64(a), Value::Euint32(b)) => Value::Euint64(a / FheUint64::cast_from(b)),
            (Value::Euint64(a), Value::Euint64(b)) => Value::Euint64(a / b),
            (Value::Euint64(a), Value::Euint128(b)) => {
                Value::Euint128(FheUint128::cast_from(a) / b)
            }

            (Value::Euint128(_a), Value::Ebool(_b)) => unimplemented!(),
            (Value::Euint128(a), Value::Euint8(b)) => Value::Euint128(a / FheUint128::cast_from(b)),
            (Value::Euint128(a), Value::Euint16(b)) => {
                Value::Euint128(a / FheUint128::cast_from(b))
            }
            (Value::Euint128(a), Value::Euint32(b)) => {
                Value::Euint128(a / FheUint128::cast_from(b))
            }
            (Value::Euint128(a), Value::Euint64(b)) => {
                Value::Euint128(a / FheUint128::cast_from(b))
            }
            (Value::Euint128(a), Value::Euint128(b)) => Value::Euint128(a / b),
        }
    }
}

macro_rules! binary_shift_op {
    ($($op:ident, $op_method:ident, $op_token:tt, $op_token_alt:tt);*) => {
        impl Value {
            $(
                pub fn $op(self, other: Self) -> Self {
                    match (self, other) {
                        // Plaintext, Plaintext
                        (Value::Bool(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Bool(a), Value::Uint8(b)) => Value::Uint8((a as u8) $op_token b),
                        (Value::Bool(a), Value::Uint16(b)) => Value::Uint16((a as u16) $op_token b),
                        (Value::Bool(a), Value::Uint32(b)) => Value::Uint32((a as u32) $op_token b),
                        (Value::Bool(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Bool(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint8(a), Value::Bool(b)) => Value::Uint8(a $op_token (b as u8)),
                        (Value::Uint8(a), Value::Uint8(b)) => Value::Uint8(a $op_token b),
                        (Value::Uint8(a), Value::Uint16(b)) => Value::Uint16((a as u16) $op_token b),
                        (Value::Uint8(a), Value::Uint32(b)) => Value::Uint32((a as u32) $op_token b),
                        (Value::Uint8(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Uint8(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint16(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint16(a), Value::Uint8(b)) => Value::Uint16(a $op_token (b as u16)),
                        (Value::Uint16(a), Value::Uint16(b)) => Value::Uint16(a $op_token b),
                        (Value::Uint16(a), Value::Uint32(b)) => Value::Uint32((a as u32) $op_token b),
                        (Value::Uint16(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Uint16(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint32(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint32(a), Value::Uint8(b)) => Value::Uint32(a $op_token (b as u32)),
                        (Value::Uint32(a), Value::Uint16(b)) => Value::Uint32(a $op_token (b as u32)),
                        (Value::Uint32(a), Value::Uint32(b)) => Value::Uint32(a $op_token b),
                        (Value::Uint32(a), Value::Uint64(b)) => Value::Uint64((a as u64) $op_token b),
                        (Value::Uint32(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint64(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint64(a), Value::Uint8(b)) => Value::Uint64(a $op_token (b as u64)),
                        (Value::Uint64(a), Value::Uint16(b)) => Value::Uint64(a $op_token (b as u64)),
                        (Value::Uint64(a), Value::Uint32(b)) => Value::Uint64(a $op_token (b as u64)),
                        (Value::Uint64(a), Value::Uint64(b)) => Value::Uint64(a $op_token b),
                        (Value::Uint64(a), Value::Uint128(b)) => Value::Uint128((a as u128) $op_token b),

                        (Value::Uint128(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Uint128(a), Value::Uint8(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint16(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint32(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint64(b)) => Value::Uint128(a $op_token (b as u128)),
                        (Value::Uint128(a), Value::Uint128(b)) => Value::Uint128(a $op_token b),

                        // Plaintext, Encrypted (change this)

                        (Value::Bool(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint8(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint16(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint32(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint64(_b)) => unimplemented!(),
                        (Value::Bool(_a), Value::Euint128(_b)) => unimplemented!(),

                        (Value::Uint8(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint8(a), Value::Euint8(b)) => Value::Euint8(b $op_token_alt a),
                        (Value::Uint8(a), Value::Euint16(b)) => Value::Euint16(b $op_token_alt (a as u16)),
                        (Value::Uint8(a), Value::Euint32(b)) => Value::Euint32(b $op_token_alt (a as u32)),
                        (Value::Uint8(a), Value::Euint64(b)) => Value::Euint64(b $op_token_alt (a as u64)),
                        (Value::Uint8(a), Value::Euint128(b)) => Value::Euint128(b $op_token_alt (a as u128)),

                        (Value::Uint16(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint16(a), Value::Euint8(b)) => Value::Euint8(FheUint8::cast_from(b) $op_token_alt a),
                        (Value::Uint16(a), Value::Euint16(b)) => Value::Euint16(b $op_token_alt a),
                        (Value::Uint16(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(b) $op_token_alt (a as u32)),
                        (Value::Uint16(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(b) $op_token_alt (a as u64)),
                        (Value::Uint16(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(b) $op_token_alt (a as u128)),

                        (Value::Uint32(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint32(a), Value::Euint8(b)) => Value::Euint8(FheUint8::cast_from(b) $op_token_alt a),
                        (Value::Uint32(a), Value::Euint16(b)) => Value::Euint16(FheUint16::cast_from(b) $op_token_alt a),
                        (Value::Uint32(a), Value::Euint32(b)) => Value::Euint32(b $op_token_alt a),
                        (Value::Uint32(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(b) $op_token_alt (a as u64)),
                        (Value::Uint32(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(b) $op_token_alt (a as u128)),

                        (Value::Uint64(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint64(a), Value::Euint8(b)) => Value::Euint8(FheUint8::cast_from(b) $op_token_alt a),
                        (Value::Uint64(a), Value::Euint16(b)) => Value::Euint16(FheUint16::cast_from(b) $op_token_alt a),
                        (Value::Uint64(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(b) $op_token_alt a),
                        (Value::Uint64(a), Value::Euint64(b)) => Value::Euint64(b $op_token_alt a),
                        (Value::Uint64(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(b) $op_token_alt (a as u128)),

                        (Value::Uint128(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Uint128(a), Value::Euint8(b)) => Value::Euint8(FheUint8::cast_from(b) $op_token_alt a),
                        (Value::Uint128(a), Value::Euint16(b)) => Value::Euint16(FheUint16::cast_from(b) $op_token_alt a),
                        (Value::Uint128(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(b) $op_token_alt a),
                        (Value::Uint128(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(b) $op_token_alt a),
                        (Value::Uint128(a), Value::Euint128(b)) => Value::Euint128(b $op_token_alt a),

                        // Encrypted, Plaintext
                        (Value::Ebool(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint8(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint16(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint32(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint64(_a), Value::Bool(_b)) => unimplemented!(),
                        (Value::Euint128(_a), Value::Bool(_b)) => unimplemented!(),

                        (Value::Ebool(_a), Value::Uint8(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint8(b)) => Value::Euint8(a $op_token b),
                        (Value::Euint16(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint8(b)) => Value::Euint8(FheUint8::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint16(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint16(b)) => Value::Euint16(a $op_token b),
                        (Value::Euint32(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint32(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint32(b)) => Value::Euint32(a $op_token b),
                        (Value::Euint64(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint64(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint64(b)) => Value::Euint64(a $op_token b),
                        (Value::Euint128(a), Value::Uint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),

                        (Value::Ebool(_a), Value::Uint128(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint64(a), Value::Uint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),
                        (Value::Euint128(a), Value::Uint128(b)) => Value::Euint128(a $op_token b),

                        // Encrypted, Encrypted
                        (Value::Ebool(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint8(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint16(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint32(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint64(_b)) => unimplemented!(),
                        (Value::Ebool(_a), Value::Euint128(_b)) => unimplemented!(),

                        (Value::Euint8(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint8(a), Value::Euint8(b)) => Value::Euint8(a $op_token b),
                        (Value::Euint8(a), Value::Euint16(b)) => Value::Euint16(FheUint16::cast_from(a) $op_token b),
                        (Value::Euint8(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint8(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint8(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint16(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint16(a), Value::Euint8(b)) => Value::Euint16(a $op_token FheUint16::cast_from(b)),
                        (Value::Euint16(a), Value::Euint16(b)) => Value::Euint16(a $op_token b),
                        (Value::Euint16(a), Value::Euint32(b)) => Value::Euint32(FheUint32::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint16(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint32(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint32(a), Value::Euint8(b)) => Value::Euint32(a $op_token FheUint32::cast_from(b)),
                        (Value::Euint32(a), Value::Euint16(b)) => Value::Euint32(a $op_token FheUint32::cast_from(b)),
                        (Value::Euint32(a), Value::Euint32(b)) => Value::Euint32(a $op_token b),
                        (Value::Euint32(a), Value::Euint64(b)) => Value::Euint64(FheUint64::cast_from(a) $op_token b),
                        (Value::Euint32(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint64(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint64(a), Value::Euint8(b)) => Value::Euint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Euint64(a), Value::Euint16(b)) => Value::Euint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Euint64(a), Value::Euint32(b)) => Value::Euint64(a $op_token FheUint64::cast_from(b)),
                        (Value::Euint64(a), Value::Euint64(b)) => Value::Euint64(a $op_token b),
                        (Value::Euint64(a), Value::Euint128(b)) => Value::Euint128(FheUint128::cast_from(a) $op_token b),

                        (Value::Euint128(_a), Value::Ebool(_b)) => unimplemented!(),
                        (Value::Euint128(a), Value::Euint8(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint16(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint32(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint64(b)) => Value::Euint128(a $op_token FheUint128::cast_from(b)),
                        (Value::Euint128(a), Value::Euint128(b)) => Value::Euint128(a $op_token b),

                    }
                }
            )*
        }
    };
}

binary_shift_op! {
    shr, shr_method, >>, <<;
    shl, shl_method, <<, <<
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    Ebool(FheBool),
    Euint8(FheUint8),
    Euint16(FheUint16),
    Euint32(FheUint32),
    Euint64(FheUint64),
    Euint128(FheUint128),

    Bool(bool),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
}

impl FheEq<Value> for Value {
    fn eq(&self, other: Self) -> FheBool {
        match (self, other) {
            (Value::Ebool(a), Value::Ebool(b)) => a.eq(b),
            (Value::Euint8(a), Value::Euint8(b)) => a.eq(b),
            (Value::Euint16(a), Value::Euint16(b)) => a.eq(b),
            (Value::Euint32(a), Value::Euint32(b)) => a.eq(b),
            (Value::Euint64(a), Value::Euint64(b)) => a.eq(b),
            (Value::Euint128(a), Value::Euint128(b)) => a.eq(b),
            _ => unimplemented!(),
        }
    }

    fn ne(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Ebool(a), Value::Ebool(b)) => a.ne(b),
            (Value::Euint8(a), Value::Euint8(b)) => a.ne(b),
            (Value::Euint16(a), Value::Euint16(b)) => a.ne(b),
            (Value::Euint32(a), Value::Euint32(b)) => a.ne(b),
            (Value::Euint64(a), Value::Euint64(b)) => a.ne(b),
            (Value::Euint128(a), Value::Euint128(b)) => a.ne(b),
            _ => unimplemented!(),
        }
    }
}

impl FheMax<Value> for Value {
    type Output = Value;
    fn max(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Euint8(a), Value::Euint8(b)) => Value::Euint8(a.max(&b)),
            (Value::Euint16(a), Value::Euint16(b)) => Value::Euint16(a.max(&b)),
            (Value::Euint32(a), Value::Euint32(b)) => Value::Euint32(a.max(&b)),
            (Value::Euint64(a), Value::Euint64(b)) => Value::Euint64(a.max(&b)),
            (Value::Euint128(a), Value::Euint128(b)) => Value::Euint128(a.max(&b)),
            _ => unimplemented!(),
        }
    }
}

impl FheMin<Value> for Value {
    type Output = Value;
    fn min(&self, other: Value) -> Value {
        match (self, other) {
            (Value::Euint8(a), Value::Euint8(b)) => Value::Euint8(a.min(&b)),
            (Value::Euint16(a), Value::Euint16(b)) => Value::Euint16(a.min(&b)),
            (Value::Euint32(a), Value::Euint32(b)) => Value::Euint32(a.min(&b)),
            (Value::Euint64(a), Value::Euint64(b)) => Value::Euint64(a.min(&b)),
            (Value::Euint128(a), Value::Euint128(b)) => Value::Euint128(a.min(&b)),
            _ => unimplemented!(),
        }
    }
}

impl FheOrd<Value> for Value {
    fn lt(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Euint8(a), Value::Euint8(b)) => a.lt(b),
            (Value::Euint16(a), Value::Euint16(b)) => a.lt(b),
            (Value::Euint32(a), Value::Euint32(b)) => a.lt(b),
            (Value::Euint64(a), Value::Euint64(b)) => a.lt(b),
            (Value::Euint128(a), Value::Euint128(b)) => a.lt(b),
            _ => unimplemented!(),
        }
    }

    fn le(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Euint8(a), Value::Euint8(b)) => a.le(b),
            (Value::Euint16(a), Value::Euint16(b)) => a.le(b),
            (Value::Euint32(a), Value::Euint32(b)) => a.le(b),
            (Value::Euint64(a), Value::Euint64(b)) => a.le(b),
            (Value::Euint128(a), Value::Euint128(b)) => a.le(b),
            _ => unimplemented!(),
        }
    }

    fn gt(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Euint8(a), Value::Euint8(b)) => a.gt(b),
            (Value::Euint16(a), Value::Euint16(b)) => a.gt(b),
            (Value::Euint32(a), Value::Euint32(b)) => a.gt(b),
            (Value::Euint64(a), Value::Euint64(b)) => a.gt(b),
            (Value::Euint128(a), Value::Euint128(b)) => a.gt(b),
            _ => unimplemented!(),
        }
    }

    fn ge(&self, other: Value) -> FheBool {
        match (self, other) {
            (Value::Euint8(a), Value::Euint8(b)) => a.ge(b),
            (Value::Euint16(a), Value::Euint16(b)) => a.ge(b),
            (Value::Euint32(a), Value::Euint32(b)) => a.ge(b),
            (Value::Euint64(a), Value::Euint64(b)) => a.ge(b),
            (Value::Euint128(a), Value::Euint128(b)) => a.ge(b),
            _ => unimplemented!(),
        }
    }
}

impl Value {
    pub fn as_ebool(&self) -> Option<FheBool> {
        match self {
            Value::Ebool(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn as_eint8(&self) -> Option<&FheUint8> {
        match self {
            Value::Euint8(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_eint16(&self) -> Option<&FheUint16> {
        match self {
            Value::Euint16(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_eint32(&self) -> Option<&FheUint32> {
        match self {
            Value::Euint32(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_eint64(&self) -> Option<&FheUint64> {
        match self {
            Value::Euint64(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_eint128(&self) -> Option<&FheUint128> {
        match self {
            Value::Euint128(value) => Some(value),
            _ => None,
        }
    }
}

pub trait Neg {
    fn neg(&self) -> Self;
}

impl Neg for Value {
    fn neg(&self) -> Self {
        match self {
            Value::Ebool(val) => Value::Ebool(!val),
            Value::Euint8(val) => Value::Euint8(-val),
            Value::Euint16(val) => Value::Euint16(-val),
            Value::Euint32(val) => Value::Euint32(-val),
            Value::Euint64(val) => Value::Euint64(-val),
            Value::Euint128(val) => Value::Euint128(-val),
            Value::Bool(val) => Value::Bool(!val),
            Value::Uint8(val) => Value::Uint8(1 - val),
            Value::Uint16(val) => Value::Uint16(1 - val),
            Value::Uint32(val) => Value::Uint32(1 - val),
            Value::Uint64(val) => Value::Uint64(1 - val),
            Value::Uint128(val) => Value::Uint128(1 - val),
        }
    }
}

impl Value {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Ebool(val) => {
                let mut bytes = vec![0];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Euint8(val) => {
                let mut bytes = vec![1];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Euint16(val) => {
                let mut bytes = vec![2];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Euint32(val) => {
                let mut bytes = vec![3];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Euint64(val) => {
                let mut bytes = vec![4];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Euint128(val) => {
                let mut bytes = vec![5];
                bytes.extend(bincode::serialize(val).unwrap());
                bytes
            }
            Value::Bool(val) => bincode::serialize(val).unwrap(),
            Value::Uint8(val) => bincode::serialize(val).unwrap(),
            Value::Uint16(val) => bincode::serialize(val).unwrap(),
            Value::Uint32(val) => bincode::serialize(val).unwrap(),
            Value::Uint64(val) => bincode::serialize(val).unwrap(),
            Value::Uint128(val) => bincode::serialize(val).unwrap(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> (Self, usize) {
        // Returns Value and bytes consumed
        match bytes[0] {
            0 => {
                let val: FheBool = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Ebool(val), 1)
            }
            1 => {
                let val: FheUint8 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Euint8(val), bytes.len())
            }
            2 => {
                let val: FheUint16 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Euint16(val), bytes.len())
            }
            3 => {
                let val: FheUint32 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Euint32(val), bytes.len())
            }
            4 => {
                let val: FheUint64 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Euint64(val), bytes.len())
            }
            5 => {
                let val: FheUint128 = bincode::deserialize(&bytes[1..]).unwrap();
                (Value::Euint128(val), bytes.len())
            }
            // Handle other Value variants...
            _ => unimplemented!(),
        }
    }
}

impl Add<u8> for Value {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        match self {
            Value::Ebool(_val) => unimplemented!(),
            Value::Euint8(val) => Value::Euint8(val + other),
            Value::Euint16(val) => Value::Euint16(val + other as u16),
            Value::Euint32(val) => Value::Euint32(val + other as u32),
            Value::Euint64(val) => Value::Euint64(val + other as u64),
            Value::Euint128(val) => Value::Euint128(val + other as u128),
            Value::Bool(_val) => unimplemented!(),
            Value::Uint8(val) => Value::Uint8(val + other),
            Value::Uint16(val) => Value::Uint16(val + other as u16),
            Value::Uint32(val) => Value::Uint32(val + other as u32),
            Value::Uint64(val) => Value::Uint64(val + other as u64),
            Value::Uint128(val) => Value::Uint128(val + other as u128),
        }
    }
}

impl Sub<u8> for Value {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        match self {
            Value::Ebool(_val) => unimplemented!(),
            Value::Euint8(val) => Value::Euint8(val - other),
            Value::Euint16(val) => Value::Euint16(val - other as u16),
            Value::Euint32(val) => Value::Euint32(val - other as u32),
            Value::Euint64(val) => Value::Euint64(val - other as u64),
            Value::Euint128(val) => Value::Euint128(val - other as u128),
            Value::Bool(_val) => unimplemented!(),
            Value::Uint8(val) => Value::Uint8(val - other),
            Value::Uint16(val) => Value::Uint16(val - other as u16),
            Value::Uint32(val) => Value::Uint32(val - other as u32),
            Value::Uint64(val) => Value::Uint64(val - other as u64),
            Value::Uint128(val) => Value::Uint128(val - other as u128),
        }
    }
}
