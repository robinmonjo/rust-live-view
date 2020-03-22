use serde_json::{Value, json};

#[derive(Debug)]
pub struct Message {
  join_ref: String,
  heartbeat_ref: String,
  topic: String,
  pub event: Event,
  pub payload: Value
}

#[derive(Debug)]
pub enum Event {
  Join,
  Heartbeat,
  Event,
  Unknown
}

impl Message {

  pub fn reply(&self, status: &str) -> String {
    self.encode(status, json!({}))
  }

  pub fn reply_with_payload(&self, status: &str, payload: Value) -> String {
    self.encode(status, payload)
  }

  fn encode(&self, status: &str, response: Value) -> String {
    let answer = json!([self.join_ref, self.heartbeat_ref, self.topic, "phx_reply", { "status": status, "response": response }]);
    serde_json::to_string(&answer).unwrap()
  }
}

pub fn decode(json_str: &str) -> Result<Message, String> {
  match serde_json::from_str(json_str) {
    Ok(arr) => slice_to_message(arr),
    Err(err) => Err(err.to_string())
  }
}

fn slice_to_message(v: Vec<Value>) -> Result<Message, String> {
  match v.as_slice() {
    [join_ref, heartbeat_ref, topic, event, payload] => {
      let m = Message {
        join_ref: string_or_empty(&join_ref),
        heartbeat_ref: string_or_empty(heartbeat_ref),
        topic: string_or_empty(topic),
        event: event_from_value(event),
        payload: payload.clone()
      };
      Ok(m)
    },
    _ => Err(String::from("expectet json array of 5 elements"))
  }
}

fn string_or_empty(json_value: &Value) -> String {
  match json_value {
    Value::String(s) => String::from(s),
    _ => String::from("")
  }
}

fn event_from_value(json_value: &Value) -> Event {
  match json_value {
    Value::String(s) => match &s[..] {
      "phx_join" => Event::Join,
      "heartbeat" => Event::Heartbeat,
      "event" => Event::Event,
      _ => Event::Unknown
    },
    _ => Event::Unknown
  }
}
