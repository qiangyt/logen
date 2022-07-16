use crate::{
    def::{AppD, Level, LoggerD, MessageD},
    formatter::Formatter,
    template::{Template, TemplateEngine},
    timestamp::Timestamp,
};
use anyhow::Result;

use rand::Rng;

pub struct App<'a> {
    def: &'a AppD,
    formatter: Box<dyn Formatter + 'a>,
    template_engine: &'a TemplateEngine,
}

impl<'a> App<'a> {
    pub fn new(def: &'a AppD, template_engine: &'a mut TemplateEngine) -> Result<App<'a>> {
        Ok(App {
            def,
            formatter: def.formatter.new_formatter(),
            template_engine,
        })
    }

    pub fn generate(&mut self) -> Result<()> {
        let d = self.def;
        let f = self.formatter.as_ref();
        let mut ts = Timestamp::new(&d.timestamp, d.num_of_lines);

        for i in 0..d.num_of_lines {
            ts.inc();

            let t = &mut self.new_template(i);
            t.set("timestamp", &f.format_timestamp(&ts));

            let l = self.def.choose_logger();
            let m = l.choose_message();

            self.populate_logger(t, l)?;
            self.populate_message(t, m)?;

            println!("{}", f.format(t, &self.def.name)?);
        }

        Ok(())
    }
    //
    fn new_template(&self, index: u64) -> Template {
        let d = self.def;

        let mut r = Template::new(self.template_engine);
        r.set("app", &d.name);
        r.set("index", &index);
        r.set("host", d.choose_host());
        r.set("pid", &rand::thread_rng().gen::<u16>());

        r
    }

    fn populate_logger(&self, t: &mut Template, d: &LoggerD) -> Result<()> {
        t.set("logger", &d.name);
        Ok(())
    }

    fn populate_message(&self, t: &mut Template, d: &MessageD) -> Result<()> {
        t.set("file", &d.file);
        t.set("line", &d.line);
        t.set("method", &d.method);
        t.set(
            "level",
            match d.level {
                Level::Fine => "FINE",
                Level::Trace => "TRACE",
                Level::Debug => "DEBUG",
                Level::Info => "INFO",
                Level::Warn => "WARN",
                Level::Error => "ERROR",
                Level::Fatal => "FATAL",
            },
        );

        let msg_text = t.render(&d.id)?;
        t.set("message", &msg_text);

        Ok(())
    }
}
