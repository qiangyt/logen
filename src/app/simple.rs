use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
struct Message {
    #[serde(skip)]
    id: String,

    template: String,
    file: String,
    line: usize,
    method: String,

    #[serde(default)]
    level: Level,
}

impl Message {
    fn init(&mut self, id: String, tmpl: &mut TemplateEngine) -> Result<()> {
        self.id = id;
        self.with_template(tmpl)
    }

    fn with_template(&self, tmpl: &mut TemplateEngine) -> Result<()> {
        tmpl.add_template(&self.id, &self.template)
    }

    fn populate(&self, t: &mut Template) -> Result<()> {
        t.set("file", &self.file);
        t.set("line", &self.line);
        t.set("method", &self.method);
        t.set("level", self.level.name());

        let msg_text = t.render(&self.id)?;
        t.set("message", &msg_text);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Logger {
    #[serde(skip)]
    id: String,

    name: String,
    message: Vec<Message>,
}

impl Logger {
    fn init(&mut self, id: String, tmpl: &mut TemplateEngine) -> Result<()> {
        self.id = id;
        self.init_message(tmpl)
    }

    fn init_message(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        if self.message.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 message",
                self.name
            ));
        }

        for (i, message_d) in self.message.iter_mut().enumerate() {
            let msg_id = format!("{}/{}", self.id, i);
            message_d.init(msg_id, tmpl)?;
        }

        return Ok(());
    }

    fn choose_message(&self) -> &Message {
        util::rand::choose_arr(&self.message)
    }

    fn populate(&self, t: &mut Template) -> Result<()> {
        t.set("logger", &self.name);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct App {
    #[serde(skip)]
    name: String,

    #[serde(skip, default)]
    template_engine: TemplateEngine,

    #[serde(default)]
    output: Output,

    //#[serde(flatten)]
    #[serde(default)]
    mdc: BTreeMap<String, Value>,

    num_of_lines: u64,
    begin_time: DateTime<Utc>, //rfc3339
    end_time: DateTime<Utc>,
    host: Vec<String>,
    logger: Vec<Logger>,
}

#[typetag::serde(name = "simple")]
impl crate::App for App {
    fn init(&mut self, name: &str) -> Result<()> {
        self.name = name.to_string();
        self.output.init(&self.name, &mut self.template_engine)?;
        self.init_logger()
    }

    fn generate(&self, sender: Sender<Line>) -> Result<()> {
        let f = self.output.formatter();
        let mut ts = Timestamp::new(&self.begin_time, &self.end_time, self.num_of_lines);

        for i in 0..self.num_of_lines {
            let timetamp = ts.inc();

            let t = &mut self.new_template(i);
            t.set("timestamp", &f.format_timestamp(&timetamp));

            let logger = self.choose_logger();
            logger.populate(t)?;
            logger.choose_message().populate(t)?;

            sender.send(Line {
                name: self.name.to_string(),
                timestamp: *timetamp,
                text: f.format(t, &self.name)?
            })?;
        }

        Ok(())
    }

}

impl App {
    fn init_logger(&mut self) -> Result<()> {
        if self.host.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 host",
                self.name
            ));
        }

        if self.logger.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 logger",
                self.name
            ));
        }

        for (i, logger) in self.logger.iter_mut().enumerate() {
            let logger_id = format!("{}/{}", self.name, i);
            logger.init(logger_id, &mut self.template_engine)?;
        }

        return Ok(());
    }

    fn choose_host(&self) -> &str {
        &util::rand::choose_arr(&self.host)
    }

    fn choose_logger(&self) -> &Logger {
        util::rand::choose_arr(&self.logger)
    }

    fn new_template<'a>(&'a self, index: u64) -> Template {
        let mut r = Template::new(&&self.template_engine);

        r.set("app", &self.name);
        r.set("index", &index);
        r.set("host", self.choose_host());
        r.set("pid", &rand::thread_rng().gen::<u16>());
        r.set("mdc", &self.mdc);

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        let f = serde_yaml::from_str::<Output>(r#"
            flat:
                timestamp_format: "%Y-%m-%d %H:%M:%S"
                template: '{{timestamp}} <{{level | upper | align_left(width=5)}}> {{logger.name}} {{file}}/{{line}} {{method}} - {{message}}'
            json:
                style: bunyan"#).unwrap();
        //assert!(f."abc".is_err());
    }
}
