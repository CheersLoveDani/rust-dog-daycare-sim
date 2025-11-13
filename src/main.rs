use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{WindowEvent, ElementState, MouseButton},
    event_loop::EventLoop,
};
use winit::application::ApplicationHandler;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const SCALE: f32 = 0.2; // Diffusion rate
const ITER: usize = 16; // Solver iterations
const DT: f32 = 0.1; // Time step
const VISCOSITY: f32 = 0.0000001;

struct FluidGrid {
    size: usize,
    dt: f32,
    diff: f32,
    visc: f32,

    // Current state
    density: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,

    // Previous state
    density_prev: Vec<f32>,
    vx_prev: Vec<f32>,
    vy_prev: Vec<f32>,
}

impl FluidGrid {
    fn new(size: usize, diffusion: f32, viscosity: f32, dt: f32) -> Self {
        let grid_size = size * size;
        Self {
            size,
            dt,
            diff: diffusion,
            visc: viscosity,
            density: vec![0.0; grid_size],
            vx: vec![0.0; grid_size],
            vy: vec![0.0; grid_size],
            density_prev: vec![0.0; grid_size],
            vx_prev: vec![0.0; grid_size],
            vy_prev: vec![0.0; grid_size],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        let idx = self.index(x, y);
        self.density[idx] += amount;
    }

    fn add_velocity(&mut self, x: usize, y: usize, vx: f32, vy: f32) {
        let idx = self.index(x, y);
        self.vx[idx] += vx;
        self.vy[idx] += vy;
    }

    fn step(&mut self) {
        // Diffuse velocity
        Self::diffuse(1, &mut self.vx_prev, &self.vx, self.visc, self.dt, self.size, ITER);
        Self::diffuse(2, &mut self.vy_prev, &self.vy, self.visc, self.dt, self.size, ITER);

        // Project velocity (make divergence free)
        Self::project(&mut self.vx_prev, &mut self.vy_prev, &mut self.vx, &mut self.vy, self.size, ITER);

        // Advect velocity
        Self::advect(1, &mut self.vx, &self.vx_prev, &self.vx_prev, &self.vy_prev, self.dt, self.size);
        Self::advect(2, &mut self.vy, &self.vy_prev, &self.vx_prev, &self.vy_prev, self.dt, self.size);

        // Project velocity again
        Self::project(&mut self.vx, &mut self.vy, &mut self.vx_prev, &mut self.vy_prev, self.size, ITER);

        // Diffuse density
        Self::diffuse(0, &mut self.density_prev, &self.density, self.diff, self.dt, self.size, ITER);

        // Advect density
        Self::advect(0, &mut self.density, &self.density_prev, &self.vx, &self.vy, self.dt, self.size);

        // Fade density slightly over time
        for d in self.density.iter_mut() {
            *d *= 0.995;
        }
    }

    fn diffuse(b: i32, x: &mut Vec<f32>, x0: &Vec<f32>, diff: f32, dt: f32, size: usize, iter: usize) {
        let a = dt * diff * (size - 2) as f32 * (size - 2) as f32;
        Self::lin_solve(b, x, x0, a, 1.0 + 6.0 * a, iter, size);
    }

    fn lin_solve(b: i32, x: &mut Vec<f32>, x0: &Vec<f32>, a: f32, c: f32, iter: usize, size: usize) {
        let c_recip = 1.0 / c;

        for _ in 0..iter {
            for j in 1..size-1 {
                for i in 1..size-1 {
                    let idx = i + j * size;
                    x[idx] = (x0[idx] + a * (
                        x[idx - 1] +
                        x[idx + 1] +
                        x[idx - size] +
                        x[idx + size]
                    )) * c_recip;
                }
            }
            Self::set_bnd(b, x, size);
        }
    }

    fn project(vx: &mut Vec<f32>, vy: &mut Vec<f32>, p: &mut Vec<f32>, div: &mut Vec<f32>, size: usize, iter: usize) {
        // Calculate divergence
        for j in 1..size-1 {
            for i in 1..size-1 {
                let idx = i + j * size;
                div[idx] = -0.5 * (
                    vx[idx + 1] - vx[idx - 1] +
                    vy[idx + size] - vy[idx - size]
                ) / size as f32;
                p[idx] = 0.0;
            }
        }
        Self::set_bnd(0, div, size);
        Self::set_bnd(0, p, size);
        Self::lin_solve(0, p, div, 1.0, 6.0, iter, size);

        // Subtract gradient
        for j in 1..size-1 {
            for i in 1..size-1 {
                let idx = i + j * size;
                vx[idx] -= 0.5 * (p[idx + 1] - p[idx - 1]) * size as f32;
                vy[idx] -= 0.5 * (p[idx + size] - p[idx - size]) * size as f32;
            }
        }
        Self::set_bnd(1, vx, size);
        Self::set_bnd(2, vy, size);
    }

    fn advect(b: i32, d: &mut Vec<f32>, d0: &Vec<f32>, vx: &Vec<f32>, vy: &Vec<f32>, dt: f32, size: usize) {
        let dtx = dt * (size - 2) as f32;
        let dty = dt * (size - 2) as f32;

        for j in 1..size-1 {
            for i in 1..size-1 {
                let idx = i + j * size;

                let mut x = i as f32 - dtx * vx[idx];
                let mut y = j as f32 - dty * vy[idx];

                x = x.max(0.5).min(size as f32 - 1.5);
                y = y.max(0.5).min(size as f32 - 1.5);

                let i0 = x.floor() as usize;
                let i1 = i0 + 1;
                let j0 = y.floor() as usize;
                let j1 = j0 + 1;

                let s1 = x - i0 as f32;
                let s0 = 1.0 - s1;
                let t1 = y - j0 as f32;
                let t0 = 1.0 - t1;

                d[idx] = s0 * (t0 * d0[i0 + j0 * size] + t1 * d0[i0 + j1 * size]) +
                         s1 * (t0 * d0[i1 + j0 * size] + t1 * d0[i1 + j1 * size]);
            }
        }
        Self::set_bnd(b, d, size);
    }

    fn set_bnd(b: i32, x: &mut Vec<f32>, size: usize) {
        for i in 1..size-1 {
            x[i] = if b == 2 { -x[i + size] } else { x[i + size] };
            x[i + (size - 1) * size] = if b == 2 { -x[i + (size - 2) * size] } else { x[i + (size - 2) * size] };
        }

        for j in 1..size-1 {
            x[j * size] = if b == 1 { -x[1 + j * size] } else { x[1 + j * size] };
            x[size - 1 + j * size] = if b == 1 { -x[size - 2 + j * size] } else { x[size - 2 + j * size] };
        }

        x[0] = 0.5 * (x[1] + x[size]);
        x[size - 1] = 0.5 * (x[size - 2] + x[2 * size - 1]);
        x[(size - 1) * size] = 0.5 * (x[1 + (size - 1) * size] + x[(size - 2) * size]);
        x[size - 1 + (size - 1) * size] = 0.5 * (x[size - 2 + (size - 1) * size] + x[size - 1 + (size - 2) * size]);
    }

    fn render(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let density = self.density[i].min(255.0).max(0.0) as u8;

            // Create a colorful fluid effect
            let color = if density > 0 {
                // Interpolate between blue -> cyan -> white based on density
                let t = (density as f32 / 255.0).powf(0.7);
                let r = (t * 255.0) as u8;
                let g = (t * 255.0) as u8;
                let b = 255;
                [r, g, b, 0xff]
            } else {
                [0x00, 0x00, 0x10, 0xff] // Dark blue background
            };

            pixel.copy_from_slice(&color);
        }
    }
}

