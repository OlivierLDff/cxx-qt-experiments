// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

use core::pin::Pin;

use ffi::{QQuickItemFlag, QQuickItemUpdatePaintNodeData, QSGNode};

#[cxx_qt::bridge]
pub mod ffi {
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QQuickItemFlag {
        /// Indicates this item should visually clip its children so that they are rendered only
        /// within the boundaries of this item.
        ItemClipsChildrenToShape = 0x01,
        /// Indicates the item supports text input methods.
        ItemAcceptsInputMethod = 0x02,
        /// Indicates the item is a focus scope. See Keyboard Focus in Qt Quick for more
        /// information.
        ItemIsFocusScope = 0x04,
        /// Indicates the item has visual content and should be rendered by the scene graph.
        ItemHasContents = 0x08,
        /// Indicates the item accepts drag and drop events.
        ItemAcceptsDrops = 0x10,
        /// Indicates that the item defines a viewport for its children.
        ItemIsViewport = 0x20,
        /// Indicates that the item wishes to know the viewport bounds when any ancestor has the
        /// ItemIsViewport flag set.
        ItemObservesViewport = 0x40,
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxxqtlib1_qquickitem.h");
        type QQuickItemFlag;
        type QQuickItemUpdatePaintNodeData;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;

        include!(<QtQuick/QQuickItem>);
        type QQuickItem;
    }

    unsafe extern "C++" {
        include!(<QtQuick/QSGNode>);
        type QSGNode;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QQuickItem]
        type Gizmo = super::GizmoRust;

        #[inherit]
        #[rust_name = "set_flag"]
        fn setFlag(self: Pin<&mut Gizmo>, flag: QQuickItemFlag, enabled: bool);

        #[inherit]
        fn update(self: Pin<&mut Gizmo>);

        #[inherit]
        fn size(self: &Gizmo) -> QSizeF;

        #[cxx_override]
        #[cxx_name = "updatePaintNode"]
        unsafe fn update_paint_node(
            self: Pin<&mut Gizmo>,
            old_node: *mut QSGNode,
            update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
        ) -> *mut QSGNode;
    }

    impl cxx_qt::Initialize for Gizmo {}
}

#[derive(Debug, Default)]
pub struct GizmoRust {}

impl cxx_qt::Initialize for ffi::Gizmo {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut()
            .set_flag(QQuickItemFlag::ItemHasContents, true);
    }
}

impl ffi::Gizmo {
    unsafe fn update_paint_node(
        self: Pin<&mut Self>,
        old_node: *mut QSGNode,
        _update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
    ) -> *mut QSGNode {
        old_node
    }
}
