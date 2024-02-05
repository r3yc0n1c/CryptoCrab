use tqdm::tqdm;

fn vec2str(v: Vec<u8>) -> String{
    let flag = String::from_utf8(v).expect("Found invalid UTF-8");
    return flag;
}

fn main() {
    let arr = vec![99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125];
    let mut flag = vec2str(arr);
    println!("Flag: {}", flag);

    let h = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let dec = hex::decode(h).expect("Dec Failed!");
    flag = vec2str(dec);
    println!("Flag: {}", flag);

    let n:i64 = 2<<30;
    for i in tqdm(1..n){
        if i==2<<29 {
            eprintln!("i = {:?}", i);
            break;            
        }
    }
}
