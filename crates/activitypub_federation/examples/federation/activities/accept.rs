use crate::{activities::follow::Follow, objects::person::MyUser, Instance, ObjectId};
use activitypub_federation::{data::Data, traits::ActivityHandler};
use activitystreams_kinds::activity::AcceptType;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Accept {
  actor: ObjectId<MyUser>,
  object: Follow,
  #[serde(rename = "type")]
  kind: AcceptType,
  id: Url,
}

impl Accept {
  pub fn new(actor: ObjectId<MyUser>, object: Follow, id: Url) -> Accept {
    Accept {
      actor,
      object,
      kind: Default::default(),
      id,
    }
  }
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for Accept {
  type DataType = Arc<Instance>;
  type Error = crate::error::Error;

  fn id(&self) -> &Url {
    &self.id
  }

  fn actor(&self) -> &Url {
    self.actor.inner()
  }

  async fn verify(
    &self,
    _data: &Data<Self::DataType>,
    _request_counter: &mut i32,
  ) -> Result<(), Self::Error> {
    todo!()
  }

  async fn receive(
    self,
    _data: &Data<Self::DataType>,
    _request_counter: &mut i32,
  ) -> Result<(), Self::Error> {
    todo!()
  }
}
