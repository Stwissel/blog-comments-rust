/*
 * A blog comment entry including the HTTP Parameters
 * (C) 2020 @notessensei, Apache 2.0 license
 */

#![allow(non_snake_case)]

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

// What we get from the Browser
#[derive(Deserialize)]
pub struct BlogComment {
    pub Commentor: String,
    pub eMail: String,
    pub webSite: String,
    pub Body: String,
    pub captcha: String,
    pub parentId: String,
}

// What we send to the backend
#[derive(Serialize)]
pub struct BlogSubmission {
    pub Commentor: String,
    pub eMail: String,
    pub webSite: String,
    pub Body: String,
    pub captcha: String,
    pub parentId: String,
    pub created: String,
    pub markdown: bool,
    pub parameters: HashMap<String, String>,
}