struct App {
    fluid: FluidGrid,
    mouse_pos: Option<(f32, f32)>,
    prev_mouse_pos: Option<(f32, f32)>,
    mouse_pressed: bool,
    window: Option<&'static winit::window::Window>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    fn new() -> Self {
        Self {
            fluid: FluidGrid::new(WIDTH, SCALE, VISCOSITY, DT),
            mouse_pos: None,
            prev_mouse_pos: None,
            mouse_pressed: false,
            window: None,
            pixels: None,
        }
    }

    fn update(&mut self) {
        // Add density and velocity at mouse position
        if let (Some((x, y)), Some((px, py))) = (self.mouse_pos, self.prev_mouse_pos) {
            if self.mouse_pressed {
                let grid_x = (x as usize).min(WIDTH - 1);
                let grid_y = (y as usize).min(HEIGHT - 1);

                // Add density in a small circle
                let radius = 3;
                for dy in -(radius as i32)..=radius {
                    for dx in -(radius as i32)..=radius {
                        if dx * dx + dy * dy <= radius * radius {
                            let nx = (grid_x as i32 + dx).max(1).min(WIDTH as i32 - 2) as usize;
                            let ny = (grid_y as i32 + dy).max(1).min(HEIGHT as i32 - 2) as usize;
                            self.fluid.add_density(nx, ny, 200.0);
                        }
                    }
                }

                // Add velocity based on mouse movement
                let vx = (x - px) * 10.0;
                let vy = (y - py) * 10.0;
                self.fluid.add_velocity(grid_x, grid_y, vx, vy);
            }
        }

        self.fluid.step();
        self.prev_mouse_pos = self.mouse_pos;
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = winit::window::Window::default_attributes()
                .with_title("Fluid Simulation - Click and drag to add fluid")
                .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
                .with_min_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64));

            let window = event_loop.create_window(window_attributes).unwrap();

            // Leak the window to get a 'static reference
            // This is acceptable for a single window application that runs until exit
            let window: &'static winit::window::Window = Box::leak(Box::new(window));

            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
            let pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

            self.window = Some(window);
            self.pixels = Some(pixels);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                self.mouse_pressed = state == ElementState::Pressed;
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(window) = &self.window {
                    let scale_x = WIDTH as f64 / window.inner_size().width as f64;
                    let scale_y = HEIGHT as f64 / window.inner_size().height as f64;
                    self.mouse_pos = Some((
                        (position.x * scale_x) as f32,
                        (position.y * scale_y) as f32,
                    ));
                }
            }
            WindowEvent::RedrawRequested => {
                self.update();
                if let Some(pixels) = &mut self.pixels {
                    self.fluid.render(pixels.frame_mut());
                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}
