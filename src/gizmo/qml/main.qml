// Copyright (C) 2019 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

import QtQuick
import QtQuick3D
import QtQuick3D.Helpers

import com.oliv.gizmo

Window {
    id: window
    width: 1280
    height: 720
    visible: true

    View3D {
        id: view
        anchors.fill: parent

        environment: SceneEnvironment {
            clearColor: "#455A64"
            backgroundMode: SceneEnvironment.Color
        }
        PerspectiveCamera {
            id: camera
            position: Qt.vector3d(0, 200, 200)
            eulerRotation.x: -30
        }
        DirectionalLight {
            eulerRotation.x: -30
            eulerRotation.y: -70
        }
        Model {
            position: Qt.vector3d(0, 0, 0)
            source: "#Cube"
            scale: Qt.vector3d(1, 1, 1)
            materials: [ PrincipledMaterial {
                    baseColor: "red"
                }
            ]
        }
        AxisHelper {
        }
    }

    Gizmo {
        id: gizmo
        anchors.fill: parent
        cameraPosition: camera.position
        cameraRotation: camera.rotation.toVector4d()
        cameraVerticalFoV: camera.fieldOfView
        cameraNearPlane: camera.clipNear
        cameraFarPlane: camera.clipFar
    }

    WasdController {
        controlledObject: camera
    }

    DebugView {
        source: view
    }
}
