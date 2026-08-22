use std::{
    io::{self, ErrorKind},
    mem::size_of,
};

use rmp::Marker;

#[inline]
pub fn skip_value(input: &[u8]) -> io::Result<usize> {
    skip_values_at(input, 0, 1)
}

#[inline]
pub fn skip_values(input: &[u8], count: u32) -> io::Result<usize> {
    skip_values_at(input, 0, count)
}

#[inline]
fn skip_values_at(input: &[u8], mut offset: usize, count: u32) -> io::Result<usize> {
    let mut remaining = count;

    while remaining != 0 {
        let marker = Marker::from_u8(read_u8(input, offset)?);
        offset += size_of::<u8>();
        remaining -= 1;

        remaining += match marker {
            Marker::FixPos(_) | Marker::FixNeg(_) | Marker::Null | Marker::False | Marker::True => {
                continue;
            }
            Marker::FixArray(len) => len as u32,
            Marker::FixMap(len) => len as u32 * 2,
            Marker::FixStr(len) => {
                offset = skip_bytes(input, offset, len as u32)?;
                continue;
            }
            marker => {
                let (next_offset, nested) = skip_value_slow_path(input, offset, marker)?;
                offset = next_offset;
                nested
            }
        };
    }

    Ok(offset)
}

#[inline(never)]
fn skip_value_slow_path(input: &[u8], offset: usize, marker: Marker) -> io::Result<(usize, u32)> {
    match marker {
        Marker::FixPos(_) | Marker::FixNeg(_) | Marker::Null | Marker::False | Marker::True => {
            Ok((offset, 0))
        }
        Marker::FixMap(len) => Ok((offset, len as u32 * 2)),
        Marker::FixArray(len) => Ok((offset, len as u32)),
        Marker::FixStr(len) => Ok((skip_bytes(input, offset, len as u32)?, 0)),
        Marker::Bin8 => {
            let len = read_u8(input, offset)? as u32;
            Ok((skip_bytes(input, offset + size_of::<u8>(), len)?, 0))
        }
        Marker::Bin16 => {
            let len = read_u16(input, offset)? as u32;
            Ok((skip_bytes(input, offset + size_of::<u16>(), len)?, 0))
        }
        Marker::Bin32 => {
            let len = read_u32(input, offset)?;
            Ok((skip_bytes(input, offset + size_of::<u32>(), len)?, 0))
        }
        Marker::Ext8 => {
            let len = read_u8(input, offset)? as u32;
            Ok((skip_ext_payload(input, offset + size_of::<u8>(), len)?, 0))
        }
        Marker::Ext16 => {
            let len = read_u16(input, offset)? as u32;
            Ok((skip_ext_payload(input, offset + size_of::<u16>(), len)?, 0))
        }
        Marker::Ext32 => {
            let len = read_u32(input, offset)?;
            Ok((skip_ext_payload(input, offset + size_of::<u32>(), len)?, 0))
        }
        Marker::F32 => Ok((skip_bytes(input, offset, size_of::<f32>() as u32)?, 0)),
        Marker::F64 => Ok((skip_bytes(input, offset, size_of::<f64>() as u32)?, 0)),
        Marker::U8 | Marker::I8 => Ok((skip_bytes(input, offset, size_of::<u8>() as u32)?, 0)),
        Marker::U16 | Marker::I16 => Ok((skip_bytes(input, offset, size_of::<u16>() as u32)?, 0)),
        Marker::U32 | Marker::I32 => Ok((skip_bytes(input, offset, size_of::<u32>() as u32)?, 0)),
        Marker::U64 | Marker::I64 => Ok((skip_bytes(input, offset, size_of::<u64>() as u32)?, 0)),
        Marker::FixExt1 => Ok((skip_ext_payload(input, offset, 1)?, 0)),
        Marker::FixExt2 => Ok((skip_ext_payload(input, offset, 2)?, 0)),
        Marker::FixExt4 => Ok((skip_ext_payload(input, offset, 4)?, 0)),
        Marker::FixExt8 => Ok((skip_ext_payload(input, offset, 8)?, 0)),
        Marker::FixExt16 => Ok((skip_ext_payload(input, offset, 16)?, 0)),
        Marker::Str8 => {
            let len = read_u8(input, offset)? as u32;
            Ok((skip_bytes(input, offset + size_of::<u8>(), len)?, 0))
        }
        Marker::Str16 => {
            let len = read_u16(input, offset)? as u32;
            Ok((skip_bytes(input, offset + size_of::<u16>(), len)?, 0))
        }
        Marker::Str32 => {
            let len = read_u32(input, offset)?;
            Ok((skip_bytes(input, offset + size_of::<u32>(), len)?, 0))
        }
        Marker::Array16 => {
            let len = read_u16(input, offset)? as u32;
            Ok((offset + size_of::<u16>(), len))
        }
        Marker::Array32 => {
            let len = read_u32(input, offset)?;
            Ok((offset + size_of::<u32>(), len))
        }
        Marker::Map16 => {
            let len = read_u16(input, offset)? as u32;
            Ok((offset + size_of::<u16>(), len * 2))
        }
        Marker::Map32 => {
            let len = read_u32(input, offset)?;
            Ok((offset + size_of::<u32>(), len * 2))
        }
        Marker::Reserved => Err(ErrorKind::InvalidData.into()),
    }
}

