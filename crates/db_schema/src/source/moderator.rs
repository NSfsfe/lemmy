// SPDX-FileCopyrightText: 2019-2022 2019 Felix Ableitner, <me@nutomic.com> et al.
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::newtypes::{CommentId, CommunityId, PersonId, PostId};
use serde::{Deserialize, Serialize};

#[cfg(feature = "full")]
use crate::schema::{
  mod_add,
  mod_add_community,
  mod_ban,
  mod_ban_from_community,
  mod_hide_community,
  mod_lock_post,
  mod_remove_comment,
  mod_remove_community,
  mod_remove_post,
  mod_sticky_post,
  mod_transfer_community,
};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_remove_post")]
pub struct ModRemovePost {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub post_id: PostId,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_remove_post")]
pub struct ModRemovePostForm {
  pub mod_person_id: PersonId,
  pub post_id: PostId,
  pub reason: Option<String>,
  pub removed: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_lock_post")]
pub struct ModLockPost {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub post_id: PostId,
  pub locked: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_lock_post")]
pub struct ModLockPostForm {
  pub mod_person_id: PersonId,
  pub post_id: PostId,
  pub locked: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_sticky_post")]
pub struct ModStickyPost {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub post_id: PostId,
  pub stickied: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_sticky_post")]
pub struct ModStickyPostForm {
  pub mod_person_id: PersonId,
  pub post_id: PostId,
  pub stickied: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_remove_comment")]
pub struct ModRemoveComment {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub comment_id: CommentId,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_remove_comment")]
pub struct ModRemoveCommentForm {
  pub mod_person_id: PersonId,
  pub comment_id: CommentId,
  pub reason: Option<String>,
  pub removed: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_remove_community")]
pub struct ModRemoveCommunity {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub community_id: CommunityId,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_remove_community")]
pub struct ModRemoveCommunityForm {
  pub mod_person_id: PersonId,
  pub community_id: CommunityId,
  pub reason: Option<String>,
  pub removed: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_ban_from_community")]
pub struct ModBanFromCommunity {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub community_id: CommunityId,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_ban_from_community")]
pub struct ModBanFromCommunityForm {
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub community_id: CommunityId,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_ban")]
pub struct ModBan {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_hide_community")]
pub struct ModHideCommunityForm {
  pub community_id: CommunityId,
  pub mod_person_id: PersonId,
  pub hidden: Option<bool>,
  pub reason: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_hide_community")]
pub struct ModHideCommunity {
  pub id: i32,
  pub community_id: CommunityId,
  pub mod_person_id: PersonId,
  pub reason: Option<String>,
  pub hidden: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_ban")]
pub struct ModBanForm {
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub reason: Option<String>,
  pub banned: Option<bool>,
  pub expires: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_add_community")]
pub struct ModAddCommunity {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub community_id: CommunityId,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_add_community")]
pub struct ModAddCommunityForm {
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub community_id: CommunityId,
  pub removed: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_transfer_community")]
pub struct ModTransferCommunity {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub community_id: CommunityId,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_transfer_community")]
pub struct ModTransferCommunityForm {
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub community_id: CommunityId,
  pub removed: Option<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "full", table_name = "mod_add")]
pub struct ModAdd {
  pub id: i32,
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub removed: Option<bool>,
  pub when_: chrono::NaiveDateTime,
}

#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", table_name = "mod_add")]
pub struct ModAddForm {
  pub mod_person_id: PersonId,
  pub other_person_id: PersonId,
  pub removed: Option<bool>,
}
