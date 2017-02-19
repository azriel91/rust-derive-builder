#![no_std]
#![feature(collections)]
#![allow(dead_code)]

#[macro_use]
extern crate derive_builder;
extern crate collections;

#[derive(Builder)]
struct IgnoreEmptyStruct {}

#[test]
fn empty_struct() {
    // this is just a compile-test - no run time checks required.
}

#[test]
fn write_docs() {
    unimplemented!()
}
