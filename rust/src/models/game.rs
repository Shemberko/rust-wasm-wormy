struct Game {
    pub map: Map,
    pub players: Vec<Player>,
    pub objects: Vec<CanvasObject>
}

impl Game {
    pub run(&mut self) {
        self.update();
        self.draw();
    }

    pub fn change_state(&mut self, new_state: GameState) {
        self.state = new_state;
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
    }

    pub fn update(&mut self, delta_time: f64) {
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
    }
}