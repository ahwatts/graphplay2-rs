use byteorder::{ByteOrder, BigEndian, LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::io;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Format {
    Ascii,
    BinaryBigEndian,
    BinaryLittleEndian,
}

impl FromStr for Format {
    type Err = io::Error;

    fn from_str(string: &str) -> io::Result<Format> {
        match string {
            "ascii"                => Ok(Format::Ascii),
            "binary_little_endian" => Ok(Format::BinaryLittleEndian),
            "binary_big_endian"    => Ok(Format::BinaryBigEndian),
            _ => Err(other_io_error(&format!("Unknown ply format: {:?}", string))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    Int8, Uint8,
    Int16, Uint16,
    Int32, Uint32,
    Float32, Float64,
}

impl FromStr for DataType {
    type Err = io::Error;

    fn from_str(string: &str) -> io::Result<DataType> {
        match string {
            "char"   | "int8"    => Ok(DataType::Int8),
            "uchar"  | "uint8"   => Ok(DataType::Uint8),
            "short"  | "int16"   => Ok(DataType::Int16),
            "ushort" | "uint16"  => Ok(DataType::Uint16),
            "int"    | "int32"   => Ok(DataType::Int32),
            "uint"   | "uint32"  => Ok(DataType::Uint32),
            "float"  | "float32" => Ok(DataType::Float32),
            "double" | "float64" => Ok(DataType::Float64),
            _ => Err(other_io_error(&format!("Unknown data type: {:?}", string))),
        }
    }
}

impl DataType {
    pub fn is_int(&self) -> bool {
        match *self {
            DataType::Float32 | DataType::Float64 => false,
            _ => true,
        }
    }

    pub fn byte_size(&self) -> i32 {
        match *self {
            DataType::Int8  | DataType::Uint8  => 1,
            DataType::Int16 | DataType::Uint16 => 2,
            DataType::Int32 | DataType::Uint32 => 4,
            DataType::Float32 => 4,
            DataType::Float64 => 8,
        }
    }

    // fn decode_int(&self, bytes: &[u8]) -> Result<i64, String> {
    //     if bytes.len() < self.byte_size() as usize {
    //         return Err(format!("Not enough bytes to decode {:?} from {:?}", self, bytes));
    //     }

    //     match *self {
    //         DataType::Int8   => Ok(unsafe { *(bytes.as_ptr() as *const i8) } as i64),
    //         DataType::Uint8  => Ok(unsafe { *(bytes.as_ptr() as *const u8) } as i64),
    //         DataType::Int16  => Ok(unsafe { *(bytes.as_ptr() as *const i16) } as i64),
    //         DataType::Uint16 => Ok(unsafe { *(bytes.as_ptr() as *const u16) } as i64),
    //         DataType::Int32  => Ok(unsafe { *(bytes.as_ptr() as *const i32) } as i64),
    //         DataType::Uint32 => Ok(unsafe { *(bytes.as_ptr() as *const u32) } as i64),
    //         _ => Err(format!("Cannot decode int for float: self = {:?}, bytes = {:?}", self, bytes)),
    //     }
    // }

    // fn decode_float(&self, bytes: &[u8]) -> Result<f64, String> {
    //     if bytes.len() < self.byte_size() as usize {
    //         return Err(format!("Not enough bytes to decode {:?} from {:?}", self, bytes));
    //     }

    //     match *self {
    //         DataType::Float32 => Ok(unsafe { *(bytes.as_ptr() as *const f32) } as f64),
    //         DataType::Float64 => Ok(unsafe { *(bytes.as_ptr() as *const f64) } as f64),
    //         _ => Err(format!("Cannot decode float for int: self = {:?}, bytes = {:?}", self, bytes)),
    //     }
    // }

    fn read_int<R: ReadBytesExt>(&self, reader: &mut R, format: &Format) -> io::Result<i64> {
        use self::DataType::*;
        use self::Format::*;

        match (*self, *format) {
            (Int8, _) => reader.read_i8().map(|i| i as i64),
            (Uint8, _) => reader.read_u8().map(|i| i as i64),

            (Int16,  BinaryLittleEndian) => reader.read_i16::<LittleEndian>().map(|i| i as i64),
            (Int16,  BinaryBigEndian)    => reader.read_i16::<BigEndian>()   .map(|i| i as i64),
            (Uint16, BinaryLittleEndian) => reader.read_u16::<LittleEndian>().map(|i| i as i64),
            (Uint16, BinaryBigEndian)    => reader.read_u16::<BigEndian>()   .map(|i| i as i64),

            (Int32,  BinaryLittleEndian) => reader.read_i32::<LittleEndian>().map(|i| i as i64),
            (Int32,  BinaryBigEndian)    => reader.read_i32::<BigEndian>()   .map(|i| i as i64),
            (Uint32, BinaryLittleEndian) => reader.read_u32::<LittleEndian>().map(|i| i as i64),
            (Uint32, BinaryBigEndian)    => reader.read_u32::<BigEndian>()   .map(|i| i as i64),
                
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("Cannot decode int for float: self = {:?}", self))),
        }
    }

    fn read_float<R: ReadBytesExt>(&self, reader: &mut R, format: &Format) -> io::Result<f64> {
        use self::DataType::*;
        use self::Format::*;

        match (*self, *format) {
            (Float32, BinaryLittleEndian) => reader.read_f32::<LittleEndian>().map(|i| i as f64),
            (Float32, BinaryBigEndian)    => reader.read_f32::<BigEndian>()   .map(|i| i as f64),
            (Float64, BinaryLittleEndian) => reader.read_f64::<LittleEndian>(),
            (Float64, BinaryBigEndian)    => reader.read_f64::<BigEndian>(),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("Cannot decode float for int: self = {:?}", self))),
        }
    }
}

