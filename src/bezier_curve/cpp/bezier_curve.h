// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

// Code taken from: https://doc.qt.io/qt-6/qtquick-scenegraph-customgeometry-example.html#beziercurve-implementation

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
                                        std::int32_t segmentCount);
