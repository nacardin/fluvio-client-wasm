use fluvio::TopicProducer as NativeTopicProducer;
use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::FluvioError;

#[wasm_bindgen]
pub struct TopicProducer {
    inner: Rc<NativeTopicProducer>,
}
#[wasm_bindgen]
impl TopicProducer {
    pub fn send(&self, key: String, value: String) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            rc.send(key, value)
                .await
                .map(|_| JsValue::null()) //
                .map_err(|e| FluvioError::from(e).into())
        })
    }
}

impl From<NativeTopicProducer> for TopicProducer {
    fn from(inner: NativeTopicProducer) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}
