pub mod matrix {
    use std::ops::Mul;
    #[derive(Debug)]
    pub struct Matrix {
        mat1 :[f32;4],
        mat2 :[f32;4],
        mat3 :[f32;4],
        mat4 :[f32;4],
        mat5 :[f32;4],
        mat6 :[f32;4],
        mat7 :[f32;4],
        mat8 :[f32;4],
    }
    impl Matrix{
        pub fn mat4
        (
            m11 :f32, m12:f32, m13: f32, m14: f32,
            m21 :f32, m22:f32, m23: f32, m24: f32,
            m31 :f32, m32:f32, m33: f32, m34: f32,
            m41 :f32, m42:f32, m43: f32, m44: f32
        ) -> Matrix
        {
            Matrix {
                mat1 : [m11,m21,m31,m41],
                mat2 : [m12,m22,m32,m42],
                mat3 : [m13,m23,m33,m43],
                mat4 : [m14,m24,m34,m44],
                ////////////////////////
                mat5 : [m11,m21,m31,m41],
                mat6 : [m21,m22,m23,m24],
                mat7 : [m31,m32,m33,m34],
                mat8 : [m41,m42,m43,m44],
            }
        }

        pub fn translate (x: f32, y: f32, z: f32) -> Matrix {
            let mut mat4 = Matrix::get_simple_matrix_4();
            mat4[0][3] = x;
            mat4[1][3] = y;
            mat4[2][3] = z;

            Matrix::set_matrix(mat4)
        }

//        pub fn rolate () -> Matrix {
//
//        }

        fn set_matrix(vec : Vec<Vec<f32>>) -> Matrix{
            Matrix::mat4(
                vec[0][0],vec[0][1],vec[0][2],vec[0][3],
                vec[1][0],vec[1][1],vec[1][2],vec[1][3],
                vec[2][0],vec[2][1],vec[2][2],vec[2][3],
                vec[3][0],vec[3][1],vec[3][2],vec[3][3],
            )
        }

        pub fn get_matrix (&self) -> Vec<f32> {
            vec![
                self.mat1[0],self.mat1[1],self.mat1[2],self.mat1[3],
                self.mat2[0],self.mat2[1],self.mat2[2],self.mat2[3],
                self.mat3[0],self.mat3[1],self.mat3[2],self.mat3[3],
                self.mat4[0],self.mat4[1],self.mat4[2],self.mat4[3],
            ]
        }

        fn get_simple_matrix_4 () ->Vec<Vec<f32>>{
            vec!
            [
                vec![1.0,0.0,0.0,0.0],
                vec![0.0,1.0,0.0,0.0],
                vec![0.0,0.0,1.0,0.0],
                vec![0.0,0.0,0.0,1.0],
            ]

        }
        pub fn vector_mul(vec1:&[f32],vec2:&[f32]) -> Vec<f32>{
            let mut m_index = 0;
            let mut m = 0.;
            let mut vec: Vec<f32> = Vec::new();
            for i in 0..4 {
                m = vec1[i] * vec2[i];
                vec.push(m);
            }
            vec
        }
        pub fn vector_add(vec1:&[f32],vec2:&[f32]) -> Vec<f32>{
            let mut m_index = 0;
            let mut m = 0.;
            let mut vec = Vec::new();
            for i in 0..4 {
                m = vec1[i] + vec2[i];
                vec.push(m);
            }
            vec
        }
        pub fn matrix_mul (mat1: Matrix, mat2: Matrix) -> Vec<Vec<f32>> {
            let m1 = Matrix::vector_mul(&mat1.mat1,&mat2.mat1);
            let m2 = Matrix::vector_mul(&mat1.mat2,&mat2.mat2);
            let m3 = Matrix::vector_mul(&mat1.mat3,&mat2.mat3);
            let m4 = Matrix::vector_mul(&mat1.mat4,&mat2.mat4);
            let vec = vec![m1,m2,m3,m4];
            println!("vec: {:?},",vec);
            vec
        }

        pub fn matrix_add (mat1: Matrix, mat2: Matrix) -> Vec<Vec<f32>> {

            let m1 = Matrix::vector_add(&mat1.mat1,&mat2.mat1);
            let m2 = Matrix::vector_add(&mat1.mat2,&mat2.mat2);
            let m3 = Matrix::vector_add(&mat1.mat3,&mat2.mat3);
            let m4 = Matrix::vector_add(&mat1.mat4,&mat2.mat4);
            let vec =vec![m1,m2,m3,m4] ;
            println!("vec: {:?},",vec);
            vec
        }
    }
    impl Mul for Matrix {
        type Output = Self;
        fn mul(self, other: Self) -> Self{
            let vec = Matrix::matrix_mul(self,other);
            Matrix::mat4(
                vec[0][0],vec[0][1],vec[0][2],vec[0][3],
                vec[1][0],vec[1][1],vec[1][2],vec[1][3],
                vec[2][0],vec[2][1],vec[2][2],vec[2][3],
                vec[3][0],vec[3][1],vec[3][2],vec[3][3],
            )
        }
    }
}