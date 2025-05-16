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
            position: Qt.vector3d(200, 500, 500)
            eulerRotation.x: -40
            eulerRotation.y: 10
        }
        DirectionalLight {
            eulerRotation.x: -30
            eulerRotation.y: -70
        }
        Model {
            id: cube1
            position: Qt.vector3d(0, 0, 0)
            source: "#Cube"
            scale: Qt.vector3d(1, 1, 1)
            materials: [ PrincipledMaterial {
                    baseColor: "red"
                }
            ]
        }

        Model {
            id: cube2
            position: Qt.vector3d(300, 50, -200)
            eulerRotation.y: 30
            source: "#Cube"
            scale: Qt.vector3d(1, 1, 1)
            materials: [ PrincipledMaterial {
                    baseColor: "green"
                }
            ]

            SequentialAnimation on y {
        loops: Animation.Infinite
        NumberAnimation {
            duration: 3000
            to: -150
            from: 150
            easing.type:Easing.InQuad
        }
        NumberAnimation {
            duration: 3000
            to: 150
            from: -150
            easing.type:Easing.OutQuad
        }
    }
        }
        AxisHelper {
        }
    }

    WasdController {
        controlledObject: camera
    }

    // NOTE: order matters, gizmo must first receive mouse events before the
    //       camera controller
    Gizmo {
        id: gizmo

        anchors.fill: parent

        cameraPosition: camera.position
        cameraRotation: camera.rotation.toVector4d()
        cameraVerticalFoV: camera.fieldOfView
        cameraNearPlane: camera.clipNear
        cameraFarPlane: camera.clipFar

        targetPosition: cube2.position
        targetRotation: cube2.rotation.toVector4d()
        targetScale: cube2.scale

        onTransformUpdated: function(newPosition: vector3d, newRotation: vector4d, newScale: vector3d) {
            cube2.position = newPosition
            // NOTE: quaternion expect the scalar part first
            cube2.rotation = Qt.quaternion(newRotation.w,
                                           newRotation.x,
                                           newRotation.y,
                                           newRotation.z,
                                           )
            cube2.scale = newScale
        }
    }

    DebugView {
        source: view
    }
}
