use eframe::epaint::{Pos2, Rect};

#[derive(Default)]
pub(super) struct SnapInput {
    pub min: (Pos2, SnapType),
    pub max: (Pos2, SnapType),
    pub center: (Pos2, SnapType),
    pub anchor: (Pos2, SnapType),
}

#[derive(Default)]
pub(super) struct SnapOutput {
    pub min: SnapPointResult,
    pub max: SnapPointResult,
    pub center: SnapPointResult,
    pub anchor: SnapPointResult,
}

#[derive(Default)]
pub(super) struct SnapPointResult {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

#[derive(Clone, Copy, Default)]
pub(super) enum SnapType {
    #[default]
    DisplayOnly,
    X,
    Y,
    Both,
}

pub(super) fn drag_snap(input: &SnapInput, basis_rects: Vec<Rect>, tolerance: f32) -> SnapOutput {
    let mut output = SnapOutput::default();

    for rect in basis_rects {
        snap_to_point(rect.center(), &input, &mut output, tolerance);

        snap_to_point(rect.left_top(), &input, &mut output, tolerance);

        snap_to_point(rect.center_top(), &input, &mut output, tolerance);

        snap_to_point(rect.right_top(), &input, &mut output, tolerance);

        snap_to_point(rect.right_center(), &input, &mut output, tolerance);

        snap_to_point(rect.right_bottom(), &input, &mut output, tolerance);

        snap_to_point(rect.center_bottom(), &input, &mut output, tolerance);

        snap_to_point(rect.left_bottom(), &input, &mut output, tolerance);

        snap_to_point(rect.left_center(), &input, &mut output, tolerance);
    }

    output
}

fn snap_to_point(snap_point: Pos2, input: &SnapInput, output: &mut SnapOutput, tolerance: f32) {
    apply_snap(
        snap_point,
        input.min.0,
        input.min.1,
        tolerance,
        &mut output.min,
    );

    apply_snap(
        snap_point,
        input.max.0,
        input.max.1,
        tolerance,
        &mut output.max,
    );

    apply_snap(
        snap_point,
        input.center.0,
        input.center.1,
        tolerance,
        &mut output.center,
    );

    apply_snap(
        snap_point,
        input.anchor.0,
        input.anchor.1,
        tolerance,
        &mut output.anchor,
    );

    fn apply_snap(
        snap_point: Pos2,
        point: Pos2,
        snap_type: SnapType,
        tolerance: f32,
        result: &mut SnapPointResult,
    ) {
        match snap_type {
            SnapType::DisplayOnly => {
                if point.x == snap_point.x {
                    result.x = Some(point.x);
                }

                if point.y == snap_point.y {
                    result.y = Some(point.y);
                }
            }
            SnapType::X => {
                if (point.x - snap_point.x).abs() <= tolerance {
                    result.x = Some(snap_point.x);
                }

                if point.y == snap_point.y {
                    result.y = Some(point.y);
                }
            }
            SnapType::Y => {
                if point.x == snap_point.x {
                    result.x = Some(point.x);
                }

                if (point.y - snap_point.y).abs() <= tolerance {
                    result.y = Some(snap_point.y);
                }
            }
            SnapType::Both => {
                if (point.x - snap_point.x).abs() <= tolerance {
                    result.x = Some(snap_point.x);
                }

                if (point.y - snap_point.y).abs() <= tolerance {
                    result.y = Some(snap_point.y);
                }
            }
        }
    }
}
