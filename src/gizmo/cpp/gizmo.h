// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

#pragma once

#include <QtQuick/QSGNode>
#include <QtQuick/QQuickItem>
#include <array>
#include <cstdint>

#include "rust/cxx.h"

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

protected:
    void hoverEnterEvent(QHoverEvent *event) override
    {
        assert(_hovering == false);

        if (pickPreview(event->position()))
        {
            event->accept();
            _hovering = true;
            callUpdateInteraction(event->position());
        }
        else
        {
            event->ignore();
        }
    }

    void hoverLeaveEvent(QHoverEvent *event) override
    {
        if (_hovering)
        {
            event->accept();
            _hovering = false;
            callUpdateInteraction(event->position());
        }
        else
        {
            event->ignore();
        }
    }

    void hoverMoveEvent(QHoverEvent *event) override
    {
        // This might be required if an object grab the mouse
        _dragging = false;
        if (pickPreview(event->position()))
        {
            event->accept();
            _hovering = true;
            callUpdateInteraction(event->position());
        }
        else if (_hovering)
        {
            event->accept();
            _hovering = false;
            callUpdateInteraction(event->position());
        }
        else
        {
            event->ignore();
        }
    }

    void mousePressEvent(QMouseEvent *event) override
    {
        assert(_dragging == false);

        if (pickPreview(event->position()))
        {
            event->accept();

            _dragging = true;
            callUpdateInteraction(event->position(), true);
            setKeepMouseGrab(true);
        }
        else
        {
            event->ignore();
        }
    }

    void mouseReleaseEvent(QMouseEvent *event) override
    {
        if (!_dragging)
        {
            event->ignore();
            return;
        }

        event->accept();
        _dragging = false;
        callUpdateInteraction(event->position());
        setKeepMouseGrab(false);
    }

    void mouseMoveEvent(QMouseEvent *event) override
    {
        if (!_dragging)
        {
            event->ignore();
            return;
        }

        event->accept();
        callUpdateInteraction(event->position());
    }

    virtual void updateInteraction(QPointF position, bool hovered, bool dragStarted, bool dragging) = 0;
    virtual bool pickPreview(QPointF position) = 0;

private:
    bool _hovering = false;
    bool _dragging = false;

    void callUpdateInteraction(QPointF position, bool dragStarted = false)
    {
        updateInteraction(position, _hovering || _dragging, dragStarted, _dragging);
    }
};

QSGNode *gizmo_update_paint_node(QSGNode *oldNode,
                                 rust::Slice<std::array<float, 2> const> vertices,
                                 rust::Slice<std::array<float, 4> const> colors,
                                 rust::Slice<std::uint32_t const> indices);
