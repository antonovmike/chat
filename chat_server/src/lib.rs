#![allow(unused)]
use std::iter::Map;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub name: String,
    pub message: String,
}

#[derive(Debug)]
pub struct UserID {
    pub id: String,
    pub data: UserData,
}