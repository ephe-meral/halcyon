// new rotation matrix for rotation around e3 and then around e1 base vectors.
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

    return mul_mat_mat(mul_mat_mat(mat3_a, mat3_b), mat3_c);
}

fn mul_mat_mat(mat3_a: [[f32; 3]; 3], mat3_b: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    let mut res = [[0.0; 3]; 3];
    for i in 0..3 { for j in 0..3 { for k in 0..3 {
        res[i][j] += mat3_a[i][k] * mat3_b[k][j];
    }}}
    res
}

// TODO obviously unify & DRY
pub fn mul_mat_vec(mat3_a: [[f32; 3]; 3], vec3_b: [f32; 3]) -> [f32; 3] {
    let mut res = [0.0; 3];
    for i in 0..3 { for k in 0..3 {
        res[i] += mat3_a[i][k] * vec3_b[k];
    }}
    res
}
