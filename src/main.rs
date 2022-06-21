use macroquad::prelude::*;
use macroquad::window;

const GRID_CELLS: i16 = 32;

struct Cell {
    x: i16,
    y: i16,
}

struct Muell {
    x: i16,
    y: i16,
    size: i16,
    shown: bool,
}

struct Bewohner {
    x: i16,
    y: i16,
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
    const GRID_SIZE: f32 = 1000 / GRID_CELLS;
    let mut player = Bewohner {
        x: screen_width() as i16 / 2,
        y: screen_height() as i16 / 2,
        size: 10,
        speed: GRID_SIZE,
    };

    let mut muells: Vec<Muell> = Vec::new();

    loop {
        clear_background(BLUE);

        muells.push(Muell {
            x: 30,
            y: 30,
            size: 10,
            shown: true,
        });

        if is_key_down(KeyCode::Right) && !(player.x + 2 * player.size > screen_width() as i16) {
            player.x += player.speed;
        } else if is_key_down(KeyCode::Left) && !(player.x - player.size < 0) {
            player.x -= player.speed;
        } else if is_key_down(KeyCode::Up) && !(player.y - player.size < 0) {
            player.y -= player.speed;
        } else if is_key_down(KeyCode::Down)
            && !(player.y + 2 * player.size > screen_height() as i16)
        {
            player.y += player.speed;
        }

        draw_rectangle(
            player.x.into(),
            player.y.into(),
            player.size.into(),
            player.size.into(),
            BLACK,
        );
        for m in muells.iter() {
            if m.shown {
                draw_rectangle(m.x.into(), m.y.into(), m.size.into(), m.size.into(), RED)
            }
        }

        next_frame().await
    }
}
