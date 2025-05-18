# `cxx-qt` experiments

This repository contains various experiments with new `cxx-qt` library. It helps me to learn how to use `cxx-qt`, what are its limitations, and how to use it for more complex examples than the ones in the official documentation.

## Prerequisites

This repository uses nix to manage dependencies. You need to have nix installed on your system. You can install it by following the instructions on the [nix website](https://nixos.org/download.html).

## Examples

### `bezier_curve`

Shows how to implement a custom geometry in the Qt Quick Scene Graph.

The custom geometry example shows how to create a QQuickItem that uses the scene graph API to build a custom geometry for the scene graph. It does this by creating a BezierCurve item, which is made part of the CustomGeometry module and makes use of this in a QML file.

![bezier_curve_img](https://doc.qt.io/qt-6/images/custom-geometry-example.png)

To run the example, use the following command:

```bash
nix develop . --command cargo run -p bezier_curve
```

More information about how I re-implemented the example can be found in the [bezier_curve](src/bezier_curve/README.md) directory.

### `gizmo`

`QQuickItem` gizmo using `transform-gizmo` crate.

`transform-gizmo` is a framework-agnostic Rust crate that provides a feature-rich and customizable 3D transformation gizmo for manipulating the position, rotation and scale of 3D entities.

![video](https://github.com/user-attachments/assets/66f1b48b-eded-40c0-907f-6f86ebe89e0e)

To run the example, use the following command:

```bash
nix develop . --command cargo run -p gizmo
```

More information about how I implemented can be found in the [gizmo](src/gizmo/README.md) directory.