#[derive(Debug)]
pub struct Document {
    format: Format,
    comments: Vec<String>,
    elements: Vec<Element>,
}

impl Document {
    pub fn from_file(filename: &str) -> io::Result<Document> {
        let file = try!(File::open(filename));
        Self::from_reader(file)
    }

    pub fn from_reader<T: Read>(reader: T) -> io::Result<Document> {
        let mut file = BufReader::new(reader);
        let mut rv = Document {
            format: Format::Ascii,
            comments: vec![],
            elements: vec![],
        };

        let mut magic = String::new();
        try!(file.read_line(&mut magic));

        if &magic != "ply\n" && &magic != "ply\r\n" {
            return Err(other_io_error(&format!("File does not appear to be a ply file: {:?}", magic)));
        }

        for line_result in (&mut file).lines() {
            let line = try!(line_result);
            let tokens: Vec<&str> = (&line).split(" ").collect();

            match *try!(tokens.get(0).ok_or(other_io_error(&format!("No tokens on line?! (line = {:?})", line)))) {
                "format" => { rv.format = try!(parse_format(&tokens)); },
                "comment" => { rv.comments.push(try!(parse_comment(&tokens))); },
                "element" => { rv.elements.push(try!(parse_element(&tokens))); },
                "property" => {
                    let elements = &mut rv.elements;
                    let element = try!(elements.last_mut().ok_or(other_io_error(("Tried to add a property but there's no current element"))));
                    element.add_property(try!(parse_property(&tokens)));
                },
                "end_header" => break,
                _ => {},
            }
        }

        for elt in rv.elements.iter_mut() {
            match rv.format {
                Format::Ascii => try!(read_ascii_element(elt, &mut file)),
                f @ Format::BinaryLittleEndian | f @ Format::BinaryBigEndian => try!(read_binary_element(elt, f, &mut file)),
            }
        }

        Ok(rv)
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    pub fn comments(&self) -> &[String] {
        &self.comments
    }
}

#[derive(Debug)]
pub struct Element {
    name: String,
    count: i32,
    properties: Vec<Property>,
}

impl Element {
    fn add_property(&mut self, prop: Property) {
        self.properties.push(prop);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn properties(&self) -> &[Property] {
        &self.properties
    }
}

#[derive(Debug, PartialEq)]
pub enum PropertyValue {
    IntScalar(Vec<i64>),
    IntList(Vec<Vec<i64>>),
    FloatScalar(Vec<f64>),
    FloatList(Vec<Vec<f64>>),
}

impl PropertyValue {
    pub fn is_list(&self) -> bool {
        match *self {
            PropertyValue::IntScalar(..) | PropertyValue::FloatScalar(..) => false,
            PropertyValue::IntList(..)   | PropertyValue::FloatList(..)   => true,
        }
    }

