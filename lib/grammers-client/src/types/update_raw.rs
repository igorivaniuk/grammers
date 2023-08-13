// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::fmt;
use std::sync::Arc;

use grammers_tl_types as tl;

use super::{Chat, ChatMap};

#[derive(Clone)]
pub struct UpdateRaw {
    update: tl::enums::Update,
    chats: Arc<ChatMap>,
}

impl UpdateRaw {
    pub(crate) fn new(update: tl::enums::Update, chats: &Arc<ChatMap>) -> Self {
        Self {
            update,
            chats: chats.clone(),
        }
    }

    pub fn update(&self) -> &tl::enums::Update {
        &self.update
    }

    pub fn entity(&self, peer: &tl::enums::Peer) -> Option<&Chat> {
        self.chats.get(peer.into())
    }

    pub fn entities(&self) -> impl Iterator<Item = &Chat> {
        self.chats.iter().map(|x| x.1)
    }
}

impl fmt::Debug for UpdateRaw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UpdateRaw")
            .field("update", &self.update())
            .field("entities", &self.entities().collect::<Vec<_>>())
            .finish()
    }
}
