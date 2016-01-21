
/// Create a new (zeroed) matrix.
/// Dimensions are [`dim_m`, `dim_n`].
/// (m are the rows, n the columns, each row is an array)
///
/// # Examples
/// `let a = mat![3, 4]`
#[macro_export]
macro_rules! mat {
    [$dim_x:expr, $dim_y:expr] => {
        [[0.0; $dim_y]; $dim_x]
    }
}

/// Creates an array of base vectors for
/// any given dimension.
/// Can also be regarded as the identity matrix
/// for that dimension.
#[macro_export]
macro_rules! base {
    ($dim:expr) => {{
        let mut base = mat![$dim, $dim];
        for i in 0..$dim { base[i][i] = 1.0 }
        base
    }}
}

/// Statically generates a multiplication of two arbitrarily sized matrices.
/// The first matrix' m dimension needs to be the same as the second's n dimension.
#[macro_export]
macro_rules! mul {
    ($mat_a:ident: [$dim_i:expr, $dim_k1:expr], $mat_b:ident: [$dim_k2:expr, $dim_j:expr]) => {{
        assert_eq!($dim_k1, $dim_k2);
        let mut res = mat![$dim_i, $dim_j];
        for i in 0..$dim_i { for j in 0..$dim_j { for k in 0..$dim_k1 {
            res[i][j] += $mat_a[i][k] * $mat_b[k][j];
        }}}
        res
    }}
}
