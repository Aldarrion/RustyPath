use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    v: [f32; 3]
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            v: [0.0, 0.0, 0.0]
        }
    }

    pub fn one() -> Vec3 {
        Vec3 {
            v: [1.0, 1.0, 1.0]
        }
    }

    pub fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            v: [x, y, z]
        }
    }

    pub fn x(&self) -> f32 {
        self.v[0]
    }
    pub fn y(&self) -> f32 {
        self.v[1]
    }
    pub fn z(&self) -> f32 {
        self.v[2]
    }

    pub fn r(&self) -> f32 {
        self.v[0]
    }
    pub fn g(&self) -> f32 {
        self.v[1]
    }
    pub fn b(&self) -> f32 {
        self.v[2]
    }

    pub fn length_sqr(&self) -> f32 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2])
    }

    pub fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.v[0] /= len;
        self.v[1] /= len;
        self.v[2] /= len;
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[1] * other.v[2] - self.v[2] * other.v[1],
                -(self.v[0] * other.v[2] - self.v[2] * other.v[0]),
                self.v[0] * other.v[1] - self.v[1] * other.v[0]
            ]
        }
    }
}

impl<'a> ops::Add<&Vec3> for &'a Vec3 {
    type Output = Vec3;
    
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        *self = Self {
            v: [self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2]
            ]
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            v: [
                -self.v[0],
                -self.v[1],
                -self.v[2]
            ]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        &self + &(-other)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            v: [self.v[0] * scalar,
                self.v[1] * scalar,
                self.v[2] * scalar
            ]
        }
    }
}

impl<'a> ops::Mul<f32> for &'a Vec3  {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            v: [self.v[0] * scalar,
                self.v[1] * scalar,
                self.v[2] * scalar
            ]
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, v: &Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self {
            v: [self.v[0] / scalar,
                self.v[1] / scalar,
                self.v[2] / scalar
            ]
        }
    }
}