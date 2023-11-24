fn main() {
    //boolean
    let b1 = true;

    //unsigned integers
    let i1:u8 = 1;
    let i2:u16 = 1;
    let i3:u32=1;
    let i4:u64=1;
    let i5:u128=1;
    //signed integers
    let u1:u8=1;
    let u2:u16=1;
    let u3:u32=1;
    let u4:u64=1;
    let u5:u128=1;
    //floating point numbers
    let f1:f32=1.0;
    let f2:f64=1.0;
    //platform specific integers
    let p1:usize = 1;
    let p2:usize = 1;
    //character, &str, and String
    let c1:char ='c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    //arrays
    let a1 = [1, 2, 3, 4, 5];
    let i1 = a1[4];

    //tuples
    let t1=(1, 2, 3);
    let t1=(5, 5.0,"5");

    let s1=t1.2;
    let (i1, f1, s1)=t1;
    let unit=();
    //type aliasing
    type age = u8;
    let a1:age=57;
}
