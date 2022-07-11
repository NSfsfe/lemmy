// SPDX-FileCopyrightText: 2019-2022 2019 Felix Ableitner, <me@nutomic.com> et al.
//
// SPDX-License-Identifier: AGPL-3.0-only

use crate::protocol::Source;
use anyhow::anyhow;
use html2md::parse_html;
use lemmy_apub_lib::values::MediaTypeMarkdownOrHtml;
use lemmy_utils::{settings::structs::Settings, LemmyError};
use url::Url;

pub mod comment;
pub mod community;
pub mod instance;
pub mod person;
pub mod post;
pub mod private_message;

pub(crate) fn read_from_string_or_source(
  content: &str,
  media_type: &Option<MediaTypeMarkdownOrHtml>,
  source: &Option<Source>,
) -> String {
  if let Some(s) = source {
    // markdown sent by lemmy in source field
    s.content.clone()
  } else if media_type == &Some(MediaTypeMarkdownOrHtml::Markdown) {
    // markdown sent by peertube in content field
    content.to_string()
  } else {
    // otherwise, convert content html to markdown
    parse_html(content)
  }
}

pub(crate) fn read_from_string_or_source_opt(
  content: &Option<String>,
  media_type: &Option<MediaTypeMarkdownOrHtml>,
  source: &Option<Source>,
) -> Option<String> {
  content
    .as_ref()
    .map(|content| read_from_string_or_source(content, media_type, source))
}

/// When for example a Post is made in a remote community, the community will send it back,
/// wrapped in Announce. If we simply receive this like any other federated object, overwrite the
/// existing, local Post. In particular, it will set the field local = false, so that the object
/// can't be fetched from the Activitypub HTTP endpoint anymore (which only serves local objects).
pub(crate) fn verify_is_remote_object(id: &Url) -> Result<(), LemmyError> {
  let local_domain = Settings::get().get_hostname_without_port()?;
  if id.domain() == Some(&local_domain) {
    Err(anyhow!("cant accept local object from remote instance").into())
  } else {
    Ok(())
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use actix::Actor;
  use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
  };
  use lemmy_api_common::request::build_user_agent;
  use lemmy_apub_lib::activity_queue::create_activity_queue;
  use lemmy_db_schema::{
    source::secret::Secret,
    utils::{establish_unpooled_connection, get_database_url_from_env},
  };
  use lemmy_utils::{
    rate_limit::{rate_limiter::RateLimiter, RateLimit},
    settings::structs::Settings,
    LemmyError,
  };
  use lemmy_websocket::{chat_server::ChatServer, LemmyContext};
  use parking_lot::Mutex;
  use reqwest::Client;
  use reqwest_middleware::ClientBuilder;
  use std::sync::Arc;

  // TODO: would be nice if we didnt have to use a full context for tests.
  //       or at least write a helper function so this code is shared with main.rs
  pub(crate) fn init_context() -> LemmyContext {
    let client = reqwest::Client::new().into();
    // activity queue isnt used in tests, so worker count makes no difference
    let queue_manager = create_activity_queue(client, 4);
    let activity_queue = queue_manager.queue_handle().clone();
    // call this to run migrations
    establish_unpooled_connection();
    let settings = Settings::init().unwrap();
    let rate_limiter = RateLimit {
      rate_limiter: Arc::new(Mutex::new(RateLimiter::default())),
      rate_limit_config: settings.rate_limit.to_owned().unwrap_or_default(),
    };
    let client = Client::builder()
      .user_agent(build_user_agent(&settings))
      .build()
      .unwrap();

    let client = ClientBuilder::new(client).build();
    let secret = Secret {
      id: 0,
      jwt_secret: "".to_string(),
    };
    let db_url = match get_database_url_from_env() {
      Ok(url) => url,
      Err(_) => settings.get_database_url(),
    };
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = Pool::builder()
      .max_size(settings.database.pool_size)
      .build(manager)
      .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    async fn x() -> Result<String, LemmyError> {
      Ok("".to_string())
    }
    let chat_server = ChatServer::startup(
      pool.clone(),
      rate_limiter,
      |_, _, _, _| Box::pin(x()),
      |_, _, _, _| Box::pin(x()),
      client.clone(),
      activity_queue.clone(),
      settings.clone(),
      secret.clone(),
    )
    .start();
    LemmyContext::create(pool, chat_server, client, activity_queue, settings, secret)
  }
}
