struct OceanFloor {
    map: Vec<Vec<i32>>, //map of the ocean floor
    pos: (i32, i32),    //current position for iteration
}
#[derive(Debug)]
struct LowSpot {
    center: i32,
    x: i32,
    y: i32,
}
impl LowSpot {
    fn new(center: i32, x: i32, y: i32) -> Self {
        LowSpot {
            center: center,
            x: x,
            y: y,
        }
    }
    fn get_risk_level(&self) -> i32 {
        self.center + 1
    }
}
impl OceanFloor {
    fn new(path: &str) -> Self {
        let map = std::fs::read_to_string(path)
            .unwrap()
            .as_str()
            .trim()
            .to_string()
            .split("\n")
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .iter()
            .map(|row| {
                row.as_str()
                    .trim()
                    .split("")
                    .collect::<String>()
                    .to_string()
                    .chars()
                    .map(|c| c.to_string().as_str().trim().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        OceanFloor {
            map: map,
            pos: (0, 0),
        }
    }
}
impl Iterator for OceanFloor {
    type Item = LowSpot;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let x_start = self.pos.0 as usize;
        let y_start = self.pos.1 as usize;
        let x_end = self.map.len() as i32 as usize;
        let y_end = (self.map.clone().iter().nth(0).clone().unwrap().len()) as usize;

        for y_index in 0..y_end {
            for x_index in 0..x_end {
                if y_index < y_start {
                    continue;
                }
                if y_index == y_start && (x_index as usize) < x_start {
                    continue;
                }

                let center = self
                    .map
                    .iter()
                    .nth(y_index)
                    .unwrap()
                    .iter()
                    .nth(x_index)
                    .unwrap();
                let top = match y_index {
                    idx if idx == 0 => None,
                    idx => Some(
                        self.map
                            .iter()
                            .nth((idx as usize) - 1)
                            .unwrap()
                            .iter()
                            .nth(x_index as usize)
                            .unwrap(),
                    ),
                };
                let right = match x_index {
                    idx if idx == x_end - 1 => None,
                    idx => Some(
                        self.map
                            .iter()
                            .nth(y_index as usize)
                            .unwrap()
                            .iter()
                            .nth((idx as usize) + 1)
                            .unwrap(),
                    ),
                };
                let left = match x_index {
                    idx if idx == 0 => None,
                    idx => Some(
                        self.map
                            .iter()
                            .nth(y_index as usize)
                            .unwrap()
                            .iter()
                            .nth((idx as usize) - 1)
                            .unwrap(),
                    ),
                };

                let below = match y_index {
                    idx if idx == y_end - 1 => None,
                    idx => Some(
                        self.map
                            .iter()
                            .nth((idx as usize) + 1)
                            .unwrap()
                            .iter()
                            .nth(x_index as usize)
                            .unwrap(),
                    ),
                };

                if top.unwrap_or(&999) > center
                    && left.unwrap_or(&999) > center
                    && right.unwrap_or(&999) > center
                    && below.unwrap_or(&999) > center
                {
                    //if x_index == 90 && y_index == 99 {
                    //    panic!("value {}, x: {}, y: {}",center, x_index,y_index);
                    //}
                    if x_index + 1 > x_end {
                        if y_index + 1 > y_end {
                            self.pos = (x_end as i32, y_end as i32);
                        } else {
                            self.pos = (0, y_index as i32 + 1);
                        }
                    } else {
                        self.pos = (x_index as i32 + 1, y_index as i32);
                    }
                    let low_spot = LowSpot::new(
                        *center,
                        x_index as i32,
                        y_index as i32,
                    );
                    return Some(low_spot);
                }
            }
        }
        None
    }
}

fn floobill(map:Vec<Vec<i32>>,x:i32,y:i32,prevVal:i32,visited: std::collections::HashMap<(usize,usize),i32>)->i32{
   if x as usize > map.iter().nth(0).unwrap().len()-1 || y as usize > map.len()-1 {
        return 0;
    }
    let current_value = map.iter().nth(y as usize).unwrap().iter().nth(x as usize).unwrap();
    
    if current_value < &prevVal {
        return 0;
    }
    if  current_value == &9 {
        return 0;
    }
    if visited.contains_key(&(x as usize,y as usize)) {
        return 0;
    } 
    let mut visited = visited;
    visited.insert((x as usize,y as usize),1);
    
    let right = 1 + floobill(/*&mut*/ map.clone(),x+1,y,*current_value,visited.clone());
    let left = 1+  floobill(/*&mut*/ map.clone(),x-1,y,*current_value,visited.clone());
    let top = 1+ floobill(/*&mut*/ map.clone(),x,y+1,*current_value,visited.clone());
    let bottom = 1+floobill(/*&mut*/ map.clone(),x,y-1,*current_value,visited.clone());
    (right + left + top + bottom)
}

fn main() {
    let of = OceanFloor::new("/home/vancha/Documenten/rust/aoc_9a/input");
    let  map = of.map.clone();
    let mut basins = vec![];
    for low in of {
        let mut muu = map.clone();
        let basin_size = floobill(muu,low.x,low.y,low.center,std::collections::HashMap::new());
        if low.x == 90 && low.y == 99 {
            println!("{},{},{},size: {}",low.x, low.y, low.center, basin_size);
        }
        basins.push(basin_size);//add one for lowest point size
    }
    basins.sort();
    let bigst = basins.iter().skip(basins.len() -3).collect::<Vec<&i32>>();
    //2824 is too low..
    //14127 is too low..
    println!("basins: {:?}",bigst.iter().map(|v|*v).sum::<i32>());
    //let result = of.map(|t| t.get_risk_level()).sum::<i32>();
    //println!("sum is: {}", result);
}
