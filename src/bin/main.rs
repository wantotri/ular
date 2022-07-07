use ular::ular::*;

fn main() {
    let mut game =
        Game::new(10, 10, "⬛", "🥓", "🟧", "⬜");
    println!("{}", game);

    game.spawn_food();
    game.tick();
    game.set_snake_direction(Direction::Up);
    println!("{}", game);
    game.tick();
    game.set_snake_direction(Direction::Left);
    println!("{}", game);
    game.tick();
    game.set_snake_direction(Direction::Right);
    println!("{}", game);
    game.tick();
    if game.is_over() { println!("Game Over"); };
    println!("{}", game);
}
