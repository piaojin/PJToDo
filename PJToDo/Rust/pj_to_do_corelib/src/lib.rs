// #![feature(unboxed_closures)]
#![feature(custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(core_intrinsics)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// #![feature(impl_trait_in_bindings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate diesel;
// use diesel::sqlite::Sqlite;
// use diesel::debug_query;
// use diesel::prelude::*;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate libc;

extern crate cc;

extern crate flate2;

pub mod to_do_type;

pub mod to_do_tag;

pub mod to_do;

#[macro_use]
pub mod common;

pub mod mine;

pub mod network;

pub mod repos;

#[macro_use]
pub mod db;

pub mod pal;

pub mod init;

pub mod service;

pub mod controllers;

pub mod delegates;

pub mod c_binding_extern;
