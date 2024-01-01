use std::thread;
use std::time::Duration;
use astar::{MazeMap, Node, Tile, Vector2};

struct Context {
    map: MazeMap,
    agent_pos: Vector2,
    fps: i32,
    is_running: bool
}

fn main() {
    let mut ctx = init();

    while ctx.is_running {
        update(&mut ctx);
        render(&mut ctx);
    }
}

fn init() -> Context {
    let context = Context {
        map: MazeMap::generate(Vector2::new(20, 10)),
        agent_pos: Vector2::new(0, 0),
        fps: 30,
        is_running: true
    };
    context
}

fn update(ctx: &mut Context) {
    thread::sleep(Duration::from_secs(2));
    ctx.agent_pos = ctx.agent_pos.right();
}

fn render(ctx: &Context) {
    let size = ctx.map.size();

    for y in 0..size.y {
        for x in 0..size.x {
            let pos = Vector2::new(x, y);
            if ctx.agent_pos == pos {
                print!("{}", '&');
            } else {
                let tile = ctx.map.get_tile(pos);
                print!("{}", render_tile(tile));
            }
        }
        print!("\r\n");
    }
}

fn render_tile(tile: &Tile) -> char {
    match tile {
        Tile::Blank => ' ',
        Tile::Floor => '☐',
        Tile::Goal => '✩',
        Tile::StartingPoint => '@',
        Tile::Wall => '◼',
        _ => '?'
    }
}