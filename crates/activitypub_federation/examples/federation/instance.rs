use crate::{
  error::Error,
  objects::{
    note::MyPost,
    person::{MyUser, PersonAcceptedActivities},
  },
  ObjectId,
};
use activitypub_federation::{
  context::WithContext,
  data::Data,
  inbox::receive_activity,
  traits::ApubObject,
  InstanceSettingsBuilder,
  LocalInstance,
  APUB_JSON_CONTENT_TYPE,
};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use http_signature_normalization_actix::prelude::VerifyDigest;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::{borrow::Borrow, ops::Deref, sync::Arc};
use tokio::task;
use url::Url;

pub struct Instance {
  /// This holds all library data
  local_instance: LocalInstance,
  /// Our "database" which contains all known users (local and federated)
  users: Vec<MyUser>,
  /// Same, but for posts
  posts: Vec<MyPost>,
}

impl Instance {
  pub fn new(hostname: String) -> Instance {
    let settings = InstanceSettingsBuilder::default()
      .testing_send_sync(true)
      .build()
      .expect("build settings");
    let local_instance = LocalInstance::new(hostname.clone(), Client::default().into(), settings);
    Instance {
      local_instance,
      users: vec![],
      posts: vec![],
    }
  }

  pub fn get_user(&self) -> &MyUser {
    self.users.iter().find(|u| u.local).unwrap()
  }

  pub fn get_all_posts(&self) -> &Vec<MyPost> {
    &self.posts
  }

  pub fn get_local_instance(&self) -> &LocalInstance {
    &self.local_instance
  }

  pub fn listen(instance: &Arc<Instance>) -> Result<(), Error> {
    let hostname = instance.local_instance.hostname();
    let instance = instance.clone();
    let server = HttpServer::new(move || {
      App::new()
        .app_data(Data::new(instance.clone()))
        .route("/objects/{user_name}", web::get().to(http_get_user))
        .service(
          web::scope("")
            // Important: this ensures that the activity json matches the hashsum in signed
            // HTTP header
            // TODO: it would be possible to get rid of this by verifying hash in
            //       receive_activity()
            .wrap(VerifyDigest::new(Sha256::new()))
            .route("/u/{user_name}/inbox", web::post().to(http_post_user_inbox)),
        )
    })
    .bind(hostname)?
    .run();
    task::spawn(server);
    Ok(())
  }
}

/// Handles requests to fetch user json over HTTP
async fn http_get_user(
  request: HttpRequest,
  data: web::Data<Arc<Instance>>,
) -> Result<HttpResponse, Error> {
  let data = data.into_inner().borrow().clone();
  let url = Url::parse(&request.uri().to_string())?;
  let user = ObjectId::<MyUser>::new(url)
    .dereference_local(&data)
    .await?
    .into_apub(&data)
    .await?;
  Ok(
    HttpResponse::Ok()
      .content_type(APUB_JSON_CONTENT_TYPE)
      .json(WithContext::new_default(user)),
  )
}

/// Handles messages received in user inbox
async fn http_post_user_inbox(
  request: HttpRequest,
  payload: String,
  data: web::Data<Arc<Instance>>,
) -> Result<HttpResponse, Error> {
  let data = data.into_inner().borrow().clone();
  let activity = serde_json::from_str(&payload)?;
  Ok(
    receive_activity::<WithContext<PersonAcceptedActivities>, MyUser, Arc<Instance>, Error>(
      request,
      activity,
      &data.local_instance,
      &Data::new(data),
    )
    .await?,
  )
}
