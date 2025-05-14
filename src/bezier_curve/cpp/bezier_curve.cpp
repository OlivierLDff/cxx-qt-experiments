// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

// Code taken from: https://doc.qt.io/qt-6/qtquick-scenegraph-customgeometry-example.html#beziercurve-implementation

#include <QtCore/QSizeF>
#include <QtCore/QPointF>
#include <QtQuick/QQuickItem>
#include <QtQuick/QSGGeometryNode>
#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGFlatColorMaterial>
#include <QtQuick/QSGNode>

#include "bezier_curve.h"

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
