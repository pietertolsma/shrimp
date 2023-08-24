
use na::{Vector3, VecStorage, DMatrix, Rotation3, Unit};
use nalgebra::{Matrix4, Matrix3, Matrix, Dyn, Const};
use nalgebra as na;

use crate::input::InputState;

type Dynamic3DPointMatrix =  Matrix<f32, Dyn, Const<3>, VecStorage<f32, Dyn, Const<3>>>;
type Dynamic4DPointMatrix =  Matrix<f32, Dyn, Const<4>, VecStorage<f32, Dyn, Const<4>>>;

pub struct Camera {
    pub extrinsic: Matrix4<f32>,
    pub intrinsic: Matrix3<f32>,
    pub inv_intrinsic: Matrix3<f32>,
    pub pixel_coords: Dynamic3DPointMatrix,
    pub homo_pixel_coords: Dynamic4DPointMatrix,

    pub look_direction: Vector3<f32>
}

impl Camera {
    pub fn new(fov_deg: f32, sensor_dim: (u32, u32)) -> Self {
        let (width, height) = sensor_dim;
        let intr: Matrix3<f32> = Matrix3::new(
            (width as f32) / (2.0 * (fov_deg.to_radians() / 2.0).tan()), 0.0, width as f32 / 2.0,
            0.0, (height as f32)/ (2.0 * (fov_deg.to_radians() / 2.0).tan()), height as f32 / 2.0,
            0.0, 0.0, 1.0
        );
        let inv_intr = intr.try_inverse().unwrap();
        let pixel_coords = Camera::calc_pixel_coords(width, height, inv_intr);
        let homo_pixel_coords = Camera::convert_to_homo(&pixel_coords);
        Self {
            intrinsic: intr,
            inv_intrinsic: inv_intr,
            extrinsic: Matrix4::identity(),
            pixel_coords,
            homo_pixel_coords,
            look_direction: Vector3::new(1.0, 0.0, 0.0)
        }
    }

    pub fn calc_pixel_coords(width: u32, height: u32, inv_intr: Matrix3<f32>) -> Dynamic3DPointMatrix {
        let mut meshgrid = DMatrix::<f32>::zeros(width as usize * height as usize, 3);

        for (i, (y, x)) in (0..height).flat_map(|y| (0..width).map(move |x| (y, x))).enumerate() {
            // set i-th row of meshgrid to x, y, 1
            meshgrid[(i, 0)] = x as f32;
            meshgrid[(i, 1)] = y as f32;
            meshgrid[(i, 2)] = 1.0;
        }
        // multiply with inv intr
        meshgrid * inv_intr.transpose()
    }

    pub fn convert_to_homo(points: &Dynamic3DPointMatrix) -> Dynamic4DPointMatrix {
        // add column of 1s to points
        Dynamic4DPointMatrix::from_fn(points.nrows(), |r, c| {
            if c < 3 {
                points[(r, c)]
            } else {
                1.0
            }
        })
    }

    pub fn world_coords(&self) -> Dynamic4DPointMatrix {
        self.homo_pixel_coords.clone() * self.extrinsic
    }

    pub fn update_extrinsic(&mut self, extrinsic: Matrix4<f32>) {
        self.extrinsic = extrinsic;
    }

    pub fn update_intrinsic(&mut self, intrinsic: Matrix3<f32>) {
        self.intrinsic = intrinsic;
        self.inv_intrinsic = intrinsic.try_inverse().unwrap();
    }

    pub fn update(&mut self, input: &mut InputState) {
        let pitch_angle = input.mouse_movement.1 as f32 * 0.005;
        let yaw_angle = input.mouse_movement.0 as f32 * 0.005;

        // Yaw: rotate direction around the world's up axis.
        let yaw_rotation = na::Rotation3::from_axis_angle(&na::Vector3::y_axis(), yaw_angle);
        self.look_direction = yaw_rotation * self.look_direction;

        // Pitch: rotate direction around the world's right axis (cross product of up and current direction).
        let right_axis = na::Vector3::y_axis().cross(&self.look_direction);
        let pitch_rotation = na::Rotation3::from_axis_angle(&Unit::new_normalize(right_axis), pitch_angle);
        self.look_direction = pitch_rotation * self.look_direction;

        let rotation = Rotation3::look_at_rh(&self.look_direction, &Vector3::y_axis());

        self.extrinsic.fixed_view_mut::<3,3>(0,0).copy_from(rotation.matrix());

        input.mouse_movement = (0.0, 0.0);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_pixel_coords() {
        let width = 2;
        let height = 2;
        let cam = Camera::new(45.0, (width, height));

        let pixel_coords = cam.pixel_coords;
        // check whether pixel_coords contains width * height rows
        assert_eq!(pixel_coords.nrows() as u32, width * height);
        // check whether pixel_coords contains 4 columns
        assert_eq!(pixel_coords.ncols(), 4);
    }
}