#[macro_use] extern crate nickel;
#[macro_use] extern crate serde_json;
extern crate hyper;

use nickel::{Nickel, HttpRouter};
use std::sync::{Arc, Mutex};
use hyper::header::{AccessControlAllowOrigin};

mod grid;
use grid::Grid;

fn handle_clear(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/clear", middleware! { |_, mut response|
        response.set(AccessControlAllowOrigin::Any);
        grid_clone.lock().unwrap().clear();
        "cleared!"
    });
}

fn handle_set(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/:x/:y", middleware! { |request, mut response|
        response.set(AccessControlAllowOrigin::Any);
        grid_clone.lock().unwrap().set_mut(
            request.param("x").unwrap().parse().unwrap(),
            request.param("y").unwrap().parse().unwrap(),
            true
        );
        "set!"
    });
}

fn handle_get(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.get("/:x/:y", middleware! { |request, mut response|
        response.set(AccessControlAllowOrigin::Any);
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

    server.post("/step", middleware! { |_, mut response|
        response.set(AccessControlAllowOrigin::Any);
        grid_clone.lock().unwrap().step_mut();
        "stepped!"
    });
}

fn handle_randomize(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.post("/randomize", middleware! { |_, mut response|
        response.set(AccessControlAllowOrigin::Any);
        grid_clone.lock().unwrap().randomize();
        "radomized!"
    });
}

fn handle_show(server: &mut Nickel, grid: &mut Arc<Mutex<Grid>>) {
    let grid_clone = grid.clone();

    server.get("/", middleware! { |_, mut response|
        response.set(AccessControlAllowOrigin::Any);
        let grid_slice = json!(grid_clone.lock().unwrap().get_grid());
        format!("{}", grid_slice)
    });
}

fn main() {
    let mut grid = Arc::new(Mutex::new(Grid::new(16, 16)));
    let mut server = Nickel::new();

    handle_clear(&mut server, &mut grid);
    handle_set(&mut server, &mut grid);
    handle_get(&mut server, &mut grid);
    handle_step(&mut server, &mut grid);
    handle_randomize(&mut server, &mut grid);
    handle_show(&mut server, &mut grid);

    server.listen("localhost:4000");
}
