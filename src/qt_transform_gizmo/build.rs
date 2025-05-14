use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Quick")
        .qt_module("Quick3D")
        .qml_module(QmlModule {
            uri: "com.oliv.transform_gizmo",
            rust_files: &["src/transform_gizmo.rs"],
            qml_files: &["./qml/main.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            cc.include("./cpp");
            // cc.file("./cpp/transform_gizmo.cpp");
        })
        .build();
}
