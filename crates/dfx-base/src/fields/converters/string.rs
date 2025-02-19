use crate::field_map::FieldValue;
use crate::fields::converters::TryFrom;
use crate::fields::ConversionError;

use super::IntoBytes;

impl<'a> TryFrom<&'a FieldValue> for String {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        Ok(value.iter().map(|b| *b as char).collect())
    }
}

impl IntoBytes<FieldValue> for String {
    fn as_bytes(&self) -> FieldValue {
        self.clone().into_bytes().into()
    }
}

impl<'a> TryFrom<&'a FieldValue> for &'a str {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        // TODO encoding latin1
        // 显示value的值
        std::str::from_utf8(value).map_err(|_| ConversionError::EncodingError)
    }
}

impl IntoBytes<FieldValue> for &&str {
    fn as_bytes(&self) -> FieldValue {
        let s: String = (**self).into();
        s.into_bytes().into()
    }
}

impl IntoBytes<FieldValue> for &str {
    fn as_bytes(&self) -> FieldValue {
        let s: String = (*self).into();
        s.into_bytes().into()
    }
}

impl IntoBytes<FieldValue> for &&String {
    fn as_bytes(&self) -> FieldValue {
        let s: String = (**self).into();
        s.into_bytes().into()
    }
}

impl IntoBytes<FieldValue> for &String {
    fn as_bytes(&self) -> FieldValue {
        let s: String = (*self).into();
        s.into_bytes().into()
    }
}


impl<'a> TryFrom<&'a FieldValue> for char {
    type Error = ConversionError;

    fn try_from(value: &'a FieldValue) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            Err(ConversionError::EncodingError)
        } else {
            Ok(value[0] as char)
        }
    }
}


impl IntoBytes<FieldValue> for char {
    fn as_bytes(&self) -> FieldValue {
        vec!(*self as u8).into()
    }
}
