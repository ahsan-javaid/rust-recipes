use tokio::fs::File;


#[tokio::main]
async fn main() -> Result<()> {
    let files: Vec<_> = (0..2).map(|i| tokio::fs::read_to_string(format!("{}", i))).collect();
    
    // parallel read file example
    let (file1, file2, file3) = join!(files[0], files[1], files[2]);

    // serial execution
    let file1 = files[0].await;
    let file2 = files[1].await;
    let file3 = files[2].await;
}
