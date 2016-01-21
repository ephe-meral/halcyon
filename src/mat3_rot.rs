
/// Creates a 3D rotation matrix.
/// Rotates counterclockwise around axis e3, e2' and finally e1''
/// We calculate a roation matrix for each and multiply them
/// in the end in that order.
pub fn rot(agl1: f32, agl2: f32, agl3: f32) -> [[f32; 3]; 3] {
    let mat3_a = 
        [[ agl1.cos(), agl1.sin(), 0.0],
         [-agl1.sin(), agl1.cos(), 0.0],
         [        0.0,        0.0, 1.0]];

    let mat3_b =
        [[ agl2.cos(), 0.0, agl2.sin()],
         [        0.0, 1.0,        0.0],
         [-agl2.sin(), 0.0, agl2.cos()]];

    let mat3_c =
        [[1.0,         0.0,        0.0],
         [0.0,  agl3.cos(), agl3.sin()],
         [0.0, -agl3.sin(), agl3.cos()]];

    let step1 = mul!(mat3_a: [[_; 3]; 3], mat3_b: [[_; 3]; 3]);
    mul!(step1: [[_; 3]; 3], mat3_c: [[_; 3]; 3])
}
