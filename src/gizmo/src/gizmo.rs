// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

use core::pin::Pin;

use cxx_qt::CxxQtType;
use cxx_qt_lib::{QPointF, QVector3D, QVector4D};
use ffi::{
    GizmoOrientation, QQuickItemFlag, QQuickItemUpdatePaintNodeData, QSGNode, TransformPivotPoint,
};

#[cxx_qt::bridge]
pub mod ffi {
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QQuickItemFlag {
        /// Indicates this item should visually clip its children so that they are rendered only
        /// within the boundaries of this item.
        ItemClipsChildrenToShape = 0x01,
        /// Indicates the item supports text input methods.
        ItemAcceptsInputMethod = 0x02,
        /// Indicates the item is a focus scope. See Keyboard Focus in Qt Quick for more
        /// information.
        ItemIsFocusScope = 0x04,
        /// Indicates the item has visual content and should be rendered by the scene graph.
        ItemHasContents = 0x08,
        /// Indicates the item accepts drag and drop events.
        ItemAcceptsDrops = 0x10,
        /// Indicates that the item defines a viewport for its children.
        ItemIsViewport = 0x20,
        /// Indicates that the item wishes to know the viewport bounds when any ancestor has the
        /// ItemIsViewport flag set.
        ItemObservesViewport = 0x40,
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxxqtlib1_qquickitem.h");
        type QQuickItemFlag;
        type QQuickItemUpdatePaintNodeData;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpointf.h");
        type QPointF = cxx_qt_lib::QPointF;

        include!("cxx-qt-lib/qsizef.h");
        type QSizeF = cxx_qt_lib::QSizeF;

        include!("cxx-qt-lib/qvector3d.h");
        type QVector3D = cxx_qt_lib::QVector3D;

        include!("cxx-qt-lib/qvector4d.h");
        type QVector4D = cxx_qt_lib::QVector4D;

        include!(<QtQuick/QQuickItem>);
        type QQuickItem;

    }

    unsafe extern "C++" {
        include!(<QtQuick/QSGNode>);
        type QSGNode;
    }

    #[qenum(Gizmo)]
    /// The point in space around which all rotations are centered.
    enum TransformPivotPoint {
        /// Pivot around the median point of targets
        MedianPoint,
        /// Pivot around each target's own origin
        IndividualOrigins,
    }

    /// Orientation of a gizmo.
    #[qenum(Gizmo)]
    enum GizmoOrientation {
        /// Transformation axes are aligned to world space.
        Global,
        /// Transformation axes are aligned to the last target's orientation.
        Local,
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = GizmoInteractionItem]
        #[qproperty(QVector3D, cameraPosition, rust_name = "camera_position")]
        #[qproperty(QVector4D, cameraRotation, rust_name = "camera_rotation")]
        #[qproperty(f32, cameraVerticalFoV, rust_name = "camera_vertical_fov")]
        #[qproperty(f32, cameraNearPlane, rust_name = "camera_near_plane")]
        #[qproperty(f32, cameraFarPlane, rust_name = "camera_far_plane")]
        #[qproperty(QVector3D, targetPosition, rust_name = "target_position")]
        #[qproperty(QVector4D, targetRotation, rust_name = "target_rotation")]
        #[qproperty(QVector3D, targetScale, rust_name = "target_scale")]
        #[qproperty(GizmoOrientation, orientation)]
        #[qproperty(TransformPivotPoint, pivotPoint, rust_name = "pivot_point")]
        type Gizmo = super::GizmoRust;

        #[inherit]
        #[rust_name = "set_flag"]
        fn setFlag(self: Pin<&mut Gizmo>, flag: QQuickItemFlag, enabled: bool);

        #[inherit]
        fn update(self: Pin<&mut Gizmo>);

        #[inherit]
        fn size(self: &Gizmo) -> QSizeF;

        #[cxx_override]
        #[cxx_name = "updateInteraction"]
        fn update_interaction(
            self: Pin<&mut Gizmo>,
            cursor_position: QPointF,
            hovered: bool,
            drag_started: bool,
            dragging: bool,
        );

        #[cxx_override]
        #[cxx_name = "pickPreview"]
        fn pick_preview(self: Pin<&mut Gizmo>, cursor_position: QPointF) -> bool;

        #[cxx_override]
        #[cxx_name = "updatePaintNode"]
        unsafe fn update_paint_node(
            self: Pin<&mut Gizmo>,
            old_node: *mut QSGNode,
            update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
        ) -> *mut QSGNode;

