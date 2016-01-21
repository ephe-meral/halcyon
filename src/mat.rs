
/// Create a new (zeroed) matrix.
/// Dimensions are `dim_x`x`dim_y`.
#[macro_export]
macro_rules! mat {
    ($dim_x:expr, $dim_y:expr) => {
        [[0.0; $dim_x]; $dim_y]
    }
}

/// Creates an array of base vectors for
/// any given dimension.
/// Can also be regarded as the identity matrix
/// for that dimension.
#[macro_export]
macro_rules! base {
    ($dim:expr) => {{
        let mut base = mat!($dim, $dim);
        for i in 0..$dim { base[i][i] = 1.0 }
        base
    }}
}
