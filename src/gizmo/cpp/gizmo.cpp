// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

#include <QtGui/QVector3D>
#include <QtGui/QVector4D>
#include <QtQuick/QQuickItem>
#include <QtQuick/QSGGeometryNode>
#include <QtQuick/QSGGeometry>
#include <QtQuick/QSGVertexColorMaterial>
#include <QtQuick/QSGNode>

#include "gizmo.h"

QSGNode *gizmo_update_paint_node(QSGNode *oldNode,
                                 rust::Slice<std::array<float, 2> const> vertices,
                                 rust::Slice<std::array<float, 4> const> colors,
                                 rust::Slice<std::uint32_t const> indices)
{
    assert(vertices.size() == colors.size());

    if (vertices.empty())
    {
        assert(indices.empty());
        if (oldNode)
            delete oldNode;

        return nullptr;
    }

    QSGGeometryNode *node = nullptr;
    QSGGeometry *geometry = nullptr;

    if (!oldNode)
    {
        node = new QSGGeometryNode;
        geometry = new QSGGeometry(QSGGeometry::defaultAttributes_ColoredPoint2D(), vertices.size(), indices.size(), QSGGeometry::UnsignedIntType);
        geometry->setDrawingMode(QSGGeometry::DrawTriangles);
        node->setGeometry(geometry);
        node->setFlag(QSGNode::OwnsGeometry);
        node->setMaterial(new QSGVertexColorMaterial);
        node->setFlag(QSGNode::OwnsMaterial);
    }
    else
    {
        node = static_cast<QSGGeometryNode *>(oldNode);
        geometry = node->geometry();
        geometry->allocate(vertices.size(), indices.size());
    }

    assert(geometry != nullptr);

    QSGGeometry::ColoredPoint2D *vertex_data = geometry->vertexDataAsColoredPoint2D();

    for (std::size_t i = 0; i < vertices.size(); ++i)
    {
        const auto &v = vertices[i];
        const auto &c = colors[i];
        const auto x = v[0];
        const auto y = v[1];
        const auto red = static_cast<uchar>(c[0] * 255.f);
        const auto green = static_cast<uchar>(c[1] * 255.f);
        const auto blue = static_cast<uchar>(c[2] * 255.f);
        const auto alpha = static_cast<uchar>(c[3] * 255.f);

        vertex_data[i].set(x, y, red, green, blue, alpha);
    }

    uint *index_data = geometry->indexDataAsUInt();
    for (std::size_t i = 0; i < indices.size(); ++i)
    {
        index_data[i] = indices[i];
        assert(index_data[i] < vertices.size());
    }

    node->markDirty(QSGNode::DirtyGeometry);
    return node;
}
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
