// A company has n employees with a unique ID for each employee from 0 to n - 1. The head of the company is the one with headID.

// Each employee has one direct manager given in the manager array where manager[i] is the direct manager of the i-th employee, manager[headID] = -1. Also, it is guaranteed that the subordination relationships have a tree structure.

// The head of the company wants to inform all the company employees of an urgent piece of news. He will inform his direct subordinates, and they will inform their subordinates, and so on until all employees know about the urgent news.

// The i-th employee needs informTime[i] minutes to inform all of his direct subordinates (i.e., After informTime[i] minutes, all his direct subordinates can start spreading the news).

// Return the number of minutes needed to inform all the employees about the urgent news.


pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut graph = vec![vec![]; n as usize];

    for (employee, &manager) in manager.iter().enumerate() {
        if manager != -1 {
            graph[manager as usize].push(employee);
        }
    }

    let mut max = 0;
    let mut stack = vec![(head_id as usize, 0)];

    while let Some((node, time)) = stack.pop() {
        max = max.max(time);

        for &child in &graph[node] {
            stack.push((child, time + inform_time[node]));
        }
    }

    max
}

fn main() {
    let n = 6;
    let head_id = 2;
    let m = vec![2,2,-1,2,2,2];
    let info = vec![0,0,1,0,0,0];

    let answer = num_of_minutes(n, head_id, m, info);
    
    println!("{}", answer);
}
