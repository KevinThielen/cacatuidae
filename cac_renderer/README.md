# Cacatuidae Renderer 

Cross platform renderer with multiple backends

Provides an abstraction over common graphics backends like OpenGL,
WebGL, and even Wgpu.

Why another abstraction over Wgpu?
Simplicity! Calling renderer.draw(), renderer.create_mesh(), etc.
is just as simple as it sounds. Behind the scenes, draw calls can be batched,
sorted on multiple threads and more.