        #[qsignal]
        #[cxx_name = "transformUpdated"]
        fn transform_updated(
            self: Pin<&mut Gizmo>,
            position: QVector3D,
            rotation: QVector4D,
            scale: QVector3D,
        );
    }

    unsafe extern "C++" {
        include!("gizmo.h");

        type GizmoInteractionItem;

        /// cpp implementation of the update_paint_node function
        ///
        /// ## Safety
        ///
        /// The function takes ownership of the `old_node` pointer and returns a new one.
        /// The caller is responsible for managing the memory of the returned pointer.
        #[allow(clippy::missing_safety_doc)] // <- Somehow false positive
        unsafe fn gizmo_update_paint_node(
            old_node: *mut QSGNode,
            vertices: &[[f32; 2]],
            colors: &[[f32; 4]],
            indices: &[u32],
        ) -> *mut QSGNode;
    }

    impl cxx_qt::Initialize for Gizmo {}
}

impl Default for GizmoOrientation {
    fn default() -> Self {
        Self::Global
    }
}

impl From<GizmoOrientation> for transform_gizmo::GizmoOrientation {
    fn from(value: GizmoOrientation) -> Self {
        match value {
            GizmoOrientation::Global => Self::Global,
            GizmoOrientation::Local => Self::Local,
            _ => {
                eprintln!("Unknown GizmoOrientation, defaulting to GizmoOrientation::Global");
                Self::Global
            }
        }
    }
}

impl Default for TransformPivotPoint {
    fn default() -> Self {
        Self::MedianPoint
    }
}

#[derive(Default)]
pub struct GizmoRust {
    camera_position: QVector3D,
    camera_rotation: QVector4D,
    camera_vertical_fov: f32,
    camera_near_plane: f32,
    camera_far_plane: f32,
    target_position: QVector3D,
    target_rotation: QVector4D,
    target_scale: QVector3D,
    gizmo: Option<transform_gizmo::Gizmo>,
    gizmo_updated_since_last_draw: bool,
    /// Keep last interaction in case the target moves while we are dragging
    gizmo_last_interaction: Option<transform_gizmo::GizmoInteraction>,
    orientation: GizmoOrientation,
    pivot_point: TransformPivotPoint,
}

impl GizmoRust {
    fn view_matrix(&self) -> glam::Mat4 {
        let rotation = glam::Quat::from_xyzw(
            self.camera_rotation.x(),
            self.camera_rotation.y(),
            self.camera_rotation.z(),
            self.camera_rotation.w(),
        );
        let translation = glam::Vec3::new(
            self.camera_position.x(),
            self.camera_position.y(),
            self.camera_position.z(),
        );

        glam::Mat4::from_rotation_translation(rotation, translation).inverse()
    }

    fn projection_matrix(&self, width: f32, height: f32) -> glam::Mat4 {
        let fov = self.camera_vertical_fov.to_radians();
        let aspect_ratio = width / height;
        let near = self.camera_near_plane;
        let far = self.camera_far_plane;

        glam::Mat4::perspective_rh(fov, aspect_ratio, near, far)
    }
}

impl cxx_qt::Initialize for ffi::Gizmo {
    fn initialize(mut self: Pin<&mut Self>) {
        self.as_mut()
            .set_flag(QQuickItemFlag::ItemHasContents, true);

        self.as_mut()
            .on_camera_far_plane_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_camera_near_plane_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_camera_vertical_fov_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_camera_position_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_camera_rotation_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_orientation_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_target_position_changed(|qobject| {
                qobject.update();
            })
            .release();
        self.as_mut()
            .on_target_rotation_changed(|qobject| {
                qobject.update();
            })
            .release();
        self.as_mut()
            .on_target_scale_changed(|qobject| {
                qobject.update();
            })
            .release();
    }
}

impl ffi::Gizmo {
    fn update_interaction(
        mut self: Pin<&mut Self>,
        cursor_position: QPointF,
        hovered: bool,
        drag_started: bool,
        dragging: bool,
    ) {
        self.as_mut().update();
        let interaction = transform_gizmo::GizmoInteraction {
            cursor_pos: (cursor_position.x() as f32, cursor_position.y() as f32),
            hovered,
            drag_started,
            dragging,
        };
        self.as_mut().update_interaction_impl(interaction);
        self.as_mut().rust_mut().gizmo_last_interaction = Some(transform_gizmo::GizmoInteraction {
            drag_started: false,
            ..interaction
        });
    }

