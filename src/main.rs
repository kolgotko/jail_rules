extern crate libc;
extern crate sysctl;

use sysctl::{ Ctl, CtlType, CtlValue };
use std::mem::size_of;

use std::ffi::CString;

pub enum Val {
    CString(CString),
    Buffer(Vec<u8>),
    I32(i32),
    U32(u32),
    Bool(bool),
    Null,
}

impl From<String> for Val {
    fn from(value: String) -> Self {
        Val::Buffer(value.into_bytes())
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Val::I32(value)
    }
}

impl From<u32> for Val {
    fn from(value: u32) -> Self {
        Val::U32(value)
    }
}

impl From<bool> for Val {
    fn from(value: bool) -> Self {
        Val::Bool(value)
    }
}

impl<'a> From<&'a str> for Val {
    fn from(value: &str) -> Self {
        Val::Buffer(value.to_string().into_bytes())
    }
}

fn main() {
    println!("Hello, world!");

    let ctl_all = Ctl::new("security.jail.param").unwrap();

    for ctl in ctl_all {

        let ctl = ctl.unwrap();

        let name = ctl.name().unwrap();
        let value = ctl.value().unwrap();
        let value_type = ctl.value_type().unwrap();

        let val: Val = match value_type {
            CtlType::Int => size_of::<i32>() as i32,
            CtlType::Ulong => size_of::<u32>() as i32,
            CtlType::String => {

                if let CtlValue::String(v) = value {

                    v.parse().unwrap()

                } else { 0 }

            },
            CtlType::Struct => {

                if let CtlValue::Struct(v) = value {

                    v[0] as i32

                } else { 0 }

            },
            _ => { 0 },
        };
    }

}