    pub fn is_int(&self) -> bool {
        match *self {
            PropertyValue::IntScalar(..)   | PropertyValue::IntList(..)   => true,
            PropertyValue::FloatScalar(..) | PropertyValue::FloatList(..) => false,
        }
    }

    pub fn is_same_variant(&self, other: &PropertyValue) -> bool {
        match (self, other) {
            (&PropertyValue::IntScalar(..),   &PropertyValue::IntScalar(..))   => true,
            (&PropertyValue::IntList(..),     &PropertyValue::IntList(..))     => true,
            (&PropertyValue::FloatScalar(..), &PropertyValue::FloatScalar(..)) => true,
            (&PropertyValue::FloatList(..),   &PropertyValue::FloatList(..))   => true,
            _ => false,
        }
    }

    pub fn int_scalar(&self) -> Option<&Vec<i64>> {
        match *self {
            PropertyValue::IntScalar(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn float_scalar(&self) -> Option<&Vec<f64>> {
        match *self {
            PropertyValue::FloatScalar(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn int_list(&self) -> Option<&Vec<Vec<i64>>> {
        match *self {
            PropertyValue::IntList(ref v) => Some(v),
            _ => None,
        }
    }

    pub fn float_list(&self) -> Option<&Vec<Vec<f64>>> {
        match *self {
            PropertyValue::FloatList(ref v) => Some(v),
            _ => None,
        }
    }

    fn push_ascii_scalar_value(&mut self, value_str: &str) -> Result<(), String> {
        match *self {
            PropertyValue::IntScalar(ref mut ilist) => {
                let ival = try!(
                    i64::from_str_radix(value_str, 10)
                        .map_err(|e| format!("Could not parse ply scalar int property value from {:?}: {:?}", value_str, e)));
                ilist.push(ival);
            },
            PropertyValue::FloatScalar(ref mut flist) => {
                let fval = try!(
                    f64::from_str(value_str)
                        .map_err(|e| format!("Could not parse ply scalar float property value from {:?}: {:?}", value_str, e)));
                flist.push(fval);
            },
            _ => return Err("Cannot push scalar value to list property value".to_string()),
        }

        Ok(())
    }

    fn push_ascii_list_value(&mut self, values: &[&str]) -> Result<(), String> {
        match *self {
            PropertyValue::IntList(ref mut illist) => {
                let mut list = PropertyValue::IntScalar(vec![]);
                for v in values.iter() {
                    try!(list.push_ascii_scalar_value(v));
                }
                illist.push(list.int_scalar().unwrap().clone());
            },
            PropertyValue::FloatList(ref mut fllist) => {
                let mut list = PropertyValue::FloatScalar(vec![]);
                for v in values.iter() {
                    try!(list.push_ascii_scalar_value(v));
                }
                fllist.push(list.float_scalar().unwrap().clone());
            },
            _ => return Err("Cannot push list value to scalar property value".to_string()),
        }

        Ok(())
    }

    // fn push_binary_scalar_value(&mut self, value_bytes: &[u8], data_type: DataType) -> Result<(), String> {
    //     match *self {
    //         PropertyValue::IntScalar(ref mut ilist) => {
    //             let ival = try!(data_type.decode_int(value_bytes));
    //             ilist.push(ival);
    //         },
    //         PropertyValue::FloatScalar(ref mut flist) => {
    //             let fval = try!(data_type.decode_float(value_bytes));
    //             flist.push(fval);
    //         },
    //         _ => return Err("Cannot push scalar value to list property value".to_string()),
    //     }

    //     Ok(())
    // }

    // fn push_binary_list_value(&mut self, values_bytes: &[&[u8]], data_type: DataType) -> Result<(), String> {
    //     match *self {
    //         PropertyValue::IntList(ref mut illist) => {
    //             let mut list = PropertyValue::IntScalar(vec![]);
    //             for v in values_bytes.iter() {
    //                 try!(list.push_binary_scalar_value(v, data_type));
    //             }
    //             illist.push(list.int_scalar().unwrap().clone());
    //         },
    //         PropertyValue::FloatList(ref mut fllist) => {
    //             let mut list = PropertyValue::FloatScalar(vec![]);
    //             for v in values_bytes.iter() {
    //                 try!(list.push_binary_scalar_value(v, data_type));
    //             }
    //             fllist.push(list.float_scalar().unwrap().clone());
    //         },
    //         _ => return Err("Cannot push list value to scalar property value".to_string()),
    //     }

    //     Ok(())
    // }
}

#[derive(Debug)]
pub struct Property {
    name: String,
    value_type: DataType,
    count_type: Option<DataType>,
    data: PropertyValue,
}

impl Property {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data(&self) -> &PropertyValue {
        &self.data
    }

    pub fn is_list(&self) -> bool {
        self.data.is_list()
    }

    pub fn is_int(&self) -> bool {
        self.data.is_int()
    }

    pub fn value_type(&self) -> DataType {
        self.value_type
    }

    pub fn count_type(&self) -> Option<DataType> {
        self.count_type
    }
}

fn parse_format(toks: &Vec<&str>) -> io::Result<Format> {
    Format::from_str(*try!(toks.get(1).ok_or(other_io_error("Not enough tokens for ply format"))))
}

fn parse_comment(toks: &Vec<&str>) -> io::Result<String> {
    Ok(toks.iter().skip(1).fold("".to_string(), |mut acc, &tok| {
        if !acc.is_empty() {
            acc.push_str(" ");
        }
        acc.push_str(tok);
        acc
    }))
}

fn parse_element(toks: &Vec<&str>) -> io::Result<Element> {
    let name = *try!(toks.get(1).ok_or(other_io_error("Not enough tokens for ply element name")));
    let count_str = *try!(toks.get(2).ok_or(other_io_error("Not enough tokens for ply element count")));
    let count = try!(i32::from_str_radix(count_str, 10).map_err(|e| other_io_error(&format!("Could not parse ply element count from {:?}: {:?}", count_str, e))));

    Ok(Element {
        name: name.to_string(),
        count: count,
        properties: vec![],
    })
}

fn parse_property(toks: &Vec<&str>) -> io::Result<Property> {
    let prop_type = *try!(toks.get(1).ok_or(other_io_error("Not enough tokens for ply property type")));

    if prop_type == "list" {
        let count_type_str = *try!(toks.get(2).ok_or(other_io_error("Not enough tokens for ply list property count type")));
        let value_type_str = *try!(toks.get(3).ok_or(other_io_error("Not enough tokens for ply list property value type")));
        let name = *try!(toks.get(4).ok_or(other_io_error("Not enough tokens for ply list property name")));

        let count_type = try!(DataType::from_str(count_type_str));
        let value_type = try!(DataType::from_str(value_type_str));

        Ok(Property {
            name: name.to_string(),
            value_type: value_type,
            count_type: Some(count_type),
            data: match value_type.is_int() {
                true => PropertyValue::IntList(vec![]),
                false => PropertyValue::FloatList(vec![]),
            }
        })
    } else {
        let name = *try!(toks.get(2).ok_or(other_io_error("Not enough tokens for ply property name")));
        let value_type = try!(DataType::from_str(prop_type));

        Ok(Property {
            name: name.to_string(),
            value_type: value_type,
            count_type: None,
            data: match value_type.is_int() {
                true => PropertyValue::IntScalar(vec![]),
                false => PropertyValue::FloatScalar(vec![]),
            }
        })
    }
}

fn read_ascii_element<T: BufRead>(element: &mut Element, file: &mut T) -> io::Result<()> {
    for line_result in file.lines().take(element.count as usize) {
        let line = try!(line_result);
        let mut toks = line.split(" ");

        for property in element.properties.iter_mut() {
            if property.is_list() {
                let count_str = try!(toks.next().ok_or(other_io_error("Not enough tokens for ply list property count")));
                let count = try!{
                    usize::from_str_radix(count_str, 10).map_err(|e| {
                        other_io_error(&format!("Could not get ply property count from {:?}: {:?}", count_str, e))
                    })
                };

                let mut value_toks = vec![];
                for _ in 0..count {
                    value_toks.push(try!(toks.next().ok_or(other_io_error("Not enough tokens for ply list property value"))));
                }

                try!(property.data.push_ascii_list_value(&value_toks).map_err(|s| other_io_error(&s)))
            } else {
                let value_str = try!(toks.next().ok_or(other_io_error("Not enough tokens for ply scalar property value")));
                try!(property.data.push_ascii_scalar_value(value_str).map_err(|s| other_io_error(&s)));
            }
        }
    }

    Ok(())
}

fn read_binary_element<T: BufRead>(element: &mut Element, format: Format, file: &mut T) -> io::Result<()> {
    use self::PropertyValue::*;

    for _ in 0..element.count {
        for property in element.properties.iter_mut() {
            if property.is_list() {
                let count_type = property.count_type().unwrap();
                let count = try!(count_type.read_int(file, &format));

                let value_type = property.value_type();
                let mut values = if value_type.is_int() {
                    IntScalar(Vec::new())
                } else {
                    FloatScalar(Vec::new())
                };

                for _ in 0..count {
                    match values {
                        IntScalar(ref mut ivec) => {
                            let ival = try!(value_type.read_int(file, &format));
                            ivec.push(ival);
                        },
                        FloatScalar(ref mut fvec) => {
                            let fval = try!(value_type.read_float(file, &format));
                            fvec.push(fval);
                        },
                        _ => return Err(other_io_error("Error parsing list value")),
                    }
                }

                match (&mut property.data, values) {
                    (&mut IntList(ref mut ilists), IntScalar(ref ivals)) => ilists.push(ivals.clone()),
                    (&mut FloatList(ref mut flists), FloatScalar(ref fvals)) => flists.push(fvals.clone()),
                    _ => return Err(other_io_error("Error parsing list value")),
                }
            } else {
                let value_type = property.value_type();
                if value_type.is_int() {
                    let value = try!(value_type.read_int(file, &format));
                    match &mut property.data {
                        &mut IntScalar(ref mut ivals) => ivals.push(value),
                        _ => return Err(other_io_error("Error parsing scalar value")),
                    }
                } else {
                    let value = try!(value_type.read_float(file, &format));
                    match &mut property.data {
                        &mut FloatScalar(ref mut fvals) => fvals.push(value),
                        _ => return Err(other_io_error("Error parsing float value")),
                    }
                }
            }
        }
    }

    Ok(())
}

// fn correct_endian(bytes: &mut [u8], format: Format) {
//     if (cfg!(target_endian = "big") && format == Format::BinaryLittleEndian) || (cfg!(target_endian = "little") && format == Format::BinaryBigEndian) {
//         bytes.reverse();
//     }
// }

fn other_io_error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

#[cfg(test)]
mod tests {
    use super::{Document, DataType, Format, PropertyValue};
    use std::error::Error;

    fn error_description(doc_str: &str) -> String {
        Document::from_reader(doc_str.as_bytes())
            .unwrap_err()
            .description()
            .to_string()
    }

    static OCTOHEDRON: &'static str = r"ply
format ascii 1.0
comment Simple Test Geometry
element vertex 6
property float32 x
property float32 y
property float32 z
element face 8
property list uint8 int32 vertex_indices
end_header
1.0 0.0 0.0
-1.0 0.0 0.0
0.0 0.0 1.0
0.0 0.0 -1.0
0.0 -1.0 0.0
0.0 1.0 0.0
3 4 0 2
3 4 3 0
3 4 1 3
3 4 2 1
3 5 2 0
3 5 0 3
3 5 3 1
3 5 1 2
";

    fn octohedron() -> Document {
        Document::from_reader(OCTOHEDRON.as_bytes()).unwrap()
    }

    fn octohedron_binary_le() -> Document {
        Document::from_file("test/files/octohedron_binary_le.ply").unwrap()
    }

    #[test]
    fn read_format() {
        assert_eq!(Format::Ascii, octohedron().format);
        assert_eq!(Format::BinaryLittleEndian, octohedron_binary_le().format);
    }

    #[test]
    fn read_comment() {
        let doc = octohedron();
        let comments = doc.comments();
        assert_eq!(1, comments.len());
        assert_eq!("Simple Test Geometry", comments[0]);
    }

    #[test]
    fn read_element() {
        let doc = octohedron();
        let elements = doc.elements();
        assert_eq!(2, elements.len());

        assert_eq!("vertex", elements[0].name());
        assert_eq!(6, elements[0].count());

        assert_eq!("face", elements[1].name());
        assert_eq!(8, elements[1].count());
    }

    #[test]
    fn read_properties() {
        let doc = octohedron();
        let elements = doc.elements();

        let vprops = elements[0].properties();
        assert_eq!(3, vprops.len());

        for (prop, name) in vprops.iter().zip([ "x", "y", "z" ].iter()) {
            assert_eq!(*name, prop.name());
            assert!(!prop.is_list());
            assert!(!prop.is_int());
            assert_eq!(DataType::Float32, prop.value_type());
            assert_eq!(None, prop.count_type());
            assert!(PropertyValue::FloatScalar(vec![]).is_same_variant(prop.data()));
        }

        let fprops = elements[1].properties();
        assert_eq!(1, fprops.len());
        assert_eq!("vertex_indices", fprops[0].name());
        assert!(PropertyValue::IntList(vec![]).is_same_variant(fprops[0].data()));
    }

    #[test]
    fn read_ascii_data() {
        let doc = octohedron();
        let elements = doc.elements();
        assert_eq!(2, elements.len());

        let vprops = elements[0].properties();
        assert_eq!(3, vprops.len());

        assert_eq!(
            &PropertyValue::FloatScalar(vec![ 1.0, -1.0, 0.0, 0.0, 0.0, 0.0 ]),
            vprops[0].data());

        assert_eq!(
            &PropertyValue::FloatScalar(vec![ 0.0, 0.0, 0.0, 0.0, -1.0, 1.0 ]),
            vprops[1].data());

        assert_eq!(
            &PropertyValue::FloatScalar(vec![ 0.0, 0.0, 1.0, -1.0, 0.0, 0.0 ]),
            vprops[2].data());

        let fprops = elements[1].properties();
        assert_eq!(1, fprops.len());

        assert_eq!(
            &PropertyValue::IntList(vec![
                vec![ 4, 0, 2 ], vec![ 4, 3, 0 ], vec![ 4, 1, 3 ], vec![ 4, 2, 1 ],
                vec![ 5, 2, 0 ], vec![ 5, 0, 3 ], vec![ 5, 3, 1 ], vec![ 5, 1, 2 ]]),
            fprops[0].data());
    }

    #[test]
    fn read_binary_le_data() {
        let doc = octohedron_binary_le();

        let elements = doc.elements();
        assert_eq!(2, elements.len());

        let vprops = elements[0].properties();
        assert_eq!(3, vprops.len());

        assert_eq!(
            &PropertyValue::FloatScalar(vec![ 1.0, -1.0, 0.0, 0.0, 0.0, 0.0 ]),
            vprops[0].data());

        assert_eq!(
            &PropertyValue::FloatScalar(vec![ 0.0, 0.0, 0.0, 0.0, -1.0, 1.0 ]),
            vprops[1].data());

        assert_eq!(
            &PropertyValue::FloatScalar(vec![ 0.0, 0.0, 1.0, -1.0, 0.0, 0.0 ]),
            vprops[2].data());

        let fprops = elements[1].properties();
        assert_eq!(1, fprops.len());

        assert_eq!(
            &PropertyValue::IntList(vec![
                vec![ 4, 0, 2 ], vec![ 4, 3, 0 ], vec![ 4, 1, 3 ], vec![ 4, 2, 1 ],
                vec![ 5, 2, 0 ], vec![ 5, 0, 3 ], vec![ 5, 3, 1 ], vec![ 5, 1, 2 ]]),
            fprops[0].data());
    }

    #[test]
    fn bad_magic() {
        assert_eq!(
            "File does not appear to be a ply file: \"ply 2\\n\"",
            error_description("ply 2\nformat ascii 1.0\n"));
    }

    #[test]
    fn bad_format() {
        assert_eq!(
            "Not enough tokens for ply format",
            error_description("ply\nformat\n"));

        assert_eq!(
            "Unknown ply format: \"blahdeblah\"",
            error_description("ply\nformat blahdeblah 1.0\n"));
    }

    #[test]
    fn bad_element() {
        assert_eq!(
            "Not enough tokens for ply element name",
            error_description("ply\nelement\n"));

        assert_eq!(
            "Not enough tokens for ply element count",
            error_description("ply\nelement vertex\n"));

        assert_eq!(
            "Could not parse ply element count from \"green\": ParseIntError { kind: InvalidDigit }",
            error_description("ply\nelement vertex green\n"));
    }

    #[test]
    fn bad_property() {
        assert_eq!(
            "Tried to add a property but there's no current element",
            error_description("ply\nproperty float32 x\n"));

        // Scalar properties.
        assert_eq!(
            "Not enough tokens for ply property type",
            error_description("ply\nelement vertex 12\nproperty\n"));

        assert_eq!(
            "Not enough tokens for ply property name",
            error_description("ply\nelement vertex 12\nproperty float32\n"));

        assert_eq!(
            "Unknown data type: \"puppy\"",
            error_description("ply\nelement vertex 12\nproperty puppy x\n"));

        // List properties.
        assert_eq!(
            "Not enough tokens for ply list property count type",
            error_description("ply\nelement vertex 12\nproperty list\n"));

        assert_eq!(
            "Not enough tokens for ply list property value type",
            error_description("ply\nelement vertex 12\nproperty list uint8\n"));

        assert_eq!(
            "Not enough tokens for ply list property name",
            error_description("ply\nelement vertex 12\nproperty list uint8 int32\n"));

        assert_eq!(
            "Unknown data type: \"puppy\"",
            error_description("ply\nelement face 12\nproperty list puppy int32 vertex_indices"));

        assert_eq!(
            "Unknown data type: \"puppy\"",
            error_description("ply\nelement face 12\nproperty list uint8 puppy vertex_indices"));
    }

    #[test]
    fn bad_ascii_data() {
        // Scalar data.
        assert_eq!(
            "Not enough tokens for ply scalar property value",
            error_description("ply\nelement vertex 12\nproperty float32 x\nproperty float32 y\nend_header\n1.0\n2.0 3.0"));

        assert_eq!(
            "Could not parse ply scalar int property value from \"1.0\": ParseIntError { kind: InvalidDigit }",
            error_description("ply\nelement vertex 12\nproperty int32 x\nproperty float32 y\nend_header\n1.0 1.0\n2.0 3.0"));

        assert_eq!(
            "Could not parse ply scalar float property value from \"puppy\": ParseFloatError { kind: Invalid }",
            error_description("ply\nelement vertex 12\nproperty float32 x\nproperty float32 y\nend_header\n1.0 puppy\n2.0 3.0"));

        // List data.
        assert_eq!(
            "Could not get ply property count from \"\": ParseIntError { kind: Empty }",
            error_description("ply\nelement face 12\nproperty list uint8 int32 vertex_indices\nend_header\n3 1 2 3\n\n"));

        assert_eq!(
            "Not enough tokens for ply list property value",
            error_description("ply\nelement face 12\nproperty list uint8 int32 vertex_indices\nend_header\n3 1 2\n"));

        assert_eq!(
            "Could not parse ply scalar int property value from \"1.0\": ParseIntError { kind: InvalidDigit }",
            error_description("ply\nelement vertex 12\nproperty list uint8 int32 vertex_indices\nend_header\n3 1.0 2 3\n"));
    }
}
