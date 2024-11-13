use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
};

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    // display as 2 * 3 as {1 2 3, 4 5 6} , 3 * 2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix {{ rows: {}, cols: {}, data: {} }}",
            self.rows, self.cols, self
        )
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default,
{
    // 矩阵乘法要求第一个矩阵的列数等于第二个矩阵的行数
    // 否则无法完成矩阵相乘运算
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Invalid matrix dimensions"));
    }

    let mut data = vec![T::default(); a.rows * b.cols];
    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() -> anyhow::Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![10, 11, 20, 21, 30, 31], 3, 2);
        let c = multiply(&a, &b)?;

        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");

        assert_eq!(
            format!("{:?}", a),
            "Matrix { rows: 2, cols: 3, data: {1 2 3, 4 5 6} }"
        );

        assert_eq!(
            format!("{:?}", b),
            "Matrix { rows: 3, cols: 2, data: {10 11, 20 21, 30 31} }"
        );

        assert_eq!(
            format!("{:?}", c),
            "Matrix { rows: 2, cols: 2, data: {140 146, 320 335} }"
        );

        Ok(())
    }
}
