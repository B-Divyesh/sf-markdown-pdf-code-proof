use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use std::os::unix::process::CommandExt;

use tempfile::TempDir;

use crate::sandbox::Sandbox;

pub struct RenderedPdf {
    pub path: PathBuf,
    pub engine: String,
    _workspace: Option<TempDir>,
}

pub fn render(
    source: &Path,
    engine: &str,
    custom_command: Option<&str>,
    timeout: Duration,
) -> Result<RenderedPdf, String> {
    let workspace = tempfile::Builder::new()
        .prefix("codeproof-")
        .tempdir()
        .map_err(|e| format!("could not create renderer workspace: {e}"))?;
    let output = workspace.path().join("release.pdf");
    let source = source
        .canonicalize()
        .map_err(|e| format!("could not resolve {}: {e}", source.display()))?;
    let source_parent = source.parent().unwrap_or(Path::new("."));

    let (program, args, label) = if let Some(spec) = custom_command {
        let words =
            shell_words::split(spec).map_err(|e| format!("invalid --engine-command: {e}"))?;
        if words.is_empty() {
            return Err("--engine-command is empty".into());
        }
        if !words.iter().any(|w| w.contains("{input}"))
            || !words.iter().any(|w| w.contains("{output}"))
        {
            return Err("--engine-command must contain both {input} and {output}".into());
        }
        let input = source.to_string_lossy();
        let out = output.to_string_lossy();
        let args = words[1..]
            .iter()
            .map(|w| w.replace("{input}", &input).replace("{output}", &out))
            .collect();
        (words[0].clone(), args, "custom".to_owned())
    } else if engine == "pandoc" {
        (
            "pandoc".to_owned(),
            vec![
                source.display().to_string(),
                "--from=gfm-raw_html-raw_tex".into(),
                "--standalone".into(),
                format!("--resource-path={}", source_parent.display()),
                format!("--output={}", output.display()),
            ],
            "pandoc".to_owned(),
        )
    } else {
        return Err(format!(
            "unsupported engine '{engine}'; use pandoc or --engine-command"
        ));
    };

    let path = std::env::var_os("PATH").unwrap_or_default();
    let program_path = resolve_program(&program, &path)?;
    let sandbox = Sandbox::prepare(source_parent, workspace.path(), &program_path)?;
    let stderr_path = workspace.path().join("renderer.stderr");
    let stderr_file = std::fs::File::create(&stderr_path)
        .map_err(|e| format!("could not prepare renderer log: {e}"))?;
    let mut command = Command::new(&program_path);
    command
        .args(&args)
        .current_dir(workspace.path())
        .env_clear()
        .env("PATH", path)
        .env("LANG", "C.UTF-8")
        .env("HOME", workspace.path())
        .env("TMPDIR", workspace.path())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::from(stderr_file));
    #[cfg(target_os = "linux")]
    unsafe {
        command.pre_exec(move || sandbox.apply());
    }
    let mut child = command
        .spawn()
        .map_err(|e| format!("could not start renderer '{program}': {e}"))?;
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    let stderr = std::fs::read_to_string(&stderr_path).unwrap_or_default();
                    return Err(format!("renderer exited with {status}: {}", stderr.trim()));
                }
                break;
            }
            Ok(None) if started.elapsed() < timeout => {
                std::thread::sleep(Duration::from_millis(25))
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "renderer exceeded the {} second timeout",
                    timeout.as_secs()
                ));
            }
            Err(e) => return Err(format!("could not monitor renderer: {e}")),
        }
    }
    if !output.is_file() {
        return Err("renderer completed without creating the requested PDF".into());
    }
    Ok(RenderedPdf {
        path: output,
        engine: label,
        _workspace: Some(workspace),
    })
}

fn resolve_program(program: &str, path: &std::ffi::OsStr) -> Result<PathBuf, String> {
    let candidate = Path::new(program);
    if candidate.components().count() > 1 {
        return candidate
            .canonicalize()
            .map_err(|e| format!("could not resolve renderer '{program}': {e}"));
    }
    for directory in std::env::split_paths(path) {
        let candidate = directory.join(program);
        if candidate.is_file() {
            return candidate
                .canonicalize()
                .map_err(|e| format!("could not resolve renderer '{program}': {e}"));
        }
    }
    Err(format!("could not find renderer '{program}' on PATH"))
}

pub fn existing(path: &Path) -> Result<RenderedPdf, String> {
    let path = path
        .canonicalize()
        .map_err(|e| format!("could not resolve PDF {}: {e}", path.display()))?;
    Ok(RenderedPdf {
        path,
        engine: "existing-pdf".into(),
        _workspace: None,
    })
}