    fn update_interaction_impl(
        self: Pin<&mut Self>,
        interaction: transform_gizmo::GizmoInteraction,
    ) {
        let transform = transform_gizmo::math::Transform::from_scale_rotation_translation(
            glam::Vec3::new(
                self.target_scale.x(),
                self.target_scale.y(),
                self.target_scale.z(),
            )
            .as_dvec3(),
            glam::Quat::from_xyzw(
                self.target_rotation.x(),
                self.target_rotation.y(),
                self.target_rotation.z(),
                self.target_rotation.w(),
            )
            .as_dquat(),
            glam::Vec3::new(
                self.target_position.x(),
                self.target_position.y(),
                self.target_position.z(),
            )
            .as_dvec3(),
        );

        self.with_gizmo(|mut qobject, gizmo| {
            qobject.as_mut().rust_mut().gizmo_updated_since_last_draw = true;
            let result = gizmo.update(interaction, &[transform]);

            if let Some((_, transforms)) = result {
                let transform = transforms.first().unwrap();
                let position = QVector3D::new(
                    transform.translation.x as f32,
                    transform.translation.y as f32,
                    transform.translation.z as f32,
                );
                let rotation = QVector4D::new(
                    transform.rotation.v.x as f32,
                    transform.rotation.v.y as f32,
                    transform.rotation.v.z as f32,
                    transform.rotation.s as f32,
                );
                let scale = QVector3D::new(
                    transform.scale.x as f32,
                    transform.scale.y as f32,
                    transform.scale.z as f32,
                );

                // Emit the signal only if the values have changed
                // Also emitting the signal from update_paint_node result in very bad performance
                if position != qobject.target_position
                    || rotation != qobject.target_rotation
                    || scale != qobject.target_scale
                {
                    // Internally update the state, so we don't trigger "onChanged" signals when the
                    // user updates its values
                    qobject.as_mut().rust_mut().target_position = position.clone();
                    qobject.as_mut().rust_mut().target_rotation = rotation.clone();
                    qobject.as_mut().rust_mut().target_scale = scale.clone();
                    qobject.transform_updated(position.clone(), rotation.clone(), scale.clone());
                }
            }
        });
    }

    fn pick_preview(self: Pin<&mut Self>, cursor_position: QPointF) -> bool {
        self.with_gizmo(|_, gizmo| {
            gizmo.pick_preview((cursor_position.x() as f32, cursor_position.y() as f32))
        })
    }

    fn with_gizmo<T>(
        mut self: Pin<&mut Self>,
        f: impl FnOnce(Pin<&mut Self>, &mut transform_gizmo::Gizmo) -> T,
    ) -> T {
        let config = self.as_ref().gizmo_config();
        let mut gizmo = match self.as_mut().rust_mut().gizmo.take() {
            Some(mut gizmo) => {
                gizmo.update_config(config);
                gizmo
            }
            None => transform_gizmo::Gizmo::new(config),
        };
        let result = f(self.as_mut(), &mut gizmo);
        self.rust_mut().gizmo = Some(gizmo);
        result
    }

    fn gizmo_config(&self) -> transform_gizmo::GizmoConfig {
        let view_matrix = self.rust().view_matrix();
        let size = self.size();
        let width = size.width() as f32;
        let height = size.height() as f32;
        let projection_matrix = self.rust().projection_matrix(width, height);
        let orientation = self.rust().orientation.into();

        transform_gizmo::GizmoConfig {
            view_matrix: view_matrix.as_dmat4().into(),
            projection_matrix: projection_matrix.as_dmat4().into(),
            viewport: transform_gizmo::Rect {
                min: transform_gizmo::math::Pos2 { x: 0., y: 0. },
                max: transform_gizmo::math::Pos2 {
                    x: width,
                    y: height,
                },
            },
            orientation,
            ..Default::default()
        }
    }

    unsafe fn update_paint_node(
        mut self: Pin<&mut Self>,
        old_node: *mut QSGNode,
        _update_paint_node_data: *mut QQuickItemUpdatePaintNodeData,
    ) -> *mut QSGNode {
        // transform_gizmo::Gizmo expect a call to `update` before a subsequent call to `draw`
        // This case can happen if the camera is updated for example
        if !self.rust().gizmo_updated_since_last_draw {
            let interaction = self.rust().gizmo_last_interaction.unwrap_or_default();
            self.as_mut().update_interaction_impl(interaction);
        }
        self.as_mut().rust_mut().gizmo_updated_since_last_draw = false;

        self.with_gizmo(|_, gizmo| unsafe {
            let draw_data = gizmo.draw();

            ffi::gizmo_update_paint_node(
                old_node,
                &draw_data.vertices,
                &draw_data.colors,
                &draw_data.indices,
            )
        })
    }
}
