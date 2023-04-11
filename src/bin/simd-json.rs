fn main() {
    // let str = "{\"v1\":64754,\"v2\":9521.824380305317}";
    let str = "9521.824380305317";
    let mut slice = str.as_bytes().to_owned();
    let value: simd_json::BorrowedValue<'_> = simd_json::to_borrowed_value(&mut slice).unwrap();
}
