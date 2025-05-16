use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Quick")
        .qt_module("Quick3D")
        .qml_module(QmlModule {
            uri: "com.oliv.gizmo",
            rust_files: &["src/gizmo.rs"],
            qml_files: &["./qml/main.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            cc.include("./cpp");
            cc.file("./cpp/gizmo.cpp");

            println!("cargo:rerun-if-changed=./cpp/gizmo.h");
            println!("cargo:rerun-if-changed=./cpp/gizmo.cpp");
        })
        .qobject_header("./cpp/gizmo.h")
        .build();
}
