extern crate byteorder;

use byteorder::{ByteOrder, BigEndian};

pub fn bytes_to_gds_real32(bytes: &[u8]) -> f32 {
    let exp: i8 = ((0b01111111 & bytes[0]) as i8) - 64 - 6;
    let mut man_arr = [0;4];
    man_arr[1..].copy_from_slice(&bytes[1..]);
    let man_arr = man_arr;
    let man: f32 = BigEndian::read_u32(&man_arr) as f32;
    let base: f32 = 16.;
    if (0b10000000 & bytes[0]) == 0{
        man*base.powi(exp as i32)
    } else {
        -man*base.powi(exp as i32)
    }
}

pub fn bytes_to_gds_real(bytes: &[u8]) -> f64 {
    let exp: i8 = ((0b01111111 & bytes[0]) as i8) - 64 - 14;
    let mut man_arr = [0;8];
    man_arr[1..].copy_from_slice(&bytes[1..]);
    let man: f64 = BigEndian::read_u64(&man_arr) as f64;
    let base: f64 = 16.;
    if (0b10000000 & bytes[0]) == 0{
        man*base.powi(exp as i32)
    } else {
        -man*base.powi(exp as i32)
    }
}

pub fn gds_real_to_bytes(r: f64) -> [u8;8] {
    let mut exp: u8 = 64;
    let mut man: f64 = r.abs();
    let base: f64 = 16.;
    if man != 0. {
        while man > 1. {
            man /= 16.;
            exp += 1;
        }
        while man < 1./16. {
            man *= 16.;
            exp -= 1;
        }
    }
    let man: u64 = (man*base.powi(14)) as u64;
    let mut man_arr = [0;8];
    BigEndian::write_u64(&mut man_arr,man);
    if r < 0. {
        exp |= 0b10000000;
    } else {
        exp &= 0b01111111;
    }
    let mut out_arr = [0;8];
    out_arr[0] = exp;
    out_arr[1..].copy_from_slice(&man_arr[1..8]);
    out_arr
}

pub fn gds_real_32_to_bytes(r: f32) -> [u8;4] {
    let mut exp: u8 = 64;
    let mut man: f32 = r.abs();
    let base: f32 = 16.;
    if man != 0. {
        while man > 1. {
            man /= 16.;
            exp += 1;
        }
        while man < 1./16. {
            man *= 16.;
            exp -= 1;
        }
    }
    let man: u32 = (man*base.powi(6)) as u32;
    let mut man_arr = [0;4];
    BigEndian::write_u32(&mut man_arr,man);
    if r < 0. {
        exp |= 0b10000000;
    } else {
        exp &= 0b01111111;
    }
    let mut out_arr = [0;4];
    out_arr[0] = exp;
    out_arr[1..].copy_from_slice(&man_arr[1..4]);
    out_arr
}

pub fn i16_to_vec(i: i16) -> Vec<u8> {
    let mut buf: [u8;2] = [0;2];
    BigEndian::write_i16(&mut buf,i);
    buf.to_vec()
}

pub fn u16_to_vec(i: u16) -> Vec<u8> {
    let mut buf: [u8;2] = [0;2];
    BigEndian::write_u16(&mut buf,i);
    buf.to_vec()
}

pub fn i32_to_vec(i: i32) -> Vec<u8> {
    let mut buf: [u8;4] = [0;4];
    BigEndian::write_i32(&mut buf,i);
    buf.to_vec()
}

pub fn u32_to_vec(i: u32) -> Vec<u8> {
    let mut buf: [u8;4] = [0;4];
    BigEndian::write_u32(&mut buf,i);
    buf.to_vec()
}
