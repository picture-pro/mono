use http::Method;
use leptos::server_fn::{
  codec::{Encoding, FromReq, FromRes, IntoReq, IntoRes},
  request::{ClientReq, Req},
  response::{ClientRes, Res},
  ServerFnError,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A codec for MessagePack.
pub struct MessagePack;

/// A wrapper for a type that can be encoded to MessagePack.
#[derive(Serialize, Deserialize)]
pub struct RmpEncoded<T>(pub T);

impl Encoding for MessagePack {
  const CONTENT_TYPE: &'static str = "application/msgpack";
  const METHOD: Method = Method::POST;
}

impl<T, Request, Err> IntoReq<MessagePack, Request, Err> for RmpEncoded<T>
where
  Request: ClientReq<Err>,
  T: Serialize,
{
  fn into_req(
    self,
    path: &str,
    accepts: &str,
  ) -> Result<Request, ServerFnError<Err>> {
    let data = rmp_serde::to_vec(&self.0)
      .map_err(|e| ServerFnError::Serialization(e.to_string()))?;
    Request::try_new_post_bytes(
      path,
      MessagePack::CONTENT_TYPE,
      accepts,
      data.into(),
    )
  }
}

impl<T, Request, Err> FromReq<MessagePack, Request, Err> for RmpEncoded<T>
where
  Request: Req<Err> + Send,
  T: DeserializeOwned,
{
  async fn from_req(req: Request) -> Result<Self, ServerFnError<Err>> {
    let data = req.try_into_bytes().await?;
    rmp_serde::from_slice::<T>(&data)
      .map(RmpEncoded)
      .map_err(|e| ServerFnError::Args(e.to_string()))
  }
}

impl<T, Response, Err> IntoRes<MessagePack, Response, Err> for RmpEncoded<T>
where
  Response: Res<Err>,
  T: Serialize + Send,
{
  async fn into_res(self) -> Result<Response, ServerFnError<Err>> {
    let data = rmp_serde::to_vec(&self.0)
      .map_err(|e| ServerFnError::Serialization(e.to_string()))?;
    Response::try_from_bytes(MessagePack::CONTENT_TYPE, data.into())
  }
}

impl<T, Response, Err> FromRes<MessagePack, Response, Err> for RmpEncoded<T>
where
  Response: ClientRes<Err> + Send,
  T: DeserializeOwned,
{
  async fn from_res(res: Response) -> Result<Self, ServerFnError<Err>> {
    let data = res.try_into_bytes().await?;
    rmp_serde::from_slice(&data)
      .map(RmpEncoded)
      .map_err(|e| ServerFnError::Deserialization(e.to_string()))
  }
}
