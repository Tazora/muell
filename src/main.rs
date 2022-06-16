use macroquad::prelude::*;
use macroquad::rand;

struct Ball {
    xkord: f32,
    ykord: f32,
    radius: f32,
    color: Color,
    speed: f32,
}

#[macroquad::main("Müllmann")]
async fn main() {
    let mut ballplayer = Ball {
        xkord: screen_width() / 2.0,
        ykord: screen_height() / 2.0,
        radius: 10.0,
        color: BLACK,
        speed: 10.0,
    };

    let mut balls: Vec<Ball> = Vec::new();
    let mut gameover = false;

    balls.push(Ball {
        xkord: 50.0,
        ykord: 50.0,
        radius: 10.0,
        color: RED,
        speed: ((rand::rand() % 5) as f32) + 1.0 * 2.0,
    });

    balls.push(Ball {
        xkord: 150.0,
        ykord: 150.0,
        radius: 10.0,
        color: BEIGE,
        speed: ((rand::rand() % 5) as f32) + 1.0 * 2.0,
    });

    balls.push(Ball {
        xkord: 450.0,
        ykord: 450.0,
        radius: 10.0,
        color: YELLOW,
        speed: ((rand::rand() % 5) as f32) + 1.0 * 2.0,
    });

    loop {
        if gameover {
            break;
        }
        clear_background(BLUE);

        if is_key_down(KeyCode::Right) && !(ballplayer.xkord + ballplayer.radius > screen_width()) {
            ballplayer.xkord += ballplayer.speed;
        }
        if is_key_down(KeyCode::Left) && !(ballplayer.xkord - ballplayer.radius < 0.0) {
            ballplayer.xkord -= ballplayer.speed;
        }
        if is_key_down(KeyCode::Up) && !(ballplayer.ykord - ballplayer.radius < 0.0) {
            ballplayer.ykord -= ballplayer.speed;
        }
        if is_key_down(KeyCode::Down) && !(ballplayer.ykord + ballplayer.radius > screen_height()) {
            ballplayer.ykord += ballplayer.speed;
        }
        if is_key_down(KeyCode::N) && ballplayer.radius > 10.0 {
            ballplayer.radius -= 1.0;
        }
        if is_key_down(KeyCode::M) {
            ballplayer.radius += 1.0;
        }

        for ball in balls.iter_mut() {
            let dirx = (ballplayer.xkord - ball.xkord).signum();
            let diry = (ballplayer.ykord - ball.ykord).signum();
            ball.xkord += ball.speed * dirx;
            ball.ykord += ball.speed * diry;

            draw_ball(&ball);
            if (ball.xkord - ballplayer.xkord).abs() < ballplayer.radius / 2.0
                && (ball.ykord - ballplayer.ykord).abs() < ballplayer.radius / 2.0
            {
                gameover = true;
            }
        }

        draw_ball(&ballplayer);
        next_frame().await
    }
}

fn draw_ball(ball: &Ball) {
    draw_circle(ball.xkord, ball.ykord, ball.radius, ball.color);
}
