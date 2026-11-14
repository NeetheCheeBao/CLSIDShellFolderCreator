use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=version.ini");
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "CLSIDShellFolderCreator".to_string());

    // 把 ico 解成裸 RGBA 交给程序，运行时不必带图片解码器
    write_icon(&root, &out);

    // 找不到 rc.exe 就跳过资源，不影响程序功能
    let rc = match find_rc() {
        Some(p) => p,
        None => {
            println!("cargo:warning=rc.exe not found, skip icon resource");
            return;
        }
    };

    let manifest = out.join("app.manifest");
    if fs::write(&manifest, manifest_xml(&name)).is_err() {
        return;
    }

    let rc_file = out.join("app.rc");
    let icon = root.join("assets").join("icon.ico");
    // 只写图标与 manifest，不注入版本信息
    let body = format!(
        "1 ICON \"{icon}\"\n1 24 \"{manifest}\"\n",
        icon = esc(&icon),
        manifest = esc(&manifest)
    );
    if fs::write(&rc_file, body).is_err() {
        return;
    }

    let res = out.join("app.res");
    let status = Command::new(&rc)
        .arg("/nologo")
        .arg("/fo")
        .arg(&res)
        .arg(&rc_file)
        .status();
    match status {
        Ok(s) if s.success() => println!("cargo:rustc-link-arg={}", res.display()),
        _ => println!("cargo:warning=rc.exe failed, skip icon and version resource"),
    }
}

fn esc(p: &Path) -> String {
    p.display().to_string().replace('\\', "\\\\")
}

// 输出 [宽:u32][高:u32][RGBA...]，程序端 include_bytes! 直接读
fn write_icon(root: &Path, out: &Path) {
    let Ok(bytes) = fs::read(root.join("assets").join("icon.ico")) else {
        println!("cargo:warning=icon.ico not found");
        return;
    };
    let Ok(img) = image::load_from_memory_with_format(&bytes, image::ImageFormat::Ico) else {
        println!("cargo:warning=icon.ico decode failed");
        return;
    };
    // 窗口图标 64px 足够，避免把 256px 的裸 RGBA 塞进 exe
    let img = if img.width() > 64 {
        img.resize_exact(64, 64, image::imageops::FilterType::Triangle)
    } else {
        img
    };
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let mut buf = Vec::with_capacity(8 + rgba.len());
    buf.extend_from_slice(&w.to_le_bytes());
    buf.extend_from_slice(&h.to_le_bytes());
    buf.extend_from_slice(&rgba);
    let _ = fs::write(out.join("icon.bin"), buf);
}

fn manifest_xml(name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity type="win32" name="{name}" version="1.0.0.0" processorArchitecture="*"/>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*"/>
    </dependentAssembly>
  </dependency>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
    </windowsSettings>
  </application>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <supportedOS Id="{{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}}"/>
    </application>
  </compatibility>
</assembly>
"#
    )
}

fn find_rc() -> Option<PathBuf> {
    if let Ok(p) = env::var("RC_EXE") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    let mut roots = Vec::new();
    for key in ["WindowsSdkDir", "ProgramFiles(x86)", "ProgramFiles"] {
        if let Ok(v) = env::var(key) {
            roots.push(PathBuf::from(v).join("Windows Kits").join("10").join("bin"));
        }
    }
    roots.push(PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\bin"));
    roots.push(PathBuf::from(r"C:\Program Files\Windows Kits\10\bin"));
    roots.push(PathBuf::from(r"D:\Windows Kits\10\bin"));

    let mut best: Option<(u64, PathBuf)> = None;
    for root in roots {
        let Ok(entries) = fs::read_dir(&root) else { continue };
        for entry in entries.flatten() {
            let dir = entry.path();
            let candidate = dir.join("x64").join("rc.exe");
            if !candidate.is_file() {
                continue;
            }
            // 版本目录名形如 10.0.26100.0，取最高的一个
            let weight = dir
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.split('.').filter_map(|x| x.parse::<u64>().ok()).fold(0u64, |a, b| a * 1000 + b))
                .unwrap_or(0);
            if best.as_ref().map(|(w, _)| weight > *w).unwrap_or(true) {
                best = Some((weight, candidate));
            }
        }
    }
    best.map(|(_, p)| p)
}
