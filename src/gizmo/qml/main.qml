// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

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
        property Model pickedModel: cube1
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
            pickable: true
            position: Qt.vector3d(0, 0, 0)
            source: "#Cube"
            scale: Qt.vector3d(1, 1, 1)
            materials: [
                PrincipledMaterial {
                    baseColor: "red"
                }
            ]
        }

        Model {
            id: cube2
            pickable: true
            position: Qt.vector3d(300, 50, -200)
            eulerRotation.y: 30
            source: "#Cube"
            scale: Qt.vector3d(1, 1, 1)
            materials: [
                PrincipledMaterial {
                    baseColor: "green"
                }
            ]

            // Showcase external update of target position during hover/dragging of the gizmo
            // SequentialAnimation on y {
            //     loops: Animation.Infinite
            //     NumberAnimation {
            //         duration: 3000
            //         to: -150
            //         from: 150
            //         easing.type: Easing.InQuad
            //     }
            //     PauseAnimation {
            //         duration: 1000
            //     }
            //     NumberAnimation {
            //         duration: 3000
            //         to: 150
            //         from: -150
            //         easing.type: Easing.OutQuad
            //     }
            //     PauseAnimation {
            //         duration: 1000
            //     }
            // }
        }
        AxisHelper {}
    }

    WasdController {
        controlledObject: camera
    }

    MouseArea {
        anchors.fill: parent
        onClicked: (mouse) => {
            const result = view.pick(mouse.x, mouse.y);
            if(result.objectHit !== view.pickedModel) {
                view.pickedModel = result.objectHit;
                mouse.accepted = true;
            }
            else {
                mouse.accepted = false;
            }
        }
    }

    // NOTE: order matters, gizmo must first receive mouse events before the
    //       camera controller
    Gizmo {
        visible: view.pickedModel !== null

        anchors.fill: parent

        cameraPosition: camera.position
        cameraRotation: camera.rotation.toVector4d()
        cameraVerticalFoV: camera.fieldOfView
        cameraNearPlane: camera.clipNear
        cameraFarPlane: camera.clipFar

        targetPosition: view.pickedModel ? view.pickedModel.position : Qt.vector3d(0, 0, 0)
        targetRotation: view.pickedModel ? view.pickedModel.rotation.toVector4d() : Qt.quaternion(0, 0, 0, 1)
        targetScale: view.pickedModel ? view.pickedModel.scale : Qt.vector3d(1, 1, 1)

        onTransformUpdated: (newPosition, newRotation, newScale) => {
            if(view.pickedModel === null) {
                return;
            }

            view.pickedModel.position = newPosition;
            // IMPORTANT: quaternion expect the scalar part first in qt's api
            view.pickedModel.rotation = Qt.quaternion(newRotation.w, newRotation.x, newRotation.y, newRotation.z);
            view.pickedModel.scale = newScale;
        }
    }

    DebugView {
        source: view
    }
}
