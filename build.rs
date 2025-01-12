use std::{fmt::Write, fs, io::Write as IoWrite, path::Path};

const PACKETS_FEATURES_BEGIN: &str = "# inject.packet-features.begin";
const PACKETS_FEATURES_END: &str = "# inject.packet-features.end";

fn main() {
    let mut packets = Vec::new();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/packets");
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            packets.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(Path::new("/tmp").join(format!(
            "ccanvas-packets-index-{}.txt",
            env!("CARGO_PKG_VERSION").replace(".", "-")
        )))
        .unwrap()
        .write_all(
            packets
                .iter()
                .map(|item| format!("{item}={}", get_id(&path.join(item).join("mod.rs"))))
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();

    if std::env::var("PROFILE").as_ref().map(String::as_str) == Ok("debug") {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let toml = fs::read_to_string(&path).unwrap();

        let (before, after) = toml.split_once(PACKETS_FEATURES_BEGIN).unwrap();
        let (_, after) = after.split_once(PACKETS_FEATURES_END).unwrap();

        let features = packets.iter().fold(String::new(), |mut acc, feature| {
            writeln!(acc, "{feature} = []").unwrap();
            acc
        });

        fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(path)
            .unwrap()
            .write_all(
                format!(
                    "{before}{PACKETS_FEATURES_BEGIN}\n{features}{PACKETS_FEATURES_END}{after}"
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

fn get_id(path: &Path) -> String {
    let mut content = String::new();
    let mut consec_slashes: u8 = 0;
    let file = fs::read_to_string(path).unwrap();

    for (i, c) in file.chars().enumerate() {
        match c {
            '\n' => consec_slashes = 0,
            '/' => consec_slashes += 1,
            ' ' => continue,
            c if matches!(consec_slashes, 0 | 1) => content.push(c),
            _ => {}
        }

        if content.ends_with("group_id(") {
            return file[i + 1..]
                .split_once(")")
                .unwrap()
                .0
                .chars()
                .filter(|c| c != &'\n')
                .collect();
        }
    }

    panic!("not found")
}
