
use std::ops;

#[derive(Copy, Clone)]
struct Vec3 {
    v: [f32; 3]
}

impl Vec3 {
    fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            v: [x, y, z]
        }
    }

    fn x(&self) -> f32 {
        self.v[0]
    }
    fn y(&self) -> f32 {
        self.v[1]
    }
    fn z(&self) -> f32 {
        self.v[2]
    }

    fn r(&self) -> f32 {
        self.v[0]
    }
    fn g(&self) -> f32 {
        self.v[1]
    }
    fn b(&self) -> f32 {
        self.v[2]
    }

    fn length_sqr(&self) -> f32 {
        (self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2])
    }

    fn length(&self) -> f32 {
        self.length_sqr().sqrt()
    }

    fn normalize(&mut self) {
        let len = self.length();
        self.v[0] /= len;
        self.v[1] /= len;
        self.v[2] /= len;
    }

    fn normalized(self) -> Vec3 {
        self / self.length()
    }

    fn dot(&self, other: &Vec3) -> f32 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
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



fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
