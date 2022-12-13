use std::ops;


#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2:f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2]}
    }

    pub fn x(self) -> f32 { self.e[0]}
    pub fn y(self) -> f32 { self.e[1]}
    pub fn z(self) -> f32 { self.e[2]}

    pub fn r(self) -> f32 { self.e[0]}
    pub fn g(self) -> f32 { self.e[1]}
    pub fn b(self) -> f32 { self.e[2]}

    pub fn length (self) -> f32 {
       (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v/v.length()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32{
        v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        ]}
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        ]}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 { e: [
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs,
        ]}
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k: f32 = 1.0/rhs;

        Vec3 { e: [
            self.e[0] * k,
            self.e[1] * k,
            self.e[2] * k,
        ]}
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(Vec3::new(2., 4., 6.) + Vec3::new(1., 1., 4.),
        Vec3::new(3., 5., 10.))
    }

    #[test]
    fn test_vec3_multiply(){
        assert_eq!(Vec3::new(2., 4., 6.) * 2.,
        Vec3::new(4., 8., 12.))
    }

    #[test]
    fn test_vec3_divide(){
        assert_eq!(Vec3::new(2., 4., 6.) / 2.,
        Vec3::new(1., 2., 3.))
    }

}