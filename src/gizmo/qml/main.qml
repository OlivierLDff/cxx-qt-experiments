// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import QtQuick.Window
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
        property list<Model> pickedModels: [cube1]
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
            SequentialAnimation on y {
                loops: Animation.Infinite
                NumberAnimation {
                    duration: 3000
                    to: -150
                    from: 150
                    easing.type: Easing.InQuad
                }
                PauseAnimation {
                    duration: 1000
                }
                NumberAnimation {
                    duration: 3000
                    to: 150
                    from: -150
                    easing.type: Easing.OutQuad
                }
                PauseAnimation {
                    duration: 1000
                }
            }

            onYChanged: () => gizmo.updateTargets()
        }
        AxisHelper {}
    }

    WasdController {
        controlledObject: camera
    }

    MouseArea {
        anchors.fill: parent
        onClicked: mouse => {
            const result = view.pick(mouse.x, mouse.y);
            if ((mouse.modifiers & Qt.ShiftModifier) || (mouse.modifiers & Qt.ControlModifier)) {
                if (result.objectHit !== null) {
                    const index = view.pickedModels.indexOf(result.objectHit);
                    if (index === -1) {
                        view.pickedModels.push(result.objectHit);
                        gizmo.updateTargets();
                        mouse.accepted = true;
                    }
                }
            } else {
                if (result.objectHit !== null) {
                    view.pickedModels = [result.objectHit];
                    gizmo.updateTargets();
                    mouse.accepted = true;
                } else {
                    if (view.pickedModels.length > 0) {
                        view.pickedModels = [];
                        gizmo.updateTargets();
                        mouse.accepted = true;
                    }
                }
            }
        }
    }

    // NOTE: order matters, gizmo must first receive mouse events before the
    //       camera controller
    Gizmo {
        id: gizmo
        visible: view.pickedModel !== null

        anchors.fill: parent

        cameraPosition: camera.position
        cameraRotation: camera.rotation.toVector4d()
        cameraVerticalFoV: camera.fieldOfView
        cameraNearPlane: camera.clipNear
        cameraFarPlane: camera.clipFar

        translateEnabled: translateCb.checked
        translatePlaneEnabled: translatePlaneCb.checked
        translateViewEnabled: translateViewCb.checked

        rotateEnabled: rotateCb.checked
        rotateViewEnabled: rotateViewCb.checked

        scaleEnabled: scaleCb.checked
        scalePlaneEnabled: scalePlaneCb.checked
        scaleUniformEnabled: scaleUniformCb.checked

        orientation: localGizmo.checked ? Gizmo.Local : Gizmo.Global
        pivotPoint: pivotIndividualOrigin.checked ? Gizmo.IndividualOrigins : Gizmo.MedianPoint

        snapping: snappingCb.checked
        snapDistance: parseFloat(snapDistanceTf.text.replace(",", ".")) * 100
        snapAngle: parseFloat(snapAngleTf.text.replace(",", ".")) * Math.PI / 180
        snapScale: parseFloat(snapScaleTf.text.replace(",", "."))
        pixelsPerPoint: Screen.devicePixelRatio

        strokeWidth: strokeWidthSlider.value
        gizmoSize: gizmoSizeSlider.value

        function updateTargets() {
            let newTargets = [];

            for (let i = 0; i < view.pickedModels.length; i++) {
                const model = view.pickedModels[i];
                newTargets.push({
                    position: model.position,
                    rotation: model.rotation.toVector4d(),
                    scale: model.scale
                });
            }

            targets = newTargets;
        }

        onTransformUpdated: (transforms) => {
            if(view.pickedModels.length !== transforms.length) {
                console.warn("Transforms count doesn't match our targets, something is wrong")
                return;
            }
            for (let i = 0; i < view.pickedModels.length; i++) {
                const transform = transforms[i];
                const model = view.pickedModels[i];

                model.position = transform.position;
                // IMPORTANT: quaternion expect the scalar part first in qt's api
                model.rotation = Qt.quaternion(transform.rotation.w, transform.rotation.x, transform.rotation.y, transform.rotation.z);
                model.scale = transform.scale;
            }
        }
    }

    DebugView {
        source: view
    }

    Pane {
        anchors.right: parent.right
        ColumnLayout {
            Label {
                text: "Modes:"
            }
            CheckBox {
                id: translateCb
                Layout.fillWidth: true
                text: "Translate"
                checked: true
            }
            CheckBox {
                id: translatePlaneCb
                Layout.fillWidth: true
                text: "Translate Plane"
                checked: true
            }
            CheckBox {
                id: translateViewCb
                Layout.fillWidth: true
                text: "Translate View"
                checked: true
            }
            CheckBox {
                id: rotateCb
                Layout.fillWidth: true
                text: "Rotate"
                checked: true
            }
            CheckBox {
                id: rotateViewCb
                Layout.fillWidth: true
                text: "Rotate View"
                checked: true
            }
            CheckBox {
                id: scaleCb
                Layout.fillWidth: true
                text: "Scale"
                checked: true
            }
            CheckBox {
                id: scalePlaneCb
                Layout.fillWidth: true
                text: "Scale Plane"
                checked: true
            }
            CheckBox {
                id: scaleUniformCb
                Layout.fillWidth: true
                text: "Scale Uniform"
                checked: true
            }
            Label {
                text: "Config:"
            }
            CheckBox {
                id: localGizmo
                Layout.fillWidth: true
                text: "Local Gizmo"
            }
            CheckBox {
                id: pivotIndividualOrigin
                Layout.fillWidth: true
                text: "Pivot Individual Origin"
            }
            Label {
                text: "Snap:"
            }
            CheckBox {
                id: snappingCb
                Layout.fillWidth: true
                text: "Snapping"
            }
            RowLayout {
                Label {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    text: "Snap Distance"
                }
                TextField {
                    id: snapDistanceTf
                    Layout.preferredWidth: 30
                    selectByMouse: true
                    placeholderText: qsTr("snapDistance")
                    text: "1"
                    validator: DoubleValidator {
                        bottom: 0.01
                        top: 10
                        decimals: 2
                    }
                }
            }
            RowLayout {
                Label {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    text: "Snap Angle (°)"
                }
                TextField {
                    id: snapAngleTf
                    Layout.preferredWidth: 30
                    selectByMouse: true
                    placeholderText: qsTr("snapAngle")
                    text: "45"
                    validator: DoubleValidator {
                        bottom: 1
                        top: 180
                        decimals: 0
                    }
                }
            }
            RowLayout {
                Label {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    text: "Snap Scale"
                }
                TextField {
                    id: snapScaleTf
                    Layout.preferredWidth: 30
                    selectByMouse: true
                    placeholderText: qsTr("snapScale")
                    text: "1"
                    validator: DoubleValidator {
                        bottom: 0.01
                        top: 10
                        decimals: 2
                    }
                }
            }
            Label {
                text: "Visuals:"
            }
            RowLayout {
                Label {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    text: "Stroke Width"
                }
                Slider {
                    id: strokeWidthSlider
                    Layout.preferredWidth: 100
                    from: 1
                    to: 10
                    stepSize: 1
                    value: 4
                }
            }
            RowLayout {
                Label {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    text: "Gizmo Size"
                }
                Slider {
                    id: gizmoSizeSlider
                    Layout.preferredWidth: 100
                    from: 1
                    to: 200
                    stepSize: 1
                    value: 75
                }
            }
        }
    }
}
