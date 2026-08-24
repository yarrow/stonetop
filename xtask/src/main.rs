use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use which::which;
use xshell::{Shell, cmd};

fn main() -> Result<()> {
    let missing = match (which("json5").is_ok(), which("quicktype").is_ok()) {
        (false, false) => Some("json5 and quicktype"),
        (false, true) => Some("json5"),
        (true, false) => Some("quicktype"),
        (true, true) => None,
    };
    if let Some(missing) = missing {
        bail!("Please use `npm install -g` to install {missing}");
    }
    let codegen = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("Can't find workspace root")?
        .join("codegen");

    let sh = Shell::new()?;
    let json_dir = sh.create_temp_dir()?;
    let json = json_dir.path();

    sh.change_dir(&codegen);
    let mut schema_data = Vec::new();
    for j5 in sh.read_dir("json5")? {
        if j5.extension().and_then(OsStr::to_str) == Some("json5") {
            let js = json.join(j5.with_extension("json").file_name().unwrap());
            cmd!(sh, "json5 --space 2 --out-file {js} {j5}")
                .quiet()
                .run()?;
            schema_data.push(js);
        }
    }
    let schema = &codegen.join("schema.json");
    cmd!(sh, "quicktype -o {schema} --lang schema {schema_data...}")
        .quiet()
        .run()?;
    let playbook = &codegen.join("src").join("playbook.rs");
    cmd!(
        sh,
        "quicktype -o {playbook} --src-lang schema {schema} --density dense --no-leading-comments"
    )
    .quiet()
    .run()?;

    Ok(())
}
