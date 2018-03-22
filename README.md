# ma_rgl

Mathematics rust OpenGL library absolutelly written in rust lang. Alternative OpenGL Mathematics

# Example
### Cargo:
```
ma_rgl = { git = "https://github.com/DixydL/ma_rgl.git" }
```
### Source:
```
use ma_rgl::matrix::Matrix;


set matrix Translate 1;
let matrix_1 = Matrix::translate(x,y,z);

set matrix Translate 2;

let matrix_2 = Matrix::translate(x,y,z);

let matrix_out = matrix_1 * matrix_2;

get matrix 4x4
let mat = matrix_out.get_matrix();
```
