use crate::runtime::{Env, TryEnvFuture};
use crate::types::api::{APIResult, FetchRequestParams};
use http::Request;
use serde::{Deserialize, Serialize};

pub fn fetch_api<
    E: Env,
    #[cfg(not(feature = "env-future-send"))] BODY: Serialize + 'static,
    #[cfg(feature = "env-future-send")] BODY: Serialize + Send + 'static,
    REQ: FetchRequestParams<BODY> + Clone + Serialize,
    #[cfg(not(feature = "env-future-send"))] RESP: for<'de> Deserialize<'de> + 'static,
    #[cfg(feature = "env-future-send")] RESP: for<'de> Deserialize<'de> + Send + 'static,
>(
    api_request: &REQ,
) -> TryEnvFuture<APIResult<RESP>> {
    let mut url = api_request
        .endpoint()
        .join("api/")
        .expect("url builder failed")
        .join(&api_request.path())
        .expect("url builder failed");
    url.set_query(api_request.query().as_deref());
    let request = Request::builder()
        .method(api_request.method())
        .uri(url.as_str())
        .body(api_request.to_owned().body())
        .expect("request builder failed");
    E::fetch::<_, _>(request)
}
