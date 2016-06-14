/// Library for handling GDSII files

extern crate byteorder;

pub mod constants;
pub mod utils;

use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use byteorder::{ByteOrder, BigEndian};

#[derive(Clone)]
pub struct Library {
    pub version: i16,
    pub name: String,
    pub date_mod: Date,
    pub date_acc: Date,
    pub units_user: f64,
    pub units_m: f64,
    pub structures: Vec<Structure>
}

#[derive(Debug,Clone)]
pub struct Date {
    pub year: i16,
    pub month: i16,
    pub day: i16,
    pub hour: i16,
    pub minute: i16,
    pub second: i16
}

#[derive(Debug,Clone)]
pub struct Structure {
    pub name: String,
    pub date_mod: Date,
    pub date_acc: Date,
    pub elements: Vec<Element>
}

#[derive(Debug,Clone)]
pub struct Element {
    pub element_type: ElementType,
    pub parameters: Vec<ElementParameter>
}

#[derive(Debug,Clone)]
pub enum ElementType {
    None,
    Boundary,
    Path,
    StructureRef,
    ArrayRef,
    Text,
    Node,
    Box
}

#[derive(Debug,Clone)]
pub enum ElementParameter {
    Layer(i16),
    XY(Vec<(i32,i32)>),
    Datatype(i16),
    Width(i32),
    StructureName(String),
    ColRow(Vec<i16>),
    TextType(i16),
    Presentation(u16),
    String(String),
    StrTransf(u16),
    Magnification(f64),
    Angle(f64),
    Pathtype(i16),
    EFlags(u16),
    Nodetype(i16),
    BeginExt(i32)
    //TODO more parameters
}

#[derive(Debug)]
pub struct Record {
    pub size: u16,
    pub rec_type: u8,
    pub data_type: u8,
    pub data: Vec<RecordData>
}

#[derive(Debug,Clone)]
pub enum RecordData {
    None,
    Bit(u16),
    Int16(i16),
    Int32(i32),
    Real32(f32),
    Real64(f64),
    Str(String)
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}/{:02}/{:02} {:02}:{:02}:{:02}", self.year, self.month,
                self.day, self.hour, self.minute, self.second)
    }
}

impl fmt::Display for Library {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Library {} (version {}), modified {} / accessed {}",
            self.name, self.version, self.date_mod, self.date_acc)
    }
}

impl Date {
    pub fn new() -> Date {
        Date{year: 1970, month: 1, day: 1, hour: 0, minute: 0, second: 0}
    }
}

impl Library {
    pub fn new(v: i16, n: String) -> Library{
        Library{version: v, name: n, date_mod: Date::new(),
            date_acc: Date::new(), units_user: 0., units_m: 0.,
            structures: Vec::new()}
    }

