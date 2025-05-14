# Scene Graph - Custom Geometry

Shows how to implement a custom geometry in the Qt Quick Scene Graph.

The custom geometry example shows how to create a QQuickItem that uses the scene graph API to build a custom geometry for the scene graph. It does this by creating a BezierCurve item, which is made part of the CustomGeometry module and makes use of this in a QML file.

> [!NOTE]
> This readme is a based on the [original example](https://doc.qt.io/qt-6/qtquick-scenegraph-customgeometry-example.html#beziercurve-implementation).
>
> - I recommend for you to first follow the original example, then follow this one to see how I re-implemented it using `cxx-qt`.
> - You should be familiar with `cxx` and followed the tutorials in the [cxx-qt documentation](https://cxx.rs/#example).
> - You should have followed the [cxx-qt getting started section](https://kdab.github.io/cxx-qt/book/getting-started/index.html) to understand shorcut I'm taking.

![bezier_curve_img](https://doc.qt.io/qt-6/images/custom-geometry-example.png)

## BezierCurve declaration

In [`src/bezier_curve.rs`](src/bezier_curve.rs), we declare the `BezierCurve` class. This class is a `QQuickItem` derived class.

```rust
#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;

        include!(<QtQuick/QQuickItem>);
        type QQuickItem;
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
    }
}

use cxx_qt_lib::QPointF;

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
```

As of version `0.7.x` of `cxx-qt`, no `QQuickItem` exists. This is why we import `#include!(<QtQuick/QQuickItem>);` in the `unsafe extern "C++"` block. This is a workaround until `cxx-qt` supports `QQuickItem` directly.

By itself it won't compile, because the constructor of `QQuickItem` expects a `QQuickItem*` and not a `QObject*`.

This can be dealt with in different ways, either using `cxx_qt::Constructor` trait or `cxx_qt::Initialize` trait. More can be found in the [Traits section](https://kdab.github.io/cxx-qt/book/bridge/traits.html).

We will go with `cxx_qt::Initialize` trait, because we also need to initialize some stuff:

- set the `ItemHasContents` flag
- connect the `p1`, `p2`, `p3`, `p4` and `segment_count` properties to the `update()` method of the `QQuickItem`.

```rust
#[cxx_qt::bridge]
pub mod ffi {
    // ... other code
    impl cxx_qt::Initialize for BezierCurve {}
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
```

And here we start to hit the limitations of `cxx-qt`. But it's ok we can deal with it ourself. We need to be able to call:

- `set_flag` method of `QQuickItem` to set the `ItemHasContents` flag and access the associated enum.
- Call the `update()` method of `QQuickItem` to update the item when the properties change.

All the code shouldn't be required anymore if `cxx-qt` supports `QQuickItem` directly and with the new `upcast` feature they are working on.

```rust
#[cxx_qt::bridge]
pub mod ffi {
    // ... other code

    unsafe extern "RustQt" {
        #[inherit]
        #[rust_name = "set_flag"]
        fn setFlag(self: Pin<&mut BezierCurve>, flag: QQuickItemFlag, enabled: bool);

        #[inherit]
        fn update(self: Pin<&mut BezierCurve>);
    }
}
```

So far so good, we use the [`inherit`](https://kdab.github.io/cxx-qt/book/bridge/extern_rustqt.html?highlight=inherit#inheritance) attribute to inherit the `setFlag` and `update` methods from `QQuickItem`. This allows us to call them directly on our `BezierCurve` object. The correct way in the future will be to upcast our `BezierCurve` object to `QQuickItem` and call the methods on it.

But what about `QQuickItemFlag` that I wrote? In c++ this is `QQuickItem::Flag`, but we can't directly access it in rust. I followed what I've seen in the `cxx-qt` codebase, and used the [`cxx documentation about extern enum`](https://cxx.rs/shared.html#extern-enums). We want to use an enum that already exists in c++ world, not create a new one.

So in `cpp/cxxqtlib1_qquickitem.h` we have:

```cpp
#pragma once
#include <QtQuick/QQuickItem>
namespace rust::cxxqtlib1
{
    using QQuickItemFlag = QQuickItem::Flag;
}
```

The namespace doesn't matter, but I used the one `cxx-qt` uses in it's codebase. Then let's use it in our code:

```rust
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
    }
}
```

Of course don't forget to update `build.rs` to include the new header file:

```rust
use cxx_qt_build::{CxxQtBuilder, QmlModule};
fn main() {
    CxxQtBuilder::new()
        // ... stuff
        .cc_builder(|cc| {
            cc.include("./cpp");
        })
        .build();
}
```

## Overriding the paint method

From now on, I will take a shortcut of not porting the whole `updatePaintNode` method to rust, as this is no longer a `cxx-qt` usage, but more a `cxx` usage. Starting to work on bindings for the whole `QSG*` family of classes is out of scope of this example.

I believe the `cxx-qt` could do gradual work on adding new API, but I guess a lot of discussion should go on because they are many way to approach the problem (as with any bindings library).

Let's override the `updatePaintNode` method of `QQuickItem` to use our own `BezierCurve` class. Note that you will see `QQuickItemUpdatePaintNodeData` in the code, I used the same alias technique as for `QQuickItemFlag`.

```rust
#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!(<QtQuick/QSGNode>);
        type QSGNode;
    }

    unsafe extern "RustQt" {
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
}

impl ffi::BezierCurve {
    unsafe fn update_paint_node(
        self: Pin<&mut Self>,
        old_node: *mut QSGNode,
        update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
    ) -> *mut QSGNode {
        unsafe {
            return ffi::bezier_curve_update_paint_node(
                old_node,
                &self.size(),
                self.p1(),
                self.p2(),
                self.p3(),
                self.p4(),
                *self.segment_count(),
            );
        }
    }
}
```

And `bezier_curve.h` mostly taken from the original example.

```cpp
#pragma once

#include <QtCore/QSizeF>
#include <QtCore/QPointF>
#include <QtQuick/QQuickItem>
#include <QtQuick/QSGGeometryNode>
#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGFlatColorMaterial>
#include <QtQuick/QSGNode>

QSGNode *bezier_curve_update_paint_node(QSGNode *oldNode,
                                        QSizeF const &itemSize,
                                        QPointF const &p1,
                                        QPointF const &p2,
                                        QPointF const &p3,
                                        QPointF const &p4,
                                        std::int32_t segmentCount)
{
    QSGGeometryNode *node = nullptr;
    QSGGeometry *geometry = nullptr;

    if (!oldNode)
    {
        node = new QSGGeometryNode;
        geometry = new QSGGeometry(QSGGeometry::defaultAttributes_Point2D(), segmentCount);
        geometry->setLineWidth(2);
        geometry->setDrawingMode(QSGGeometry::DrawLineStrip);
        node->setGeometry(geometry);
        node->setFlag(QSGNode::OwnsGeometry);
        auto *material = new QSGFlatColorMaterial;
        material->setColor(QColor(255, 0, 0));
        node->setMaterial(material);
        node->setFlag(QSGNode::OwnsMaterial);
    }
    else
    {
        node = static_cast<QSGGeometryNode *>(oldNode);
        geometry = node->geometry();
        geometry->allocate(segmentCount);
    }

    geometry = node->geometry();

    QSGGeometry::Point2D *vertices = geometry->vertexDataAsPoint2D();
    for (int i = 0; i < segmentCount; ++i)
    {
        qreal t = i / qreal(segmentCount - 1);
        qreal invt = 1 - t;

        QPointF pos = invt * invt * invt * p1 + 3 * invt * invt * t * p2 + 3 * invt * t * t * p3 + t * t * t * p4;

        float x = pos.x() * itemSize.width();
        float y = pos.y() * itemSize.height();

        vertices[i].set(x, y);
    }
    node->markDirty(QSGNode::DirtyGeometry);

    return node;
}
```

And there we should have everything!
