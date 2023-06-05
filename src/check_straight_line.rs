// You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.

// Using slop formula to check if points are collinear
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    if coordinates.len() <= 2 {
        return true;
    }

    let x0 = coordinates[0][0];
    let y0 = coordinates[0][1];

    let x1 = coordinates[1][0];
    let y1 = coordinates[1][1];

    for i in 2..coordinates.len() {
        let x = coordinates[i][0];
        let y = coordinates[i][1];
        
        if (x1 - x0) * (y - y0) != (x - x0) * (y1 - y0) {
            return false;
        }
    }

    true
}

fn main() {
    let coordinates: [[i32; 2]; 6] = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]];
    let mut c_vec: Vec<Vec<i32>> = Vec::new();
    for row in coordinates.iter() {
        let mut temp: Vec<i32> = Vec::new();
        for &v in row.iter() {
            temp.push(v);
        } 
        c_vec.push(temp);
    }

    println!("{:?}", c_vec);

    let coordinatesss = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7]
    ];
    let r = check_straight_line(coordinatesss);
    println!("{}", r);
}
