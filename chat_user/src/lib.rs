#![allow(unused)]
use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UserData {
    // pub id: u16,
    pub name: String,
    pub message: String,
}