use rand::Rng;
struct Ant{
    x:i32,
    y:i32,
    direction:Direction,
}

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

struct Grid{
    width:usize,
    height:usize,
    cells:Vec<Vec<bool>>,
}


impl Grid{
    fn new(width:usize, height:usize) ->Grid{
        let cells = vec![vec![false;height];width];
        Grid{
            width,
            height,
            cells,
        }
    }
    fn replace_cell(&mut self,x:i32,y:i32){
        if x>=0 && x<self.width as i32 && y>=0 && y <self.height as i32{
            self.cells[x as usize][y as usize]=!self.cells[x as usize][y as usize];
        }
    }
}

impl Ant{
    fn new(x:i32,y:i32)->Ant{
        Ant{
            x,
            y,
            direction:Direction::Up,
        }
    }
    fn turn_left(&mut self){
        self.direction = match self.direction{
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right(&mut self){
        self.direction = match self.direction{
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    fn forward(&mut self,grid:&mut Grid){
        
        match self.direction{
            Direction::Up => {
                self.y = (self.y + grid.height as i32 - 1) % grid.height as i32;

            }

            Direction::Down => {
                if self.y+1>grid.height as i32-1{
                    self.y =self.y-(grid.height as i32-1);
                }
                else {
                    self.y +=1;
                }
            }

            Direction::Left => {
                if self.x-1<0{
                    self.x=self.x+(grid.width as i32-1);
                }
                else {
                    self.x -=1;
                }
            }
            
            Direction::Right => {
                if self.x+1>(grid.width as i32-1){
                    self.x=self.x-(grid.width as i32-1)
                }
                else {
                    self.x +=1;
                }
            }
        }
        
    }
    fn process_step(&mut self,grid:&mut Grid){
        let cell =grid.cells[self.x as usize][self.y as usize];
        if cell{
            self.turn_right();
        }else{
            self.turn_left();
        }
        grid.replace_cell(self.x,self.y);
        self.forward(grid);
        
    }
}
fn main() {
    let width=100;
    let height=100;
    let mut grid=Grid::new(width,height);

    let mut ants= Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..5{
        let x = rng.gen_range(0..width as i32);
        let y = rng.gen_range(0..height as i32);
        ants.push(Ant::new(x,y));
    }
    for i in 1..=10{
        println!("{}",i);
        for ant in ants.iter_mut(){
            ant.process_step(&mut grid);
        }
        for y in 0..height{
            for x in 0..width{
                if grid.cells[x][y]{
                    print!("{}",'X');
                }
                else{
                    print!("{}",'-');
                }
            }
            println!();
        }
    }
    
}
