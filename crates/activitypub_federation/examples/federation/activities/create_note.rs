use crate::{
  objects::{note::Note, person::MyUser},
  Instance,
  ObjectId,
};
use activitypub_federation::{data::Data, deser::deserialize_one_or_many, traits::ActivityHandler};
use activitystreams_kinds::activity::CreateType;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNote {
  pub(crate) actor: ObjectId<MyUser>,
  #[serde(deserialize_with = "deserialize_one_or_many")]
  pub(crate) to: Vec<Url>,
  pub(crate) object: Note,
  #[serde(rename = "type")]
  pub(crate) kind: CreateType,
  pub(crate) id: Url,
}

impl CreateNote {
  pub fn new(note: Note, id: Url) -> CreateNote {
    CreateNote {
      actor: note.attributed_to.clone(),
      to: note.to.clone(),
      object: note,
      kind: CreateType::Create,
      id,
    }
  }
}

#[async_trait::async_trait(?Send)]
impl ActivityHandler for CreateNote {
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