#[inline]
fn skip_ext_payload(input: &[u8], offset: usize, data_len: u32) -> io::Result<usize> {
    let offset = skip_bytes(input, offset, 1)?;
    skip_bytes(input, offset, data_len)
}

#[inline]
fn skip_bytes(input: &[u8], offset: usize, len: u32) -> io::Result<usize> {
    let end = offset + len as usize;
    if end > input.len() {
        return Err(ErrorKind::UnexpectedEof.into());
    }
    Ok(end)
}

#[inline]
fn read_u8(input: &[u8], offset: usize) -> io::Result<u8> {
    if let Some(value) = input.get(offset).copied() {
        Ok(value)
    } else {
        Err(ErrorKind::UnexpectedEof.into())
    }
}

#[inline]
fn read_u16(input: &[u8], offset: usize) -> io::Result<u16> {
    let end = skip_bytes(input, offset, size_of::<u16>() as u32)?;
    Ok(u16::from_be_bytes(input[offset..end].try_into().unwrap()))
}

#[inline]
fn read_u32(input: &[u8], offset: usize) -> io::Result<u32> {
    let end = skip_bytes(input, offset, size_of::<u32>() as u32)?;
    Ok(u32::from_be_bytes(input[offset..end].try_into().unwrap()))
}

#[cfg(test)]
mod tests {
    use rmpv::{ValueRef, encode::write_value_ref};

    use super::*;

    fn encode(value: ValueRef<'_>) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_value_ref(&mut bytes, &value).unwrap();
        bytes
    }

    fn fixed_payload(marker: Marker, len: usize) -> Vec<u8> {
        let mut bytes = vec![marker.to_u8()];
        bytes.resize(len + 1, 0);
        bytes
    }

    fn fixed_ext_payload(marker: Marker, len: usize) -> Vec<u8> {
        let mut bytes = vec![marker.to_u8(), 1];
        bytes.resize(len + 2, 0);
        bytes
    }

    fn len8_payload(marker: Marker, is_ext: bool) -> Vec<u8> {
        let mut bytes = vec![marker.to_u8(), 1];
        if is_ext {
            bytes.push(1);
        }
        bytes.push(0);
        bytes
    }

    fn len16_payload(marker: Marker, is_ext: bool) -> Vec<u8> {
        let mut bytes = vec![marker.to_u8()];
        bytes.extend_from_slice(&1_u16.to_be_bytes());
        if is_ext {
            bytes.push(1);
        }
        bytes.push(0);
        bytes
    }

    fn len32_payload(marker: Marker, is_ext: bool) -> Vec<u8> {
        let mut bytes = vec![marker.to_u8()];
        bytes.extend_from_slice(&1_u32.to_be_bytes());
        if is_ext {
            bytes.push(1);
        }
        bytes.push(0);
        bytes
    }

    #[test]
    fn skips_one_value_without_including_trailing_data() {
        let mut bytes = encode(ValueRef::from("value"));
        let value_len = bytes.len();
        bytes.extend(encode(ValueRef::from("trailing")));

        assert_eq!(skip_value(&bytes).unwrap(), value_len);
    }

    #[test]
    fn skips_requested_number_of_values() {
        let mut bytes = encode(ValueRef::from(1));
        bytes.extend(encode(ValueRef::Map(vec![(
            ValueRef::from("nested"),
            ValueRef::Nil,
        )])));
        bytes.extend(encode(ValueRef::Boolean(false)));
        let values_len = bytes.len();
        bytes.extend(encode(ValueRef::from("trailing")));

        assert_eq!(skip_values(&bytes, 3).unwrap(), values_len);
    }

