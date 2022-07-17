
use std::collections::{HashMap, BTreeMap};

use tera::{to_value, try_get_value, Tera, Value};

use super::text;


pub fn default() -> Tera {
  let mut tera = Tera::default();

  // disable autoescaping completely
  tera.autoescape_on(vec![]);

  register_default_filters(&mut tera);

  return tera;
}


pub fn register_default_filters(tera: &mut Tera) {
  tera.register_filter("map", filter_map);
  tera.register_filter("align_left", filter_align_left);
  tera.register_filter("align_right",filter_align_right);
}


fn filter_map(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
  let value = try_get_value!("map", "value", BTreeMap<String,Value>, value);

  let mut sep = match args.get("sep") {
      Some(sep) => try_get_value!("map", "sep", String, sep),
      None => "=".to_string(),
  };
  if sep.len() == 0 {
      sep = "=".to_string();
  }

  let mut delimit = match args.get("delimit") {
      Some(delimit) => try_get_value!("map", "delimit", String, delimit),
      None => ",".to_string(),
  };
  if delimit.len() == 0 {
      delimit = ",".to_string();
  }

  let mut r = String::with_capacity(value.len() * 64);
  let mut first = true;
  for (k, v) in value.iter() {
      if first {
          first = false;
      } else {
          r.push_str(&delimit);
      }

      r.push_str(k);
      r.push_str(&sep);
      r.push_str(&v.to_string());
  }
  return Ok(to_value(r).unwrap());
}


fn filter_align_left(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
  let value = try_get_value!("align_left", "value", String, value);

  let width = match args.get("width") {
      Some(width) => try_get_value!("align_left", "width", usize, width),
      None => {
          return Err(tera::Error::msg(
              "filter `align_left` expected an arg called `width`",
          ))
      }
  };

  let r = text::align_left(&value, width);
  return Ok(to_value(r).unwrap());
}

fn filter_align_right(value: &Value, args: &HashMap<String, Value>)-> tera::Result<Value> {

  let value = try_get_value!("align_right", "value", String, value);

  let width = match args.get("width") {
      Some(width) => try_get_value!("align_right", "width", usize, width),
      None => {
          return Err(tera::Error::msg(
              "filter `align_right` expected an arg called `width`",
          ))
      }
  };

  let r = text::align_right(&value, width);
  return Ok(to_value(r).unwrap());
}