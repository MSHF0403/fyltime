use std::path::Path;

use clap::{Command, CommandFactory};
use clap_complete::Shell;

use crate::config::Args;

fn generate_impl(
    shell: Shell,
    app: &mut Command,
    app_name: &str,
    output_dir: &Path,
    file_name: String,
) {
    let destination = output_dir.join(file_name);

    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).expect("補完用ディレクトリを作成できません");
    }

    let mut file = std::fs::File::create(destination).expect("補完ファイルを作成できません");

    clap_complete::generate(shell, app, app_name, &mut file);
}

pub(crate) fn generate(output_dir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};

    let app_name = "fyt";
    let mut app = Args::command();
    app.set_bin_name(app_name);

    generate_impl(
        Bash,
        &mut app,
        app_name,
        output_dir,
        format!("bash/{app_name}"),
    );
    generate_impl(
        Elvish,
        &mut app,
        app_name,
        output_dir,
        format!("elvish/{app_name}"),
    );
    generate_impl(
        Fish,
        &mut app,
        app_name,
        output_dir,
        format!("fish/{app_name}"),
    );
    generate_impl(
        PowerShell,
        &mut app,
        app_name,
        output_dir,
        format!("powershell/{app_name}.ps1"),
    );
    generate_impl(
        Zsh,
        &mut app,
        app_name,
        output_dir,
        format!("zsh/_{app_name}"),
    );
}
