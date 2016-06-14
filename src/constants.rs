#![allow(dead_code)]
pub const REC_TYPE_HEADER: u8 = 0x00;
pub const REC_TYPE_BGNLIB: u8 = 0x01;
pub const REC_TYPE_LIBNAME: u8 = 0x02;
pub const REC_TYPE_UNITS: u8 = 0x03;
pub const REC_TYPE_ENDLIB: u8 = 0x04;
pub const REC_TYPE_BGNSTR: u8 = 0x05;
pub const REC_TYPE_STRNAME: u8 = 0x06;
pub const REC_TYPE_ENDSTR: u8 = 0x07;
pub const REC_TYPE_BOUNDARY: u8 = 0x08;
pub const REC_TYPE_PATH: u8 = 0x09;
pub const REC_TYPE_SREF: u8 = 0x0A;
pub const REC_TYPE_AREF: u8 = 0x0B;
pub const REC_TYPE_TEXT: u8 = 0x0C;
pub const REC_TYPE_LAYER: u8 = 0x0D;
pub const REC_TYPE_DATATYPE: u8 = 0x0E;
pub const REC_TYPE_WIDTH: u8 = 0x0F;
pub const REC_TYPE_XY: u8 = 0x10;
pub const REC_TYPE_ENDEL: u8 = 0x11;
pub const REC_TYPE_SNAME: u8 = 0x12;
pub const REC_TYPE_COLROW: u8 = 0x13;
pub const REC_TYPE_NODE: u8 = 0x15;
pub const REC_TYPE_TEXTTYPE: u8 = 0x16;
pub const REC_TYPE_PRESENTATION: u8 = 0x17;
pub const REC_TYPE_STRING: u8 = 0x19;
pub const REC_TYPE_STRANS: u8 = 0x1A;
pub const REC_TYPE_MAG: u8 = 0x1B;
pub const REC_TYPE_ANGLE: u8 = 0x1C;
pub const REC_TYPE_PATHTYPE: u8 = 0x21;
pub const REC_TYPE_BOX: u8 = 0x2D;
pub const REC_TYPE_EFLAGS: u8 = 0x26;
pub const REC_TYPE_NODETYPE: u8 = 0x2A;
pub const REC_TYPE_BGNEXTN: u8 = 0x30;

pub const DATA_TYPE_NONE: u8 = 0x00;
pub const DATA_TYPE_BIT: u8 = 0x01;
pub const DATA_TYPE_INT16: u8 = 0x02;
pub const DATA_TYPE_INT32: u8 = 0x03;
pub const DATA_TYPE_REAL32: u8 = 0x04;
pub const DATA_TYPE_REAL64: u8 = 0x05;
pub const DATA_TYPE_STR: u8 = 0x06;

pub const MAX_DATA_SIZE: usize = 8;

pub fn data_size(t: u8) -> usize {
    match t {
        x if x == DATA_TYPE_NONE => 0,
        x if x == DATA_TYPE_BIT => 2,
        x if x == DATA_TYPE_INT16 => 2,
        x if x == DATA_TYPE_INT32 => 4,
        x if x == DATA_TYPE_REAL32 => 4,
        x if x == DATA_TYPE_REAL64 => 8,
        _ => 0
    }
}
