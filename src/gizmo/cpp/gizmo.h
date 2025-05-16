// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

#pragma once

#include <QtQuick/QSGNode>
#include <array>
#include <cstdint>

#include "rust/cxx.h"

QSGNode *gizmo_update_paint_node(QSGNode *oldNode,
                                 rust::Slice<std::array<float, 2> const> vertices,
                                 rust::Slice<std::array<float, 4> const> colors,
                                 rust::Slice<std::uint32_t const> indices);
