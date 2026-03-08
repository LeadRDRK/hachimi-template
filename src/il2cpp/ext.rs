use std::{ffi::CStr, hash::Hasher};

use fnv::FnvHasher;
use widestring::{Utf16Str, Utf16String};

use super::{
    api::il2cpp_string_new_utf16,
    types::*
};

pub trait StringExt {
    fn to_il2cpp_string(&self) -> *mut Il2CppString;
}

impl StringExt for str {
    fn to_il2cpp_string(&self) -> *mut Il2CppString {
        let text_utf16 = Utf16String::from_str(self);
        il2cpp_string_new_utf16(text_utf16.as_ptr(), text_utf16.len().try_into().unwrap())
    }
}

impl StringExt for String {
    fn to_il2cpp_string(&self) -> *mut Il2CppString {
        str::to_il2cpp_string(self)
    }
}

pub trait Il2CppStringExt {
    fn chars_ptr(&self) -> *const Il2CppChar;
    fn as_utf16str(&self) -> &Utf16Str;
    fn hash(&self) -> u64;
}

impl Il2CppStringExt for Il2CppString {
    fn chars_ptr(&self) -> *const Il2CppChar {
        self.chars.as_ptr()
    }

    fn as_utf16str(&self) -> &Utf16Str {
        unsafe { Utf16Str::from_slice_unchecked(self.chars.as_slice(self.length as usize)) }
    }

    fn hash(&self) -> u64 {
        let data = self.chars_ptr() as *const u8;
        let len = self.length as usize * std::mem::size_of::<Il2CppChar>();
        
        let mut hasher = FnvHasher::default();
        hasher.write(unsafe { std::slice::from_raw_parts(data, len) });
        hasher.finish()
    }
}

pub trait Il2CppObjectExt {
    fn klass(&self) -> *mut Il2CppClass;
}

impl Il2CppObjectExt for Il2CppObject {
    fn klass(&self) -> *mut Il2CppClass {
        unsafe { *self.__bindgen_anon_1.klass.as_ref() }
    }
}

pub trait MethodInfoExt {
    fn name_cstr(&self) -> &CStr;
    fn name_str(&self) -> Result<&str, std::str::Utf8Error>;
}

impl MethodInfoExt for MethodInfo {
    fn name_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name) }
    }

    fn name_str(&self) -> Result<&str, std::str::Utf8Error> {
        self.name_cstr().to_str()
    }
}