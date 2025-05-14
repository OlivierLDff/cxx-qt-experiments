use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Quick")
        .qml_module(QmlModule {
            uri: "com.oliv.bezier_curve",
            rust_files: &["src/bezier_curve.rs"],
            qml_files: &["./qml/main.qml"],
            ..Default::default()
        })
        .cc_builder(|cc| {
            cc.include("./cpp");
            cc.file("./cpp/bezier_curve.cpp");
        })
        .build();
}
