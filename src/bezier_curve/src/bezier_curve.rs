// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

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
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;

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
        #[qproperty(QPointF, p1)]
        #[qproperty(QPointF, p2)]
        #[qproperty(QPointF, p3)]
        #[qproperty(QPointF, p4)]
        #[qproperty(i32, segmentCount, rust_name = "segment_count")]
        type BezierCurve = super::BezierCurveRust;

        #[inherit]
        #[rust_name = "set_flag"]
        fn setFlag(self: Pin<&mut BezierCurve>, flag: QQuickItemFlag, enabled: bool);

        #[inherit]
        fn update(self: Pin<&mut BezierCurve>);

        #[inherit]
        fn size(self: &BezierCurve) -> QSizeF;

        #[cxx_override]
        #[cxx_name = "updatePaintNode"]
        unsafe fn update_paint_node(
            self: Pin<&mut BezierCurve>,
            old_node: *mut QSGNode,
            update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
        ) -> *mut QSGNode;
    }

    unsafe extern "C++" {
        include!("bezier_curve.h");

        /// cpp implementation of the update_paint_node function
        ///
        /// ## Safety
        ///
        /// The function takes ownership of the `old_node` pointer and returns a new one.
        /// The caller is responsible for managing the memory of the returned pointer.
        #[allow(clippy::missing_safety_doc)] // <- Somehow false positive
        unsafe fn bezier_curve_update_paint_node(
            old_node: *mut QSGNode,
            item_size: &QSizeF,
            p1: &QPointF,
            p2: &QPointF,
            p3: &QPointF,
            p4: &QPointF,
            segmentCount: i32,
        ) -> *mut QSGNode;
    }

    impl cxx_qt::Initialize for BezierCurve {}
}

use core::pin::Pin;

use cxx_qt_lib::QPointF;
use ffi::{QQuickItemFlag, QQuickItemUpdatePaintNodeData, QSGNode};

pub struct BezierCurveRust {
    p1: QPointF,
    p2: QPointF,
    p3: QPointF,
    p4: QPointF,
    segment_count: i32,
}

impl Default for BezierCurveRust {
    fn default() -> Self {
        Self {
            p1: QPointF::new(0.0, 0.0),
            p2: QPointF::new(1.0, 0.0),
            p3: QPointF::new(0.0, 1.0),
            p4: QPointF::new(1.0, 1.0),
            segment_count: 32,
        }
    }
}

impl cxx_qt::Initialize for ffi::BezierCurve {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut()
            .set_flag(QQuickItemFlag::ItemHasContents, true);

        self.as_mut()
            .on_p1_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_p2_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_p3_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_p4_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_segment_count_changed(|qobject| qobject.update())
            .release();
    }
}

impl ffi::BezierCurve {
    unsafe fn update_paint_node(
        self: Pin<&mut Self>,
        old_node: *mut QSGNode,
        _update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
    ) -> *mut QSGNode {
        unsafe {
            ffi::bezier_curve_update_paint_node(
                old_node,
                &self.size(),
                self.p1(),
                self.p2(),
                self.p3(),
                self.p4(),
                *self.segment_count(),
            )
        }
    }
}
