use glam::{Vec2, Vec4};
use miniquad::{Bindings, Buffer, BufferType, Context, Pipeline, Shader, Texture};

pub fn render_image(
    ctx: &mut Context,
    tex: Texture,
    src: Vec4,
    dst: Vec4,
    pos: Vec2,
    rot: f32,
    pipeline: &Pipeline,
    shader: &Shader,
){
    let tex_w = tex.width as f32;
    let tex_h = tex.height as f32;

    //UV
    let u0 = src.x / tex_w;
    let v0 = src.y / tex_h;
    let u1 = (src.x + src.w) / tex_w;
    let v1 = (src.x + src.z) / tex_h;

    //Quad temporário
    let hw = dst.w /2.0;
    let hh = dst.z /2.0;

    let mut verts: [(f32, f32, f32, f32); 4] = [
        (-hw, -hh, u0, v1),
        ( hw, -hh, u1, v1),
        ( hw,  hh, u1, v0),
        (-hw,  hh, u0, v0),
    ];

    let sin = rot.sin();
    let cos = rot.cos();

    // rot e pos
    for vert in verts.iter_mut() {
        let (x,y,u,v) = *vert;
        let xr = x * cos - y * sin + pos.x;
        let yr = x * sin + y * cos + pos.y;
        *vert = (xr, yr, u, v);
    }

    let verts: Vec<f32> = verts.iter()
        .flat_map(|(x,y,u,v)| vec![*x,*y,*u,*v])
        .collect();

    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, bytemuck::cast_slice(&verts));
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, bytemuck::cast_slice(&indices));

    let bindings = Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![tex],
    };

    ctx.apply_pipeline(pipeline);
    ctx.apply_bindings(&bindings);
    ctx.draw(0, 6, 1);
}