fn main() {
    if cfg!(target_os = "windows") {
        let _ = embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
