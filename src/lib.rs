use std::collections::BTreeMap;
use std::path::{Component, Path};

use upon::Value;
use walkdir::WalkDir;

pub fn cubesteak(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = source.as_ref();
    let target = target.as_ref();

    let mut globals = BTreeMap::new();
    let mut templates = Vec::new();

    let mut engine = upon::Engine::new();

    for entry in WalkDir::new(source).follow_links(true) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let rel_path = path.strip_prefix(source)?;
            match path.extension() {
                Some(ext) if ext == "md" => {
                    let value = std::fs::read_to_string(path)?;
                    let html = markdown::to_html(&value);
                    let mut map = &mut globals;
                    for component in rel_path.parent().into_iter().flat_map(Path::components) {
                        let Component::Normal(name) = component else { unreachable!() };
                        map = {
                            let Value::Map(data) = map
                                .entry(name.to_string_lossy().replace(' ', "_"))
                                .or_insert_with(|| Value::Map(BTreeMap::new()))
                            else { unreachable!() };
                            data
                        };
                    }
                    let mut data = BTreeMap::new();
                    data.insert("body".to_owned(), Value::String(html));
                    map.insert(
                        path.with_extension("")
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .replace(' ', "_"),
                        Value::Map(data),
                    );
                }
                Some(ext) if ext == "html" || ext == "css" || ext == "js" => {
                    let name = path.to_string_lossy().to_string();
                    let value = std::fs::read_to_string(path)?;
                    engine.add_template(name.clone(), value)?;
                    templates.push((rel_path.to_string_lossy().to_string(), name));
                }
                _ => {
                    let out_path = target.join(rel_path);
                    if let Some(out_dir) = out_path.parent() {
                        std::fs::create_dir_all(out_dir)?;
                    }
                    std::fs::copy(path, out_path)?;
                }
            }
        }
    }

    let globals = Value::Map(globals);

    for (rel_path, name) in templates {
        let out_path = target.join(rel_path);
        if let Some(out_dir) = out_path.parent() {
            std::fs::create_dir_all(out_dir)?;
        }
        let template = engine.get_template(&name).unwrap();
        std::fs::write(out_path, template.render_from(globals.clone())?)?;
    }

    Ok(())
}
