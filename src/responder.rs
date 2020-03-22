
use serde_json::{json, Value};

use crate::message::{Message, Event};
use crate::view;

pub fn response_for(msg: &Message) -> Option<String> {
  match msg {
    Message { event: Event::Join, .. } => {
      let s = [view::counter_component(0)];
      Some(msg.reply_with_payload("ok", json!({ "rendered": { "s": s }})))
    }
    Message { event: Event::Heartbeat, .. } => Some(msg.reply("ok")),
    Message { event: Event::Event, .. } => handle_counter_event(msg),
    Message { event: Event::Unknown, .. } => None
  }
}

fn handle_counter_event(msg: &Message) -> Option<String> {
  let event = match &msg.payload["event"] {
    Value::String(e) => e,
    _ => return None
  };

  let c: i32 = match &msg.payload["value"]["counter"] {
    Value::String(c) => c.parse().unwrap(),
    _ => return None
  };

  let c = c + match &event[..] {
    "inc_counter" => 1,
    "dec_counter" => -1,
    _ => 0
  };
  let s = [view::counter_component(c)];
  Some(msg.reply_with_payload("ok", json!({ "diff": { "s": s }})))
}
