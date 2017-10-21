#[macro_use] extern crate nickel;
#[macro_use] extern crate serde_json;

use nickel::{Nickel, HttpRouter};
use std::sync::{Arc, Mutex};

mod grid;
use grid::Grid;

fn handle_clear(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/new", middleware! {
        grid_clone.lock().unwrap().clear();
        "cleared!"
    });
}

fn handle_set(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/:x/:y", middleware! { |request|
        grid_clone.lock().unwrap().set_mut(
            request.param("x").unwrap().parse().unwrap(),
            request.param("y").unwrap().parse().unwrap(),
            true
        );
        //grid_clone.lock().unwrap().print();
        "set!"
    });
}

fn handle_get(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.get("/:x/:y", middleware! { |request|
        json!({
            "value": grid_clone.lock().unwrap().get(
                request.param("x").unwrap().parse().unwrap(),
                request.param("y").unwrap().parse().unwrap()
            ).unwrap(),
        }).to_string()
    });
}

fn handle_step(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/step", middleware! { |request|
        grid_clone.lock().unwrap().step_mut();
        "stepped!"
    });
}

fn handle_randomize(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/randomize", middleware! { |request|
        grid_clone.lock().unwrap().randomize();
        "radomized!"
    });
}

fn handle_show(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.get("/", middleware! { |request|
        let grid_slice = json!(grid_clone.lock().unwrap().get_grid());
        format!("{}", grid_slice)
    });
}

fn main() {
    let mut grid = Arc::new(Mutex::new(Grid::new(4, 4)));
    //grid.lock().unwrap().randomize();
    //grid.lock().unwrap().run();

    let mut server = Nickel::new();
    handle_clear(&mut server, &mut grid);
    handle_set(&mut server, &mut grid);
    handle_get(&mut server, &mut grid);
    handle_step(&mut server, &mut grid);
    handle_randomize(&mut server, &mut grid);
    handle_show(&mut server, &mut grid);

    server.listen("localhost:3000");
}