    pub fn read(s: &str) -> Library {
        let mut file = File::open(s).unwrap();
        let mut version = 0;
        let mut name: String = String::from("");
        let mut date_mod = Date::new();
        let mut date_acc = Date::new();
        let mut units_user: f64 = 0.;
        let mut units_m: f64 = 0.;
        let mut structures: Vec<Structure> = Vec::new();
        let mut stru = Structure::new();
        let mut elem = Element::new();

        loop {
            let rec = Record::read(&mut file);
            if rec.rec_type == constants::REC_TYPE_ENDLIB {
                break;
            } else if rec.rec_type == constants::REC_TYPE_BGNLIB {
                let mut d_data = [0; 12];
                for i in 0..12 {
                    d_data[i] = match rec.data.get(i) {
                        Some(&RecordData::Int16(x)) => x,
                        _ => 0
                    };
                }
                date_mod = Date{year: d_data[0], month: d_data[1],
                    day: d_data[2], hour: d_data[3], minute: d_data[4],
                    second: d_data[5]};
                date_acc = Date{year: d_data[6], month: d_data[7],
                    day: d_data[8], hour: d_data[9], minute: d_data[10],
                    second: d_data[11]};
            } else if rec.rec_type == constants::REC_TYPE_HEADER {
                version = match rec.data.get(0) {
                    Some(&RecordData::Int16(x)) => x,
                    _ => 0
                };
            } else if rec.rec_type == constants::REC_TYPE_LIBNAME {
                name = match rec.data.get(0) {
                    Some(&RecordData::Str(ref x)) => x.clone(),
                    _ => String::from("")
                };
            } else if rec.rec_type == constants::REC_TYPE_UNITS {
                units_user = match rec.data.get(0) {
                    Some(&RecordData::Real64(x)) => x,
                    _ => 0.
                };
                units_m = match rec.data.get(1) {
                    Some(&RecordData::Real64(x)) => x,
                    _ => 0.
                };
            } else if rec.rec_type == constants::REC_TYPE_BGNSTR {
                let mut d_data = [0; 12];
                for i in 0..12 {
                    d_data[i] = match rec.data.get(i) {
                        Some(&RecordData::Int16(x)) => x,
                        _ => 0
                    };
                }
                stru.date_mod = Date{year: d_data[0],
                    month: d_data[1], day: d_data[2], hour: d_data[3],
                    minute: d_data[4], second: d_data[5]};
                stru.date_acc = Date{year: d_data[6],
                    month: d_data[7], day: d_data[8], hour: d_data[9],
                    minute: d_data[10], second: d_data[11]};
            } else if rec.rec_type == constants::REC_TYPE_ENDSTR {
                structures.push(stru);
                stru = Structure::new();
            } else if rec.rec_type == constants::REC_TYPE_STRNAME {
                let str_name = match rec.data.get(0) {
                    Some(&RecordData::Str(ref x)) => x.clone(),
                    _ => String::from("")
                };
                stru.name = str_name;
            } else if rec.rec_type == constants::REC_TYPE_BOUNDARY {
                elem.element_type = ElementType::Boundary;
            } else if rec.rec_type == constants::REC_TYPE_PATH {
                elem.element_type = ElementType::Path;
            } else if rec.rec_type == constants::REC_TYPE_SREF {
                elem.element_type = ElementType::StructureRef;
            } else if rec.rec_type == constants::REC_TYPE_AREF {
                elem.element_type = ElementType::ArrayRef;
            } else if rec.rec_type == constants::REC_TYPE_TEXT {
                elem.element_type = ElementType::Text;
            } else if rec.rec_type == constants::REC_TYPE_NODE {
                elem.element_type = ElementType::Node;
            } else if rec.rec_type == constants::REC_TYPE_BOX {
                elem.element_type = ElementType::Box;
            } else if rec.rec_type == constants::REC_TYPE_LAYER {
                match rec.data.get(0) {
                    Some(&RecordData::Int16(x)) =>
                        elem.parameters.push(ElementParameter::Layer(x)),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_XY {
                let mut c = 0;
                let mut xy_vec: Vec<(i32,i32)> = Vec::new();
                while c < (rec.data.len() - 1) {
                    let mut x_coord: i32 = 0;
                    let mut y_coord: i32 = 0;
                    match rec.data.get(c) {
                        Some(&RecordData::Int32(x)) => x_coord = x,
                        _ => {}
                    };
                    match rec.data.get(c+1) {
                        Some(&RecordData::Int32(x)) => y_coord = x,
                        _ => {}
                    };
                    c += 2;
                    xy_vec.push((x_coord,y_coord));
                }
                elem.parameters.push(ElementParameter::XY(xy_vec));
            } else if rec.rec_type == constants::REC_TYPE_DATATYPE {
                match rec.data.get(0) {
                    Some(&RecordData::Int16(x)) => elem.parameters.push(
                        ElementParameter::Datatype(x)),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_WIDTH {
                match rec.data.get(0) {
                    Some(&RecordData::Int32(x)) => elem.parameters.push(
                        ElementParameter::Width(x)),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_SNAME {
                match rec.data.get(0) {
                    Some(&RecordData::Str(ref x)) => elem.parameters.push(
                        ElementParameter::StructureName(x.clone())),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_COLROW {
                let mut c = 0;
                let mut cr_vec: Vec<i16> = Vec::new();
                while c < (rec.data.len() - 1) {
                    let mut colrow: i16 = 0;
                    match rec.data.get(c) {
                        Some(&RecordData::Int16(x)) => colrow = x,
                        _ => {}
                    };
                    cr_vec.push(colrow);
                    c += 1;
                }
                elem.parameters.push(ElementParameter::ColRow(cr_vec));
            } else if rec.rec_type == constants::REC_TYPE_TEXTTYPE {
                match rec.data.get(0) {
                    Some(&RecordData::Int16(x)) => elem.parameters.push(
                        ElementParameter::TextType(x)),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_PRESENTATION {
                match rec.data.get(0) {
                    Some(&RecordData::Bit(x)) => elem.parameters.push(
                        ElementParameter::Presentation(x)),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_STRING {
                match rec.data.get(0) {
                    Some(&RecordData::Str(ref x)) => elem.parameters.push(
                        ElementParameter::String(x.clone())),
                    _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_STRANS {
                match rec.data.get(0) {
                    Some(&RecordData::Bit(x)) => elem.parameters.push(
                        ElementParameter::StrTransf(x)),
                        _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_MAG {
                match rec.data.get(0) {
                    Some(&RecordData::Real64(x)) => elem.parameters.push(
                        ElementParameter::Magnification(x)),
                        _ => {}
                    };
            } else if rec.rec_type == constants::REC_TYPE_ANGLE {
                match rec.data.get(0) {
                    Some(&RecordData::Real64(x)) => elem.parameters.push(
                        ElementParameter::Angle(x)),
                        _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_PATHTYPE {
                match rec.data.get(0) {
                    Some(&RecordData::Int16(x)) => elem.parameters.push(
                        ElementParameter::Pathtype(x)),
                        _=> {}
                };
            } else if rec.rec_type == constants::REC_TYPE_EFLAGS {
                match rec.data.get(0) {
                    Some(&RecordData::Bit(x)) => elem.parameters.push(
                        ElementParameter::EFlags(x)),
                        _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_NODETYPE {
                match rec.data.get(0) {
                    Some(&RecordData::Int16(x)) => elem.parameters.push(
                        ElementParameter::Nodetype(x)),
                        _ => {}
                };
            } else if rec.rec_type == constants::REC_TYPE_BGNEXTN {
                match rec.data.get(0) {
                    Some(&RecordData::Int32(x)) => elem.parameters.push(
                        ElementParameter::BeginExt(x)),
                        _ => {}
                };
                // TODO other parameters
            } else if rec.rec_type == constants::REC_TYPE_ENDEL {
                stru.elements.push(elem);
                elem = Element::new();
            }
        }

        Library{version: version, name: name, date_mod: date_mod,
            date_acc: date_acc, units_user: units_user, units_m: units_m,
            structures: structures}
    }

    pub fn write(&self, s: &str) {
        println!("Writing to {}",s);
        let mut file = File::create(s).unwrap();
        let mut vec: Vec<Record> = Vec::new();

        // header
        vec.push(Record::new_single(constants::REC_TYPE_HEADER,
                           constants::DATA_TYPE_INT16,
                           RecordData::Int16(self.version)));
        let mut date_data = self.date_mod.to_record_data();
        date_data.extend(self.date_acc.to_record_data());
        vec.push(Record::new(constants::REC_TYPE_BGNLIB,
                           constants::DATA_TYPE_INT16,date_data));
        vec.push(Record::new_single(constants::REC_TYPE_LIBNAME,
                           constants::DATA_TYPE_STR,
                           RecordData::Str(self.name.clone())));
        let mut unit_data: Vec<f64> = Vec::new();
        unit_data.push(self.units_user);
        unit_data.push(self.units_m);
        let unit_data_rec = unit_data.iter().map(
            |x| RecordData::Real64(*x)).collect();
        vec.push(Record::new(constants::REC_TYPE_UNITS,
                             constants::DATA_TYPE_REAL64,
                             unit_data_rec));

        // structures
        for stru in &self.structures {
            let mut date_data = stru.date_mod.to_record_data();
            date_data.extend(stru.date_acc.to_record_data());
            vec.push(Record::new(constants::REC_TYPE_BGNSTR,
                            constants::DATA_TYPE_INT16,date_data));
            vec.push(Record::new_single(constants::REC_TYPE_STRNAME,
                            constants::DATA_TYPE_STR,
                            RecordData::Str(stru.name.clone())));
            // elements
            for elem in &stru.elements {
                vec.extend(elem.to_records());
            }
            vec.push(Record::new_none(constants::REC_TYPE_ENDSTR));
        }

        // tail
        vec.push(Record::new_none(constants::REC_TYPE_ENDLIB));

        // write file
        let _ = vec.iter().map(|x| x.write(&mut file)).collect::<Vec<_>>();
    }
}

impl Date {
    pub fn to_record_data(&self) -> Vec<RecordData> {
        let mut vec: Vec<RecordData> = Vec::new();
        vec.push(RecordData::Int16(self.year));
        vec.push(RecordData::Int16(self.month));
        vec.push(RecordData::Int16(self.day));
        vec.push(RecordData::Int16(self.hour));
        vec.push(RecordData::Int16(self.minute));
        vec.push(RecordData::Int16(self.second));
        vec
    }
}

impl Record {
    pub fn new(rec_type: u8, data_type: u8, data: Vec<RecordData>) -> Record {
        let size: u16 = 4+(constants::data_size(data_type)*data.len()) as u16;
        Record{size: size, rec_type: rec_type, data_type: data_type, data: data}
    }

    pub fn new_none(rec_type: u8) -> Record {
        let size: u16 = 4;
        let data = vec![RecordData::None];
        let data_type = constants::DATA_TYPE_NONE;
        Record{size: size, rec_type: rec_type, data_type: data_type, data: data}
    }

    pub fn new_single(rec_type: u8, data_type: u8, data: RecordData) -> Record {
        let mut rec = Record{size: 0, rec_type: rec_type,
            data_type: data_type, data: vec![data]};
        rec.update_size();
        rec
    }

    pub fn push_data(&mut self, data: RecordData) {
        self.data.push(data);
        self.update_size();
    }

    pub fn update_size(&mut self) {
        self.size = 4;
        if self.data_type == constants::DATA_TYPE_STR {
            for i in &self.data {
                match i {
                    &RecordData::Str(ref x) => self.size += x.len() as u16,
                    _ => {}
                }
            }
        } else {
            self.size += (constants::data_size(self.data_type)*self.data.len())
                as u16;
        }
    }

    pub fn read(file: &mut File) -> Record {
        let mut buffer = [0; 2];
        let _ = file.read(&mut buffer);
        let size = BigEndian::read_u16(&buffer);
        let _ = file.read(&mut buffer);
        let rec_type = buffer[0];
        let data_type = buffer[1];
        let mut data: Vec<RecordData> = Vec::new();
        let mut byte_counter: u16 = 4;
        let mut buffer = [0;1];

        if data_type == constants::DATA_TYPE_STR {
            let mut str_buf: Vec<u8> = Vec::new();
            loop {
                let _ = file.read(&mut buffer);
                str_buf.push(buffer[0]);
                byte_counter += 1;
                if byte_counter == size {break;}
            }
            data.push(RecordData::Str(String::from_utf8(str_buf.to_owned())
                                      .unwrap()));
        } else if data_type != constants::DATA_TYPE_NONE {
            let data_size = constants::data_size(data_type);
            let mut buffer_arr = [0;constants::MAX_DATA_SIZE];
            loop {
                for i in 0..data_size {
                    let _ = file.read(&mut buffer);
                    buffer_arr[i] = buffer[0];
                }
                match data_type {
                    x if x == constants::DATA_TYPE_BIT =>
                        data.push(RecordData::Bit(
                        BigEndian::read_u16(&buffer_arr[0..2]))),
                    x if x == constants::DATA_TYPE_INT16 =>
                        data.push(RecordData::Int16(
                        BigEndian::read_i16(&buffer_arr[0..2]))),
                    x if x == constants::DATA_TYPE_INT32 =>
                        data.push(RecordData::Int32(
                        BigEndian::read_i32(&buffer_arr[0..4]))),
                    x if x == constants::DATA_TYPE_REAL32 =>
                        data.push(RecordData::Real32(
                        utils::bytes_to_gds_real32(&buffer_arr[0..4]))),
                    x if x == constants::DATA_TYPE_REAL64 =>
                        data.push(RecordData::Real64(
                        utils::bytes_to_gds_real(&buffer_arr[0..8]))),
                    _ => {},
                }
                byte_counter += data_size as u16;
                if byte_counter == size {break;}
                if byte_counter + (data_size as u16) > size {
                    let mut buffer = [0;1];
                    for _ in 0..(size-byte_counter) {
                        let _ = file.read(&mut buffer);
                    }
                    break;
                }
            }
        }

        Record{size: size, rec_type: rec_type, data_type: data_type,
            data: data}
    }

    pub fn write(&self, file: &mut File) {
        let mut buf: Vec<u8> = Vec::new();
        buf.extend(utils::u16_to_vec(self.size));
        buf.push(self.rec_type);
        buf.push(self.data_type);
        for d in self.data.iter() {
            match d {
                &RecordData::Bit(x) => buf.extend(utils::u16_to_vec(x)),
                &RecordData::Int16(x) => buf.extend(utils::i16_to_vec(x)),
                &RecordData::Int32(x) => buf.extend(utils::i32_to_vec(x)),
                &RecordData::Real32(x) =>
                    buf.extend(utils::gds_real_32_to_bytes(x).to_vec()),
                &RecordData::Real64(x) =>
                    buf.extend(utils::gds_real_to_bytes(x).to_vec()),
                &RecordData::Str(ref x) => buf.extend(x.clone().into_bytes()),
                _ => {}
            }
        }
        let _ = file.write(&buf);
    }

}

impl Structure {
    pub fn new() -> Structure {
        Structure{name: String::from(""), elements: Vec::new(),
            date_mod: Date::new(), date_acc: Date::new()}
    }
}

impl Element {
    pub fn new() -> Element {
        Element{element_type: ElementType::None, parameters: Vec::new()}
    }

    pub fn to_records(&self) -> Vec<Record> {
        let mut res = Vec::new();
        let rec_type = match self.element_type {
            ElementType::Boundary => constants::REC_TYPE_BOUNDARY,
            ElementType::Path => constants::REC_TYPE_PATH,
            ElementType::StructureRef => constants::REC_TYPE_SREF,
            ElementType::ArrayRef => constants::REC_TYPE_AREF,
            ElementType::Text => constants::REC_TYPE_TEXT,
            ElementType::Node => constants::REC_TYPE_NODE,
            ElementType::Box => constants::REC_TYPE_BOX,
            ElementType::None => 0
        };
        res.push(Record::new_none(rec_type));
        for param in &self.parameters {
            match param {
                &ElementParameter::Layer(x) => res.push(Record::new_single(
                    constants::REC_TYPE_LAYER, constants::DATA_TYPE_INT16,
                    RecordData::Int16(x))),
                &ElementParameter::XY(ref x) => {
                        let mut xy_data: Vec<RecordData> = Vec::new();
                        for &(x_coord,y_coord) in x {
                            xy_data.push(RecordData::Int32(x_coord));
                            xy_data.push(RecordData::Int32(y_coord));
                        }
                        res.push(Record::new(constants::REC_TYPE_XY,
                            constants::DATA_TYPE_INT32,xy_data));
                    },
                &ElementParameter::Datatype(x) => res.push(Record::new_single(
                    constants::REC_TYPE_DATATYPE, constants::DATA_TYPE_INT16,
                    RecordData::Int16(x))),
                &ElementParameter::Width(x) => res.push(Record::new_single(
                    constants::REC_TYPE_WIDTH, constants::DATA_TYPE_INT32,
                    RecordData::Int32(x))),
                &ElementParameter::StructureName(ref x) => res.push(Record::new_single(
                    constants::REC_TYPE_SNAME, constants::DATA_TYPE_STR,
                    RecordData::Str(x.clone()))),
                &ElementParameter::ColRow(ref x) => {
                        let mut cr_data: Vec<RecordData> = Vec::new();
                        for &cr in x{
                            cr_data.push(RecordData::Int16(cr));
                        }
                        res.push(Record::new(constants::REC_TYPE_COLROW,
                            constants::DATA_TYPE_INT16, cr_data));
                    },
                &ElementParameter::TextType(x) => res.push(Record::new_single(
                    constants::REC_TYPE_TEXTTYPE, constants::DATA_TYPE_INT16,
                    RecordData::Int16(x))),
                &ElementParameter::Presentation(x) => res.push(Record::new_single(
                    constants::REC_TYPE_PRESENTATION, constants::DATA_TYPE_BIT,
                    RecordData::Bit(x))),
                &ElementParameter::String(ref x) => res.push(Record::new_single(
                    constants::REC_TYPE_STRING, constants::DATA_TYPE_STR,
                    RecordData::Str(x.clone()))),
                &ElementParameter::StrTransf(x) => res.push(Record::new_single(
                    constants::REC_TYPE_STRANS, constants::DATA_TYPE_BIT,
                    RecordData::Bit(x))),
                &ElementParameter::Magnification(x) => res.push(Record::new_single(
                    constants::REC_TYPE_MAG, constants::DATA_TYPE_REAL64,
                    RecordData::Real64(x))),
                &ElementParameter::Angle(x) => res.push(Record::new_single(
                    constants::REC_TYPE_ANGLE, constants::DATA_TYPE_REAL64,
                    RecordData::Real64(x))),
                &ElementParameter::Pathtype(x) => res.push(Record::new_single(
                    constants::REC_TYPE_PATHTYPE, constants::DATA_TYPE_INT16,
                    RecordData::Int16(x))),
                &ElementParameter::EFlags(x) => res.push(Record::new_single(
                    constants::REC_TYPE_EFLAGS, constants::DATA_TYPE_BIT,
                    RecordData::Bit(x))),
                &ElementParameter::Nodetype(x) => res.push(Record::new_single(
                    constants::REC_TYPE_NODETYPE, constants::DATA_TYPE_INT16,
                    RecordData::Int16(x))),
                &ElementParameter::BeginExt(x) => res.push(Record::new_single(
                    constants::REC_TYPE_BGNEXTN, constants::DATA_TYPE_INT32,
                    RecordData::Int32(x)))
                //TODO more parameters
            }
        }
        res.push(Record::new_none(constants::REC_TYPE_ENDEL));
        res
    }
}
