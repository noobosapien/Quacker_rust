pub struct Mat4<T> {
    m11: T,
    m21: T,
    m31: T,
    m41: T,
    m12: T,
    m22: T,
    m32: T,
    m42: T,
    m13: T,
    m23: T,
    m33: T,
    m43: T,
    m14: T,
    m24: T,
    m34: T,
    m44: T,
}

pub type mat4 = Mat4<f32>;
impl<T> Mat4<T> {}
