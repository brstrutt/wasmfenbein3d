use crate::{
    primitives::point2d::Point2D,
    render::{rgb::RGB, screen::ScreenBuffer, textures::Textures},
    state::GameState,
    web::{self, main_canvas},
    world::walls::WALL_HEIGHT,
};
use std::{cell::RefCell, rc::Rc};

pub mod rgb;
pub mod screen;
pub mod texture;
pub mod textures;

pub fn setup(
    state: Rc<RefCell<GameState>>,
    screen_buffer: Rc<RefCell<ScreenBuffer>>,
    textures: Rc<RefCell<Textures>>,
) {
    web::window::run_function_every_animation_frame(move || {
        render(&screen_buffer, &state, &textures);
    });
}

pub fn render(
    screen_buffer: &Rc<RefCell<ScreenBuffer>>,
    state: &RefCell<GameState>,
    textures: &Rc<RefCell<Textures>>,
) {
    let render_start_time = web::window::now_in_ms();

    let mut state = state.borrow_mut();
    let mut screen_buffer = screen_buffer.borrow_mut();
    let textures = textures.borrow();

    screen_buffer.reset_draw_history();
    render_walls(&mut screen_buffer, &state, &textures);
    render_background(&mut screen_buffer, &state, &textures);

    main_canvas::render_screen_buffer(&screen_buffer);

    let render_end_time = web::window::now_in_ms();
    state.last_time_to_render_one_frame_ms = render_end_time - render_start_time;
}

fn render_background(screen_buffer: &mut ScreenBuffer, state: &GameState, textures: &Textures) {
    let camera = state.world.camera.clone();
    let half_screen_height = screen_buffer.height as f64 / 2.0;

    let half_wall_height = half_screen_height * WALL_HEIGHT;

    for y in 0..screen_buffer.height {
        let dist_to_floor = ((1.0 / (y as f64 - half_screen_height)) * half_wall_height).abs();

        screen_buffer.render_textured_row(
            &y,
            &camera,
            dist_to_floor,
            &textures.floor,
            (dist_to_floor / 5.0).max(1.0),
        );
    }
}

fn render_walls(screen_buffer: &mut ScreenBuffer, state: &GameState, textures: &Textures) {
    let screen_height = screen_buffer.height as f64;

    for x in 0..screen_buffer.width {
        let ray = state.world.camera.ray_for_column(x);
        let wall_intersection = state.world.nearest_wall_intersecting_ray(&ray);

        let mut height = 0.0;
        let mut wall_color_adjustment = 1.0;

        if let Some(wall_intersection) = wall_intersection {
            let distance = Point2D::dist(
                &state.world.camera.ray.origin,
                &wall_intersection.intersection,
            );
            wall_color_adjustment = (distance / 5.0).max(1.0);

            if distance != 0.0 {
                height = WALL_HEIGHT * screen_height / distance;
            }

            screen_buffer.render_textured_column(
                &x,
                height,
                &textures.wall,
                &wall_intersection,
                wall_color_adjustment,
            );
        } else {
            const NO_WALL_COLOUR: RGB = RGB {
                red: 0,
                green: 0,
                blue: 0,
            };
            screen_buffer.render_solid_colour_column(
                &x,
                0.0,
                &NO_WALL_COLOUR,
                wall_color_adjustment,
            );
        }
    }
}
