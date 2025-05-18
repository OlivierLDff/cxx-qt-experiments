# Transform Gizmo

`transform-gizmo` is a framework-agnostic Rust crate that provides a feature-rich and customizable 3D transformation gizmo for manipulating the position, rotation and scale of 3D entities.

I will split my feedback in 3 parts:

- doing the rendering (`updatePaintNode`)
- interaction with the mouse/hovering
- multi-selection/feedback API for qml

## Rendering via `updatePaintNode`

The crate have a simple `draw` function that return a [`GizmoDrawData`](https://docs.rs/transform-gizmo/latest/transform_gizmo/gizmo/struct.GizmoDrawData.html):

```rust
pub struct GizmoDrawData {
    pub vertices: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
}
```

We need to render that using QSG API. Like in my previous test ([bezier curve](https://github.com/KDAB/cxx-qt/discussions/1270)) I went with interacting from C++ as they are no bindings for that. As I found before I don't know if doing 1:1 mapping brings anything to the table.

```rust
    unsafe extern "C++" {
include!("gizmo.h");
unsafe fn gizmo_update_paint_node(
    old_node: *mut QSGNode,
    vertices: &[[f32; 2]],
    colors: &[[f32; 4]],
    indices: &[u32],
) -> *mut QSGNode;
}
```

And this call the following cpp code:

```cpp
QSGNode *gizmo_update_paint_node(QSGNode *oldNode,
                                 rust::Slice<std::array<float, 2> const> vertices,
                                 rust::Slice<std::array<float, 4> const> colors,
                                 rust::Slice<std::uint32_t const> indices)
```

Then this is just regular Qt with C++. I've found myself a bit in uncharted water because they are not much online example/documentation on how to create something else than bezier curve and [Efficient custom shapes in Qt Quick](https://www.kdab.com/efficient-custom-shapes-in-qt-quick/). But I guess this is mainly my lack of knowledge of graphic programming.

## Interaction with the Mouse

The `transform_gizmo` crate expect a [`GizmoInteraction`](https://docs.rs/transform-gizmo/latest/transform_gizmo/gizmo/struct.GizmoInteraction.html) to describe the interaction:

```rust
pub struct GizmoInteraction {
    pub cursor_pos: (f32, f32),
    pub hovered: bool,
    pub drag_started: bool,
    pub dragging: bool,
}
```

So I wanted to have interaction with the mouse click/move/hover information. For that I needed to inherit 6 functions and manipulate `QMouseEvent*` & `QHoverEvent*`. I didn't know what I needed before hand so trying to solve making new bindings and figuring out what needed to be done was too much at once.
I went with the same trick as before: writing the stuff in C++ that doesn't bring much value, and call the logic written in rust.

So my base class:

```cpp
class GizmoInteractionItem : public QQuickItem
{
    Q_OBJECT

public:
    GizmoInteractionItem(QQuickItem *parent = nullptr)
        : QQuickItem(parent)
    {
        setAcceptHoverEvents(true);
        setAcceptedMouseButtons(Qt::LeftButton);
    }
    virtual ~GizmoInteractionItem() = default;

protected:
    void hoverEnterEvent(QHoverEvent *event) override;
    void hoverLeaveEvent(QHoverEvent *event) override;
    void hoverMoveEvent(QHoverEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    void mouseReleaseEvent(QMouseEvent *event) override;
    void mouseMoveEvent(QMouseEvent *event) override;

    virtual void updateInteraction(QPointF position, bool hovered, bool dragStarted, bool dragging) = 0;
    virtual bool pickPreview(QPointF position) = 0;
};
```

This make life so much easier since I just need to "inherit" from rust `updateInteraction` and `pickPreview` (that check if we are hovering a clickable part of the gizmo).

```rust
#[cxx_override]
#[cxx_name = "updateInteraction"]
fn update_interaction(
    self: Pin<&mut Gizmo>,
    cursor_position: QPointF,
    hovered: bool,
    drag_started: bool,
    dragging: bool,
);

#[cxx_override]
#[cxx_name = "pickPreview"]
fn pick_preview(self: Pin<&mut Gizmo>, cursor_position: QPointF) -> bool;
```

I'm not sure what is the best way to provide bindings for `QMouseEvent` and `QHoverEvent`, but I'm sure the new upcast feature will help.

## Api for Targets Manipulation

That part is very opinionated. The role of the gizmo is to update translation/rotation/scale of 1 or N objects. At first I thought about doing some `QQmlListProperty` or something using `QObject` for each targe that need to be manipulated. But that sounded like a hassle so I went with something way simpler to avoid any binding: `QVariant`, or more precisely a `QList<QMap<QString, QVariant>>`.

So user could just pass a list of js object, and get back a list of update object.

But somehow I struggle with lots of cryptic error message, I would really like your feedback on how to write that using only rust because I think this is possible with what is in `cxx-qt`.

Since I had trouble with `QVariant` usage from rust I went the C++ route again:

```rust
    unsafe extern "C++" {
        include!("gizmo.h");
        fn extract_target_count_from_qvariant(targets: &QVariant) -> usize;

        fn extract_targets_from_qvariant(
            targets: &QVariant,
            positions: &mut [QVector3D],
            rotations: &mut [QVector4D],
            scales: &mut [QVector3D],
        );

        fn transforms_to_qvariant(
            positions: &[QVector3D],
            rotations: &[QVector4D],
            scales: &[QVector3D],
        ) -> QVariant;
    }
```

And the code I would love to be able to write in rust:

```cpp
std::size_t extract_target_count_from_qvariant(const QVariant &targets)
{
    const auto targetsList = targets.toList();
    return targetsList.size();
}

void extract_targets_from_qvariant(const QVariant &targets, rust::Slice<QVector3D> positions, rust::Slice<QVector4D> rotations, rust::Slice<QVector3D> scales)
{
    const auto targetsList = targets.toList();

    assert(std::size_t(targetsList.size()) == positions.size());
    assert(std::size_t(targetsList.size()) == rotations.size());
    assert(std::size_t(targetsList.size()) == scales.size());

    for (int i = 0; i < targetsList.size(); ++i)
    {
        const auto target = targetsList.at(i);

        const QMap<QString, QVariant> targetMap = target.toMap();
        const QVariant position = targetMap.value("position", QVector3D());
        const QVariant rotation = targetMap.value("rotation", QVector4D(0.f, 0.f, 0.f, 1.f));
        const QVariant scale = targetMap.value("scale", QVector3D(1.f, 1.f, 1.f));

        positions[i] = qvariant_cast<QVector3D>(position);
        rotations[i] = qvariant_cast<QVector4D>(rotation);
        scales[i] = qvariant_cast<QVector3D>(scale);
    }
}

QVariant transforms_to_qvariant(rust::Slice<QVector3D const> positions, rust::Slice<QVector4D const> rotations, rust::Slice<QVector3D const> scales)
{
    assert(positions.size() == rotations.size());
    assert(positions.size() == scales.size());

    QVector<QVariant> transforms;
    for (std::size_t i = 0; i < positions.size(); ++i)
    {
        QMap<QString, QVariant> transform;

        transform.insert("position", positions[i]);
        transform.insert("rotation", rotations[i]);
        transform.insert("scale", scales[i]);

        transforms.emplace_back(transform);
    }

    return transforms;
}
```

Also I wanted to use `CxxVector` or `rust::Vector` but somehow this gave me linking error about a missing cxx symbol. Is qt cxx bridge linking to cxx library?

## What Have I Been Missing from `cxx-qt`

- `QVariant` examples
- `QQuaternion` for rotation bindings, I went with `QVector4D`
- `QVariant` with `QtGui` types (`QVector3D`, `QVector4D`)

I don't know if there is a right way to bind all the QSG & QEvent object, so manipulating them from C++ is okish.

I've also seen there is a `qt-lib-extra`crate so I will see if I can contribute some work for `QQuickItem`.
