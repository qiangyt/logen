use logen::{cli, Logen};

fn main() {
    if let Some(yaml) = cli::yaml_with_args().unwrap() {
        let mut logen = Logen::from_yaml(&yaml);
        logen.init().unwrap();
        logen.generate().unwrap();
    }
}
