use macroquad::prelude::*;
use macroquad::window;

const GRID_CELLS: i16 = 32;
const SPEED: f32 = 1.0;

type Point = (i16, i16);

struct Cell {
    koord: Point,
}

struct Muell {
    koord: Point,
    size: i16,
    shown: bool,
}

struct Bewohner {
    koord: Point,
    size: i16,
    speed: i16,
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Müllmann".to_owned(),
        high_dpi: true,
        window_height: 1000,
        window_width: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    const GRID_SIZE: i16 = 1000 / GRID_CELLS;
    let mut player = Bewohner {
        koord: (16, 16),
        size: 10,
        speed: GRID_SIZE,
    };

    let mut muells: Vec<Muell> = Vec::new();
    let mut last_update = get_time();

    loop {
        clear_background(BLUE);

        if get_time() - last_update > SPEED.into() {
            last_update = get_time();
            muells.push(Muell {
                koord: (30, 30),
                size: 10,
                shown: true,
            });

            if is_key_down(KeyCode::Right)
                && !(player.koord.0 + 2 * player.size > screen_width() as i16)
            {
                player.koord.0 += player.speed;
            } else if is_key_down(KeyCode::Left) && !(player.koord.0 - player.size < 0) {
                player.koord.0 -= player.speed;
            } else if is_key_down(KeyCode::Up) && !(player.koord.1 - player.size < 0) {
                player.koord.1 -= player.speed;
            } else if is_key_down(KeyCode::Down)
                && !(player.koord.1 + 2 * player.size > screen_height() as i16)
            {
                player.koord.1 += player.speed;
            }
        }

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / GRID_CELLS as f32;

        for i in 1..GRID_CELLS {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..GRID_CELLS {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        draw_rectangle(
            offset_x + player.koord.0 as f32 * sq_size,
            offset_y + player.koord.1 as f32 * sq_size,
            sq_size,
            sq_size,
            BLACK,
        );
        for m in muells.iter() {
            if m.shown {
                draw_rectangle(
                    m.koord.0.into(),
                    m.koord.1.into(),
                    m.size.into(),
                    m.size.into(),
                    RED,
                )
            }
        }

        next_frame().await
    }
}
