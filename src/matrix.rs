use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::vector::{dot_produc, Vector};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    rows: Vector<T>,
    cols: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, rows: Vector<T>, cols: Vector<T>) -> Self {
        Self { idx, rows, cols }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
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

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        multiply(&self, &rhs).expect("Failed to multiply matrices")
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
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    // 矩阵乘法要求第一个矩阵的列数等于第二个矩阵的行数
    // 否则无法完成矩阵相乘运算
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Invalid matrix dimensions"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_produc(msg.input.rows, msg.input.cols)?;
                    msg.sender
                        .send(MsgOutput {
                            idx: msg.input.idx,
                            value,
                        })
                        .map_err(|e| {
                            anyhow::anyhow!("Failed to send dot product result: {:?}", e)
                        })?;
                }
                Ok::<_, anyhow::Error>(())
            });

            tx
        })
        .collect::<Vec<_>>();

    // let (tx, rx) = mpsc::channel();

    let matrix_len = a.rows * b.cols;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let idx = i * b.cols + j;
            let a_rows = a.data[i * a.cols..(i + 1) * a.cols].to_vec();
            let b_cols = b.data[j..b.cols * b.rows]
                .iter()
                .step_by(b.cols)
                .copied()
                .collect::<Vec<_>>();

            let input = MsgInput::new(idx, Vector::new(a_rows), Vector::new(b_cols));
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            senders[idx % NUM_THREADS]
                .send(msg)
                .map_err(|e| anyhow::anyhow!("Failed to send message to thread: {:?}", e))?;
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let result = rx.recv()?;
        data[result.idx] = result.value;
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
        // let c = multiply(&a, &b)?;
        let c = a * b;

        assert_eq!(c.data, vec![140, 146, 320, 335],);
        assert_eq!(format!("{}", c), "{140 146, 320 335}");

        Ok(())
    }

    #[test]
    fn test_a_can_multiply_b() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![10, 11, 20, 21, 30, 31], 2, 2);
        let c = multiply(&a, &b);

        assert!(c.is_err());
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![10, 11, 20, 21, 30, 31], 2, 2);
        let _ = a * b;
    }
}
