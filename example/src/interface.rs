#![allow(unused)]

// see https://github.com/graphql-rust/graphql-client/issues/176
use serde::Serialize;
use serde_derive::*;
use typescript_definitions::{TypeScriptify, TypescriptDefinition};
use wasm_bindgen::prelude::*;

#[derive(Serialize, TypescriptDefinition)]
pub struct Newtype(i64);

#[derive(Serialize, TypescriptDefinition, TypeScriptify, Debug)]
pub struct Point {
    #[serde(rename = "X")]
    pub x: i64,
    #[serde(rename = "Y")]
    pub y: i64,
    pub z: i64,
}

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub enum Enum {
    #[allow(unused)]
    V1 {
        #[serde(rename = "Foo")]
        foo: bool,
    },
    #[allow(unused)]
    V2 {
        #[serde(rename = "Bar")]
        bar: i64,
        #[serde(rename = "Baz")]
        baz: u64,
    },
    #[allow(unused)]
    V3 {
        #[serde(rename = "Quux")]
        quux: String,
    },
}
// #[derive(Serialize)]
#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub struct Value<T> {
    value: T,
}

#[derive(TypescriptDefinition, Serialize, TypeScriptify)]
#[serde(tag = "tag", content = "fields")]
pub enum FrontendMessage {
    Init {
        id: String,
    },
    ButtonState {
        selected: Vec<String>,
        time: u32,
        other: Option<String>,
    },
    Render {
        html: String,
        time: u32,
        other: Result<&'static str, i32>,
    },
    Stuff {
        borrow: Value<i32>,
    },
}

use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub struct Borrow<'a> {
    raw: &'a str,
    cow: Cow<'a, str>,
    map: HashMap<String, i32>,
}

#[derive(Serialize, TypescriptDefinition, TypeScriptify)]
pub struct MyBytes {
    #[serde(serialize_with = "typescript_definitions::as_byte_string")]
    pub buffer: Vec<u8>, //pub buffer: &'a [u8]
                         // pub buffer : Bytes<'a>
}
