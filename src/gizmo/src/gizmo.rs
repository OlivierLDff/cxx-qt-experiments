// SPDX-FileCopyrightText: Olivier Le Doeuff <olivier.ldff@gmail.com>
// SPDX-License-Identifier: MIT

use core::pin::Pin;

use cxx_qt::CxxQtType;
use cxx_qt_lib::{QColor, QPointF, QVariant, QVector3D, QVector4D};
use ffi::{
    GizmoModeOverride, GizmoOrientation, QQuickItemFlag, QQuickItemUpdatePaintNodeData, QSGNode,
    TransformPivotPoint,
};
use transform_gizmo::Color32;

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

        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

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

    #[qenum(Gizmo)]
    enum GizmoModeOverride {
        NoOverride,
        /// Rotate around the view forward axis
        RotateView,
        /// Rotate around the X axis
        RotateX,
        /// Rotate around the Y axis
        RotateY,
        /// Rotate around the Z axis
        RotateZ,
        /// Translate along the view forward axis
        TranslateView,
        /// Translate along the X axis
        TranslateX,
        /// Translate along the Y axis
        TranslateY,
        /// Translate along the Z axis
        TranslateZ,
        /// Translate along the XY plane
        TranslateXY,
        /// Translate along the XZ plane
        TranslateXZ,
        /// Translate along the YZ plane
        TranslateYZ,
        /// Scale uniformly in all directions
        ScaleUniform,
        /// Scale along the X axis
        ScaleX,
        /// Scale along the Y axis
        ScaleY,
        /// Scale along the Z axis
        ScaleZ,
        /// Scale along the XY plane
        ScaleXY,
        /// Scale along the XZ plane
        ScaleXZ,
        /// Scale along the YZ plane
        ScaleYZ,
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
        #[qproperty(QVariant, targets)]
        #[qproperty(GizmoOrientation, orientation)]
        #[qproperty(TransformPivotPoint, pivotPoint, rust_name = "pivot_point")]
        #[qproperty(bool, snapping)]
        #[qproperty(f32, snapAngle, rust_name = "snap_angle")]
        #[qproperty(f32, snapDistance, rust_name = "snap_distance")]
        #[qproperty(f32, snapScale, rust_name = "snap_scale")]
        #[qproperty(f32, pixelsPerPoint, rust_name = "pixels_per_point")]
        #[qproperty(bool, translateEnabled, rust_name = "translate_enabled")]
        #[qproperty(bool, translateViewEnabled, rust_name = "translate_view_enabled")]
        #[qproperty(bool, translatePlaneEnabled, rust_name = "translate_plane_enabled")]
        #[qproperty(bool, rotateEnabled, rust_name = "rotate_enabled")]
        #[qproperty(bool, rotateViewEnabled, rust_name = "rotate_view_enabled")]
        #[qproperty(bool, scaleEnabled, rust_name = "scale_enabled")]
        #[qproperty(bool, scaleUniformEnabled, rust_name = "scale_uniform_enabled")]
        #[qproperty(bool, scalePlaneEnabled, rust_name = "scale_plane_enabled")]
        #[qproperty(GizmoModeOverride, modeOverride, rust_name = "mode_override")]
        #[qproperty(QColor, xColor, rust_name = "x_color")]
        #[qproperty(QColor, yColor, rust_name = "y_color")]
        #[qproperty(QColor, zColor, rust_name = "z_color")]
        #[qproperty(QColor, sColor, rust_name = "s_color")]
        #[qproperty(f32, inactiveAlpha, rust_name = "inactive_alpha")]
        #[qproperty(f32, highlightAlpha, rust_name = "highlight_alpha")]
        #[qproperty(f32, strokeWidth, rust_name = "stroke_width")]
        #[qproperty(f32, gizmoSize, rust_name = "gizmo_size")]
        type Gizmo = super::GizmoRust;

        #[inherit]
        #[rust_name = "set_flag"]
        fn setFlag(self: Pin<&mut Gizmo>, flag: QQuickItemFlag, enabled: bool);

        #[inherit]
        fn update(self: Pin<&mut Gizmo>);

        #[inherit]
        fn size(self: &Gizmo) -> QSizeF;

        #[inherit]
        #[rust_name = "is_visible"]
        fn isVisible(self: &Gizmo) -> bool;

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
        fn transform_updated(self: Pin<&mut Gizmo>, transforms: QVariant);
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

        fn extract_target_count_from_qvariant(targets: QVariant) -> usize;

        fn extract_targets_from_qvariant(
            targets: QVariant,
            positions: &mut [QVector3D],
            rotations: &mut [QVector4D],
            scales: &mut [QVector3D],
        );

        fn transforms_to_qvariant(
            positions: &[QVector3D],
            rotations: &[QVector4D],
            scales: &[QVector3D],
        ) -> QVariant;
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

impl From<TransformPivotPoint> for transform_gizmo::config::TransformPivotPoint {
    fn from(value: TransformPivotPoint) -> Self {
        match value {
            TransformPivotPoint::MedianPoint => Self::MedianPoint,
            TransformPivotPoint::IndividualOrigins => Self::IndividualOrigins,
            _ => {
                eprintln!(
                    "Unknown TransformPivotPoint, defaulting to TransformPivotPoint::MedianPoint"
                );
                Self::MedianPoint
            }
        }
    }
}

impl Default for GizmoModeOverride {
    fn default() -> Self {
        Self::NoOverride
    }
}

impl From<GizmoModeOverride> for Option<transform_gizmo::GizmoMode> {
    fn from(value: GizmoModeOverride) -> Self {
        match value {
            GizmoModeOverride::RotateView => Some(transform_gizmo::GizmoMode::RotateView),
            GizmoModeOverride::RotateX => Some(transform_gizmo::GizmoMode::RotateX),
            GizmoModeOverride::RotateY => Some(transform_gizmo::GizmoMode::RotateY),
            GizmoModeOverride::RotateZ => Some(transform_gizmo::GizmoMode::RotateZ),
            GizmoModeOverride::TranslateView => Some(transform_gizmo::GizmoMode::TranslateView),
            GizmoModeOverride::TranslateX => Some(transform_gizmo::GizmoMode::TranslateX),
            GizmoModeOverride::TranslateY => Some(transform_gizmo::GizmoMode::TranslateY),
            GizmoModeOverride::TranslateZ => Some(transform_gizmo::GizmoMode::TranslateZ),
            GizmoModeOverride::TranslateXY => Some(transform_gizmo::GizmoMode::TranslateXY),
            GizmoModeOverride::TranslateXZ => Some(transform_gizmo::GizmoMode::TranslateXZ),
            GizmoModeOverride::TranslateYZ => Some(transform_gizmo::GizmoMode::TranslateYZ),
            GizmoModeOverride::ScaleUniform => Some(transform_gizmo::GizmoMode::ScaleUniform),
            GizmoModeOverride::ScaleX => Some(transform_gizmo::GizmoMode::ScaleX),
            GizmoModeOverride::ScaleY => Some(transform_gizmo::GizmoMode::ScaleY),
            GizmoModeOverride::ScaleZ => Some(transform_gizmo::GizmoMode::ScaleZ),
            GizmoModeOverride::ScaleXY => Some(transform_gizmo::GizmoMode::ScaleXY),
            GizmoModeOverride::ScaleXZ => Some(transform_gizmo::GizmoMode::ScaleXZ),
            GizmoModeOverride::ScaleYZ => Some(transform_gizmo::GizmoMode::ScaleYZ),
            _ => None,
        }
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
    targets: QVariant,
    gizmo: Option<transform_gizmo::Gizmo>,
    gizmo_updated_since_last_draw: bool,
    /// Keep last interaction in case the target moves while we are dragging
    gizmo_last_interaction: Option<transform_gizmo::GizmoInteraction>,
    /// Determines the gizmo's orientation relative to global or local axes.
    orientation: GizmoOrientation,
    /// Pivot point for transformations
    pivot_point: TransformPivotPoint,
    /// Toggles snapping to predefined increments during transformations for precision.
    snapping: bool,
    /// Angle increment for snapping rotations, in radians.
    snap_angle: f32,
    /// Distance increment for snapping translations.
    snap_distance: f32,
    /// Scale increment for snapping scalings.
    snap_scale: f32,
    /// Ratio of window's physical size to logical size.
    pixels_per_point: f32,
    translate_enabled: bool,
    translate_view_enabled: bool,
    translate_plane_enabled: bool,
    rotate_enabled: bool,
    rotate_view_enabled: bool,
    scale_enabled: bool,
    scale_uniform_enabled: bool,
    scale_plane_enabled: bool,
    mode_override: GizmoModeOverride,
    x_color: QColor,
    y_color: QColor,
    z_color: QColor,
    s_color: QColor,
    /// Alpha of the gizmo color when inactive
    inactive_alpha: f32,
    /// Alpha of the gizmo color when highlighted/active
    highlight_alpha: f32,
    /// Width (thickness) of the gizmo strokes
    stroke_width: f32,
    /// Gizmo size in pixels
    gizmo_size: f32,
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
        {
            let mut this = self.as_mut().rust_mut();
            this.snap_angle = transform_gizmo::config::DEFAULT_SNAP_ANGLE;
            this.snap_distance = transform_gizmo::config::DEFAULT_SNAP_DISTANCE;
            this.snap_scale = transform_gizmo::config::DEFAULT_SNAP_SCALE;
            this.pixels_per_point = 1.;

            this.translate_enabled = true;
            this.translate_plane_enabled = true;
            this.translate_view_enabled = true;
            this.rotate_enabled = true;
            this.rotate_view_enabled = true;
            this.scale_enabled = true;
            this.scale_plane_enabled = true;
            this.scale_uniform_enabled = true;

            this.x_color = QColor::from_rgb(255, 0, 125);
            this.y_color = QColor::from_rgb(0, 255, 125);
            this.z_color = QColor::from_rgb(0, 125, 255);
            this.s_color = QColor::from_rgb(255, 255, 255);

            this.inactive_alpha = 0.7;
            this.highlight_alpha = 1.0;
            this.stroke_width = 4.0;
            this.gizmo_size = 75.0;
        }

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
            .on_translate_enabled_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_translate_plane_enabled_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_translate_view_enabled_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_rotate_enabled_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_rotate_view_enabled_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_scale_enabled_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_scale_plane_enabled_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_scale_uniform_enabled_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_mode_override_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_orientation_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_x_color_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_y_color_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_z_color_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_s_color_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_inactive_alpha_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_highlight_alpha_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_stroke_width_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_gizmo_size_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_targets_changed(|qobject| qobject.update())
            .release();

        self.as_mut()
            .on_target_position_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_target_rotation_changed(|qobject| qobject.update())
            .release();
        self.as_mut()
            .on_target_scale_changed(|qobject| qobject.update())
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
        let result = self.as_mut().update_interaction_impl(interaction);
        if let Some((_, transforms)) = result {
            let (positions, rotations, scales): (Vec<QVector3D>, Vec<QVector4D>, Vec<QVector3D>) =
                itertools::multiunzip(transforms.iter().map(|transform| {
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

                    (position, rotation, scale)
                }));
            let transforms = ffi::transforms_to_qvariant(&positions, &rotations, &scales);

            self.as_mut().rust_mut().targets = transforms.clone();
            self.as_mut().transform_updated(transforms);
        }
        self.as_mut().rust_mut().gizmo_last_interaction = Some(transform_gizmo::GizmoInteraction {
            drag_started: false,
            ..interaction
        });
    }

    fn update_interaction_impl(
        self: Pin<&mut Self>,
        interaction: transform_gizmo::GizmoInteraction,
    ) -> Option<(
        transform_gizmo::GizmoResult,
        Vec<transform_gizmo::math::Transform>,
    )> {
        let target_count = ffi::extract_target_count_from_qvariant(self.rust().targets.clone());
        let mut positions = vec![QVector3D::default(); target_count];
        let mut rotations = vec![QVector4D::default(); target_count];
        let mut scales = vec![QVector3D::default(); target_count];

        ffi::extract_targets_from_qvariant(
            self.rust().targets.clone(),
            &mut positions,
            &mut rotations,
            &mut scales,
        );

        assert!(positions.len() == rotations.len());
        assert!(positions.len() == scales.len());
        let transforms = itertools::multizip((positions, rotations, scales))
            .map(|(position, rotation, scale)| {
                transform_gizmo::math::Transform::from_scale_rotation_translation(
                    glam::Vec3::new(scale.x(), scale.y(), scale.z()).as_dvec3(),
                    glam::Quat::from_xyzw(rotation.x(), rotation.y(), rotation.z(), rotation.w())
                        .as_dquat(),
                    glam::Vec3::new(position.x(), position.y(), position.z()).as_dvec3(),
                )
            })
            .collect::<Vec<_>>();

        self.with_gizmo(|mut qobject, gizmo| {
            qobject.as_mut().rust_mut().gizmo_updated_since_last_draw = true;
            gizmo.update(interaction, &transforms)
        })
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
        let size = self.size();
        let this = self.rust();
        let view_matrix = this.view_matrix();
        let width = size.width() as f32;
        let height = size.height() as f32;
        let projection_matrix = this.projection_matrix(width, height);
        let orientation = this.orientation.into();
        let pivot_point = this.pivot_point.into();
        let snapping = this.snapping;
        let snap_angle = if this.snap_angle.is_finite() {
            this.snap_angle.abs()
        } else {
            transform_gizmo::config::DEFAULT_SNAP_ANGLE
        };
        let snap_distance = if this.snap_distance.is_finite() {
            this.snap_distance.abs()
        } else {
            transform_gizmo::config::DEFAULT_SNAP_DISTANCE
        };
        let snap_scale = if this.snap_scale.is_finite() {
            this.snap_scale.abs()
        } else {
            transform_gizmo::config::DEFAULT_SNAP_SCALE
        };
        let pixels_per_point = if this.pixels_per_point.is_finite() {
            this.pixels_per_point.abs()
        } else {
            1.0
        };
        let modes = {
            let mut modes = transform_gizmo::EnumSet::<transform_gizmo::GizmoMode>::new();
            if this.translate_enabled {
                modes.insert(transform_gizmo::GizmoMode::TranslateX);
                modes.insert(transform_gizmo::GizmoMode::TranslateY);
                modes.insert(transform_gizmo::GizmoMode::TranslateZ);
            }
            if this.translate_plane_enabled {
                modes.insert(transform_gizmo::GizmoMode::TranslateXY);
                modes.insert(transform_gizmo::GizmoMode::TranslateXZ);
                modes.insert(transform_gizmo::GizmoMode::TranslateYZ);
            }
            if this.translate_view_enabled {
                modes.insert(transform_gizmo::GizmoMode::TranslateView);
            }
            if this.rotate_enabled {
                modes.insert(transform_gizmo::GizmoMode::RotateX);
                modes.insert(transform_gizmo::GizmoMode::RotateY);
                modes.insert(transform_gizmo::GizmoMode::RotateZ);
            }
            if this.rotate_view_enabled {
                modes.insert(transform_gizmo::GizmoMode::RotateView);
            }
            if this.scale_enabled {
                modes.insert(transform_gizmo::GizmoMode::ScaleX);
                modes.insert(transform_gizmo::GizmoMode::ScaleY);
                modes.insert(transform_gizmo::GizmoMode::ScaleZ);
            }
            if this.scale_plane_enabled {
                modes.insert(transform_gizmo::GizmoMode::ScaleXY);
                modes.insert(transform_gizmo::GizmoMode::ScaleXZ);
                modes.insert(transform_gizmo::GizmoMode::ScaleYZ);
            }
            if this.scale_uniform_enabled {
                modes.insert(transform_gizmo::GizmoMode::ScaleUniform);
            }
            modes
        };
        let mode_override = this.mode_override.into();

        let visuals = transform_gizmo::GizmoVisuals {
            x_color: Color32::from_rgb(
                this.x_color.red() as u8,
                this.x_color.green() as u8,
                this.x_color.blue() as u8,
            ),
            y_color: Color32::from_rgb(
                this.y_color.red() as u8,
                this.y_color.green() as u8,
                this.y_color.blue() as u8,
            ),
            z_color: Color32::from_rgb(
                this.z_color.red() as u8,
                this.z_color.green() as u8,
                this.z_color.blue() as u8,
            ),
            s_color: Color32::from_rgb(
                this.s_color.red() as u8,
                this.s_color.green() as u8,
                this.s_color.blue() as u8,
            ),
            inactive_alpha: if this.inactive_alpha.is_finite() {
                this.inactive_alpha.abs()
            } else {
                0.7
            },
            highlight_alpha: if this.highlight_alpha.is_finite() {
                this.highlight_alpha.abs()
            } else {
                1.0
            },
            stroke_width: if this.stroke_width.is_finite() {
                this.stroke_width.abs()
            } else {
                4.0
            },
            gizmo_size: if this.gizmo_size.is_finite() {
                this.gizmo_size.abs()
            } else {
                75.0
            },
            ..Default::default()
        };

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
            modes,
            mode_override,
            orientation,
            pivot_point,
            snapping,
            snap_angle,
            snap_distance,
            snap_scale,
            visuals,
            pixels_per_point,
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
            assert!(
                !interaction.drag_started,
                "We don't want to repeat a drag started interaction"
            );

            let _ = self.as_mut().update_interaction_impl(interaction);
        }
        self.as_mut().rust_mut().gizmo_updated_since_last_draw = false;

        self.with_gizmo(|qobject, gizmo| unsafe {
            let target_count = ffi::extract_target_count_from_qvariant(qobject.targets().clone());
            let draw_data = if target_count > 0 && qobject.is_visible() {
                gizmo.draw()
            } else {
                transform_gizmo::GizmoDrawData::default()
            };

            ffi::gizmo_update_paint_node(
                old_node,
                &draw_data.vertices,
                &draw_data.colors,
                &draw_data.indices,
            )
        })
    }
}
