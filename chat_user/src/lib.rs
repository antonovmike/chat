#![allow(unused)]
use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct UserData {
    // pub id: u16,
    pub name: String,
    pub message: String,
}