use std::collections::HashMap;

// 编译期嵌入，运行时无需外部文件
const CLSIDS_JSON: &str = include_str!("../clsids.json");
const VERSION_INI: &str = include_str!("../version.ini");

pub struct Entry {
    pub clsid: String,
    pub name: String,
}

pub fn load_entries() -> Vec<Entry> {
    parse_entries(CLSIDS_JSON)
}

// 只认 "clsid" / "name" 两个键，足够解析本项目的数据文件
fn parse_entries(json: &str) -> Vec<Entry> {
    let tokens = quoted_tokens(json);
    let mut list = Vec::new();
    let mut clsid: Option<String> = None;
    let mut i = 0;
    while i < tokens.len() {
        let key = tokens[i].as_str();
        if key == "clsid" && i + 1 < tokens.len() {
            clsid = Some(tokens[i + 1].clone());
            i += 2;
            continue;
        }
        if key == "name" && i + 1 < tokens.len() {
            if let Some(c) = clsid.take() {
                list.push(Entry { clsid: c, name: tokens[i + 1].clone() });
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    list
}

fn quoted_tokens(json: &str) -> Vec<String> {
    let chars: Vec<char> = json.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '"' {
            i += 1;
            continue;
        }
        i += 1;
        let mut buf = String::new();
        while i < chars.len() {
            match chars[i] {
                '\\' if i + 1 < chars.len() => {
                    buf.push(chars[i + 1]);
                    i += 2;
                }
                '"' => {
                    i += 1;
                    break;
                }
                c => {
                    buf.push(c);
                    i += 1;
                }
            }
        }
        out.push(buf);
    }
    out
}

// 优先读 exe 同目录的 version.ini，缺失时回退到编译期内置副本
pub fn app_info() -> (String, String) {
    let text = external_ini().unwrap_or_else(|| VERSION_INI.to_string());
    let map = parse_ini(&text);
    let name = map.get("name").filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "CLSIDShellFolderCreator".to_string());
    let version = map.get("version").filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| "0.0.0".to_string());
    (name, version)
}

fn external_ini() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    std::fs::read_to_string(exe.parent()?.join("version.ini")).ok()
}

fn parse_ini(text: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_are_complete() {
        let list = load_entries();
        assert_eq!(list.len(), 160);
        assert_eq!(list[0].name, "上帝模式");
        assert_eq!(list[0].clsid, ".{ED7BA470-8E54-465E-825C-99712043E01C}");
        assert_eq!(list[159].clsid, ".{FF393560-C2A7-11CF-BFF4-444553540000}");
        assert!(list.iter().all(|e| e.clsid.starts_with(".{")));
    }

    #[test]
    fn ini_ignores_spaces() {
        let map = parse_ini("name = Demo \r\nversion= 1.2.3 \r\n");
        assert_eq!(map["name"], "Demo");
        assert_eq!(map["version"], "1.2.3");
    }
}
