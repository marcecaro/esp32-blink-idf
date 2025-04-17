fn main() {
    embuild::cargo::CfgBuilder::new()
        .idf_env()   // read EXTRA_COMPONENT_DIRS, IDF_PATH, sdkconfig, …
        .probe().unwrap();
}
