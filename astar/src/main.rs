use astar::{MazeMap, Node, Vector2};

struct Context {
    map: MazeMap,
    agent_pos: Vector2
}

fn main() {
    let fps = 30;

    let mut ctx = init();

    loop {
        update(&mut ctx);
        render(&mut ctx);
    }
}

fn init() -> Context {
    Context {
        map: MazeMap::generate(10, 10),
        agent_pos: Vector2::new(0, 0)
    }
}

fn update(ctx: &mut Context) {
}

fn render(ctx: &mut Context) {
}