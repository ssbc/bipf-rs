use std::collections::HashMap;

/*
// type constants ('tags') indicating the encoding of the value
const STRING: u8 = 0;       // 000 - utf8 encoded string
const BUFFER: u8 = 1;       // 001 - raw binary buffer
const INT: u8 = 2;          // 010 - little endian 32 bit integer (u32)
const DOUBLE: u8 = 3;       // 011 - little endian 64 bit float (f64)
const ARRAY: u8 = 4;        // 100 - array of any other value
const OBJECT: u8 = 5;       // 101 - list of string:value pairs
const BOOLNULL: u8 = 6;     // 110 - a boolean, or null (Option<bool, None>)
const RESERVED: u8 = 7;     // 111 - custom type. specific type should be indicated by varint at start of buffer
*/

// const TAG_SIZE: u8 = 3;
// const TAG_MASK: u8 = 7;

/// A representation of all data types which can be encoded and decoded using the Binary In-Place Format (BIPF). Each type has an associated type tag.
pub enum Type {
    String = 0,   // String (UTF-8)
    Buffer = 1,   // Vec<u8>
    Integer = 2,  // i32
    Double = 3,   // f64
    Array = 4,    // [T]
    Object = 5,   // HashMap<K, V>
    Boolnull = 6, // Option<bool>
    Reserved = 7, //
}

/// The `Value` trait enables functions to be written with generic type signatures which
/// allow all valid data types, as defined by the BIPF specification.
trait Value {
    fn to_type(&self) -> Type;
}

// 0 - string
impl Value for String {
    fn to_type(&self) -> Type {
        Type::String
    }
}

// 1 - buffer
impl Value for Vec<u8> {
    fn to_type(&self) -> Type {
        Type::Buffer
    }
}

// 2 - integer
impl Value for i32 {
    fn to_type(&self) -> Type {
        Type::Integer
    }
}

// 3 - double
impl Value for f64 {
    fn to_type(&self) -> Type {
        Type::Double
    }
}

// 4 - array
impl<T> Value for [T] {
    fn to_type(&self) -> Type {
        Type::Array
    }
}

// 5 - object
impl<K, V> Value for HashMap<K, V> {
    fn to_type(&self) -> Type {
        Type::Object
    }
}

// 6 - boolnull
impl Value for Option<bool> {
    fn to_type(&self) -> Type {
        Type::Boolnull
    }
}

// 7 - reserved
// how to catch any type which is not listed above?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_string_type_int() {
        let i = Type::String as i32;
        assert_eq!(i, 0);
    }

    #[test]
    fn get_string_type() -> Result<(), String> {
        let s = "this is a string".to_string();
        let t = s.to_type();
        match t {
            Type::String => Ok(()),
            _ => Err(String::from("type is not a string")),
        }
    }

    #[test]
    fn get_buffer_type_int() {
        let i = Type::Buffer as i32;
        assert_eq!(i, 1);
    }

    #[test]
    fn get_buffer_type() -> Result<(), String> {
        let b = vec![0; 5];
        let t = b.to_type();
        match t {
            Type::Buffer => Ok(()),
            _ => Err(String::from("type is not a buffer (vector)")),
        }
    }

    #[test]
    fn get_integer_type_int() {
        let i = Type::Integer as i32;
        assert_eq!(i, 2);
    }

    #[test]
    fn get_integer_type() -> Result<(), String> {
        let i = 0;
        let t = i.to_type();
        match t {
            Type::Integer => Ok(()),
            _ => Err(String::from("type is not an integer")),
        }
    }

    #[test]
    fn get_double_type_int() {
        let d = Type::Double as i32;
        assert_eq!(d, 3);
    }

    #[test]
    fn get_double_type() -> Result<(), String> {
        let d = 1.0;
        let t = d.to_type();
        match t {
            Type::Double => Ok(()),
            _ => Err(String::from("type is not a double (f64)")),
        }
    }

    #[test]
    fn get_array_type_int() {
        let a = Type::Array as i32;
        assert_eq!(a, 4);
    }

    #[test]
    // since array can be of any type T, we test with u8 (here) and str (below)
    fn get_array_u8_type() -> Result<(), String> {
        let a: [u8; 5] = [0, 0, 0, 0, 0];
        let t = a.to_type();
        match t {
            Type::Array => Ok(()),
            _ => Err(String::from("type is not an array")),
        }
    }

    #[test]
    fn get_array_str_type() -> Result<(), String> {
        let a = ["kyanite", "galactic", "mycelium"];
        let t = a.to_type();
        match t {
            Type::Array => Ok(()),
            _ => Err(String::from("type is not an array")),
        }
    }

    #[test]
    fn get_object_type_int() {
        let o = Type::Object as i32;
        assert_eq!(o, 5);
    }

    #[test]
    fn get_object_type() -> Result<(), String> {
        let mut hm = HashMap::new();
        hm.insert("first".to_string(), 25);
        hm.insert("second".to_string(), 50);
        let t = hm.to_type();
        match t {
            Type::Object => Ok(()),
            _ => Err(String::from("type is not an object (hashmap)")),
        }
    }

    #[test]
    fn get_boolnull_type_int() {
        let b = Type::Boolnull as i32;
        assert_eq!(b, 6);
    }

    #[test]
    fn get_boolnull_some_type() -> Result<(), String> {
        let os = Some(true);
        let t = os.to_type();
        match t {
            Type::Boolnull => Ok(()),
            _ => Err(String::from("type is not a boolnull (option<bool>)")),
        }
    }

    #[test]
    fn get_boolnull_none_type() -> Result<(), String> {
        let on = None;
        let t = on.to_type();
        match t {
            Type::Boolnull => Ok(()),
            _ => Err(String::from("type is not a boolnull (option<bool>)")),
        }
    }
}
