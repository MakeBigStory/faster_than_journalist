

use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es31;
use gles::es30;
use gles::es32;


#[derive(Copy, Debug, Clone, Drop)]
pub struct Buffer {
    pub name: String,
    target: es20d::GLenum,
    usage: es20d::GLenum,
    size: es20d::GLsizeiptr,
    raw: GLuint,
}

impl Buffer {
    fn new(name: String, target: es20d::GLenum, usage: es20d::GLenum) -> Self {
        Buffer {
            name,
            target,
            usage,
            raw: es20::wrapper::gen_buffers(1)[0],
            size: 0,
        }
    }


}