    #[test]
    fn skips_empty_containers_and_nested_single_element_arrays() {
        #[track_caller]
        fn check(bytes: Vec<u8>) {
            assert_eq!(skip_value(&bytes).unwrap(), bytes.len());
        }

        check(encode(ValueRef::Array(vec![])));
        check(encode(ValueRef::Map(vec![])));
        check(encode(ValueRef::Array(vec![ValueRef::Array(vec![
            "value".into(),
        ])])));
        check(fixed_payload(Marker::F32, size_of::<f32>()));
        check(fixed_payload(Marker::F64, size_of::<f64>()));
        check(fixed_payload(Marker::U8, size_of::<u8>()));
        check(fixed_payload(Marker::I8, size_of::<i8>()));
        check(fixed_payload(Marker::U16, size_of::<u16>()));
        check(fixed_payload(Marker::I16, size_of::<i16>()));
        check(fixed_payload(Marker::U32, size_of::<u32>()));
        check(fixed_payload(Marker::I32, size_of::<i32>()));
        check(fixed_payload(Marker::U64, size_of::<u64>()));
        check(fixed_payload(Marker::I64, size_of::<i64>()));
        check(fixed_ext_payload(Marker::FixExt1, 1));
        check(fixed_ext_payload(Marker::FixExt2, 2));
        check(fixed_ext_payload(Marker::FixExt4, 4));
        check(fixed_ext_payload(Marker::FixExt8, 8));
        check(fixed_ext_payload(Marker::FixExt16, 16));
        check(len8_payload(Marker::Bin8, false));
        check(len16_payload(Marker::Bin16, false));
        check(len32_payload(Marker::Bin32, false));
        check(len8_payload(Marker::Ext8, true));
        check(len16_payload(Marker::Ext16, true));
        check(len32_payload(Marker::Ext32, true));
        check(len8_payload(Marker::Str8, false));
        check(len16_payload(Marker::Str16, false));
        check(len32_payload(Marker::Str32, false));
        check({
            let mut bytes = vec![Marker::Array16.to_u8()];
            bytes.extend_from_slice(&2_u16.to_be_bytes());
            bytes.extend([Marker::Null.to_u8(), Marker::True.to_u8()]);
            bytes
        });
        check({
            let mut bytes = vec![Marker::Array32.to_u8()];
            bytes.extend_from_slice(&2_u32.to_be_bytes());
            bytes.extend([Marker::Null.to_u8(), Marker::True.to_u8()]);
            bytes
        });
        check({
            let mut bytes = vec![Marker::Map16.to_u8()];
            bytes.extend_from_slice(&1_u16.to_be_bytes());
            bytes.extend([Marker::Null.to_u8(), Marker::True.to_u8()]);
            bytes
        });
        check({
            let mut bytes = vec![Marker::Map32.to_u8()];
            bytes.extend_from_slice(&1_u32.to_be_bytes());
            bytes.extend([Marker::Null.to_u8(), Marker::True.to_u8()]);
            bytes
        });
    }

    #[test]
    fn skipping_no_values_consumes_nothing() {
        assert_eq!(skip_values(&[], 0).unwrap(), 0);
    }

    #[test]
    fn skips_deeply_nested_values_without_recursion() {
        const DEPTH: usize = 100_000;

        let mut bytes = Vec::with_capacity(DEPTH * 3 + 1);
        for _ in 0..DEPTH {
            bytes.push(Marker::Array16.to_u8());
            bytes.extend_from_slice(&1_u16.to_be_bytes());
        }
        bytes.push(Marker::Null.to_u8());

        assert_eq!(skip_value(&bytes).unwrap(), bytes.len());
    }

    #[test]
    fn reports_incomplete_values() {
        let mut bytes = encode(ValueRef::Array(vec![
            ValueRef::from("complete"),
            ValueRef::from("truncated"),
        ]));
        bytes.pop();

        assert_eq!(
            skip_value(&bytes).unwrap_err().kind(),
            ErrorKind::UnexpectedEof
        );

        let mut bytes = encode(ValueRef::from(1));
        bytes.push(Marker::Str8.to_u8());

        assert_eq!(
            skip_values(&bytes, 2).unwrap_err().kind(),
            ErrorKind::UnexpectedEof
        );
    }

    #[test]
    fn rejects_reserved_marker() {
        assert_eq!(
            skip_value(&[Marker::Reserved.to_u8()]).unwrap_err().kind(),
            ErrorKind::InvalidData
        );
    }
}
