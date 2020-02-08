// use std::io::stdin;

fn main() {
    // ok
    let sql = "select * from tbl1";
    println!("sql = {} len = {}", sql, sql.len());
    let sl = sql.len();
    for i in 0 .. sl {
      println!("{}  {}", i, &sql[i..i+1]);
    }
  }