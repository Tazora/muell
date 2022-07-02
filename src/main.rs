use macroquad::prelude::*;
use macroquad::window;

const GRID_CELLS: i16 = 16;
const SPEED: i16 = 1;

type Point = (i16, i16);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Visibility {
    Show,
    Hide,
}

#[derive(Copy, Clone, Debug)]
struct Muell {
    koord: Point,
    shown: Visibility,
}

impl Muell {
    fn new(koord: Point) -> Self {
        Self {
            koord,
            shown: Visibility::Show,
        }
    }
}

trait Collect {
    fn take(&mut self) -> Self;
    fn drop(self, koord: Point) -> Self;
}

impl Collect for Muell {
    fn take(&mut self) -> Self {
        self.shown = Visibility::Hide;
        *self
    }

    fn drop(mut self, koord: Point) -> Self {
        self.koord = koord;
        self.koord.1 += 1;
        self.shown = Visibility::Show;
        self
    }
}

struct Bewohner {
    koord: Point,
    speed: f32,
    muells: Vec<Muell>,
}

impl Bewohner {
    fn new(koord: Point, speed: f32) -> Self {
        Self {
            koord,
            speed,
            muells: Vec::new(),
        }
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Müllmann".to_owned(),
        high_dpi: true,
        window_height: 1000,
        window_width: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);

    let player_stats_text: String = "Müllstand: ".to_owned();

    let mut player = Bewohner::new((3, 3), 1.);
    let mut muells: Vec<Muell> = Vec::new();
    let mut last_spawn_update = get_time();
    let mut last_update = get_time();
    let spawn_speed = SPEED * 3;
    muells.push(Muell::new((10, 12)));

    loop {
        clear_background(BLUE);

        if get_time() - last_spawn_update > spawn_speed as f64 {
            last_spawn_update = get_time();
            muells.push(Muell::new((
                rand::gen_range(1, GRID_CELLS - 1),
                rand::gen_range(1, GRID_CELLS - 1),
            )));
        }

        if get_time() - last_update > player.speed as f64 / 10. {
            last_update = get_time();

            if is_key_down(KeyCode::Right) && !(player.koord.0 + 2 > GRID_CELLS) {
                player.koord.0 += 1;
            } else if is_key_down(KeyCode::Left) && !(player.koord.0 < 1) {
                player.koord.0 -= 1;
            } else if is_key_down(KeyCode::Up) && !(player.koord.1 < 1) {
                player.koord.1 -= 1;
            } else if is_key_down(KeyCode::Down) && !(player.koord.1 + 2 > GRID_CELLS) {
                player.koord.1 += 1;
            }
            if is_key_down(KeyCode::A) && !player.muells.is_empty() {
                let muell = player.muells.pop();
                match muell {
                    Some(muell) => {
                        muells.push(muell.drop(player.koord));
                        player.speed -= 0.1;
                    }
                    None => break,
                }
            }

            muells.retain_mut(|m| {
                if m.koord == player.koord {
                    player.muells.push(m.take());
                    player.speed += 0.1;
                    false
                } else {
                    true
                }
            });
        }

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2.;
        let offset_y = (screen_height() - game_size) / 2.;
        let sq_size = (screen_height() - offset_y * 2.) / GRID_CELLS as f32;

        draw_grid_game(offset_x, offset_y, sq_size);
        draw_muells(offset_x, offset_y, sq_size, &muells);
        draw_player(offset_x, offset_y, sq_size, &player);

        draw_text(
            &(player_stats_text.clone() + &player.muells.len().to_string()),
            64.,
            64.,
            64.,
            BLACK,
        );

        next_frame().await
    }
}

fn draw_grid_game(offset_x: f32, offset_y: f32, sq_size: f32) {
    for i in 1..GRID_CELLS {
        draw_line(
            offset_x,
            offset_y + sq_size * i as f32,
            screen_width() - offset_x,
            offset_y + sq_size * i as f32,
            1.,
            PINK,
        );
    }

    for i in 1..GRID_CELLS {
        draw_line(
            offset_x + sq_size * i as f32,
            offset_y,
            offset_x + sq_size * i as f32,
            screen_height() - offset_y,
            1.,
            LIGHTGRAY,
        );
    }
}

fn draw_muells(offset_x: f32, offset_y: f32, sq_size: f32, muells: &Vec<Muell>) {
    for m in muells {
        if m.shown == Visibility::Show {
            draw_rectangle(
                offset_x + m.koord.0 as f32 * sq_size + sq_size * 0.25,
                offset_y + m.koord.1 as f32 * sq_size + sq_size * 0.25,
                sq_size * 0.5,
                sq_size * 0.5,
                RED,
            )
        }
    }
}

fn draw_player(offset_x: f32, offset_y: f32, sq_size: f32, player: &Bewohner) {
    draw_rectangle(
        offset_x + player.koord.0 as f32 * sq_size,
        offset_y + player.koord.1 as f32 * sq_size,
        sq_size,
        sq_size,
        BLACK,
    );
}
