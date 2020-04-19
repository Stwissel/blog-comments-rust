/*
 * A blog comment entry including the HTTP Parameters
 * (C) 2020 @notessensei, Apache 2.0 license
 */

#![allow(non_snake_case)]

use actix::prelude::*;
use chrono::Utc;
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
#[derive(Serialize, Debug, Clone)]
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

impl BlogSubmission {
    pub fn from_blog_comment(
        comment_post: BlogComment,
        params: HashMap<String, String>,
    ) -> BlogSubmission {
        BlogSubmission {
            created: Utc::now().format("%B %m, %Y %r").to_string(),
            markdown: true,
            parameters: params,
            Commentor: comment_post.Commentor.clone(),
            eMail: comment_post.eMail.clone(),
            webSite: comment_post.webSite.clone(),
            Body: comment_post.Body.clone(),
            captcha: comment_post.captcha.clone(),
            parentId: comment_post.parentId.clone(),
        }
    }
}

impl Message for BlogSubmission {
    type Result = String;
}
