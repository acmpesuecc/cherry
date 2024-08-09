/*
 * This file is a modified version of test vectors from :
 * https://datatracker.ietf.org/doc/html/rfc5769
 * */

#![allow(unused)] //All these fixtures are used in tests and raises unused warning in clippy
#![cfg_attr(any(), rustfmt::skip)] //To maintain the readable binary formatting

pub const EXAMPLE_STUN_REQUEST_TRANSACTION_ID: [u8; 12] = [
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_REQUEST_BINDING_HEADER_BINARY: [u8; 20] = [
    0x00, 0x01, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_INDICATION_BINDING_HEADER_BINARY: [u8; 20] = [
    0x00, 0x11, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_SUCCESS_BINDING_RESPONSE_HEADER_BINARY: [u8; 20] = [
    0x01, 0x01, 0x00, 0x58,
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_ERROR_BINDING_RESPONSE_HEADER_BINARY: [u8; 20] = [
    0x01, 0x11, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_INCORRECT_METHOD_HEADER_BINARY: [u8; 20] = [
    0x01, 0x02, 0x00, 0x58, //0000_0001_0000_0010 -- Incorrect method, not binding
                            //(0000_000X_000X_0001)
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_INCORRECT_MAGIC_NUMBER_HEADER_BINARY: [u8; 20] = [
    0x00, 0x01, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x43, // 0x21 0x12 0xa4 0x42 --Is the corrent magic Number
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf, 0xae,
];

pub const STUN_SMALLER_HEADER_BINARY: [u8; 19] = [
    0x00, 0x01, 0x00, 0x58, 
    0x21, 0x12, 0xa4, 0x42, 
    0xb7, 0xe7, 0xa7, 0x01, 
    0xbc, 0x34, 0xd6, 0x86,
    0xfa, 0x87, 0xdf,      //Header must alwaus be 20 bytes in size
];
