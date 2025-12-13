#[derive(Copy, Clone)]

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }

    pub fn get_z(&self) -> f64 {
        return self.z;
    }

    pub fn negate(&self) -> Vec3 {
        return Vec3::new(-self.x, -self.y, -self.z);
    }

    pub fn add(&self, v: Vec3) -> Vec3 {
        return Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z);
    }

    pub fn subtract(&self, v: Vec3) -> Vec3 {
        return Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z);
    }

    pub fn multiply_vec(&self, v: Vec3) -> Vec3 {
        return Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z);
    }

    pub fn multiply_scalar(&self, t: f64) -> Vec3 {
        return Vec3::new(self.x * t, self.y * t, self.z * t);
    }

    pub fn divide(&self, t: f64) -> Vec3 {
        return self.multiply_scalar(1.0 / t);
    }

    pub fn add_assign(&mut self, v: Vec3) -> &mut Vec3 {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        return self;
    }

    pub fn multiply_assign(&mut self, t: f64) -> &mut Vec3 {
        self.x *= t;
        self.y *= t;
        self.z *= t;
        return self;
    }

    pub fn divide_assign(&mut self, t: f64) -> &mut Vec3 {
        return self.multiply_assign(1.0 / t);
    }

    pub fn length(&self) -> f64 {
        return self.length_squared();
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        return Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        );
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        return v.divide(v.length());
    }

    pub fn to_string(&self) -> String {
        return format!("{} {} {}", self.x, self.y, self.z);
    }
}
