fn main() {
    unsafe {
        hello();
        let n1 = 1;
        let n2 = 2;
        let s = sum(1, 2);
        print!("经CPP计算: {:?} 加 {:?} 的和为{:?}", n1, n2, s)
    }
    println!("")
}

extern "C" {
    fn hello();
    fn sum(n1: i32, n2: i32) -> i32;
}
