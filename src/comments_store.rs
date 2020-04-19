/*
 * Store comments into a git repository
 * (C) 2020 @notessensei, Apache 2.0 license
 */

use crate::comments_entry::BlogSubmission;
use actix::prelude::*;

pub struct CommentStore {}

impl Actor for CommentStore {
    type Context = Context<Self>;
    // fn started(&mut self, ctx: &mut Context<Self>) {
    //     let addr = ctx.address();
    //  }
}

impl Handler<BlogSubmission> for CommentStore {
    type Result = String;

    fn handle(&mut self, _msg: BlogSubmission, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{:?}", _msg);
        "It worked".to_string()
    }
}
