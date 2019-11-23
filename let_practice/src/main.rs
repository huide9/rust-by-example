/*
rust variable types:

u8  i8
u16 i16
u32 i32
u64 i64
usize
f32 
f64 
bool
Unicode  let c = 'x';  4 bytes
str
[T; N]
&[T]
* const T 但解引用它们是不安全的，必须放到unsafe块里。
* mut T    
tuple ? 元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
()  
*/
fn convert(){
  let decimal = 65.123_456_789;
  let integer  = decimal as u8;
  let character = integer as char;
  println!("dec->{:?}, int->{:?},  cha->{:?}", decimal, integer, character);

  //可以使用type关键字定义某个类型的别名，并且应该采用驼峰命名法。
  // like #define ?
  //type NanoSecond = u64;
  //type Point = (u8, u8);

}
fn callee(){
  let str = "hello world!"; // 用r#"..."#标记来表示原始字符串，不需要对特殊字符进行转义
  let mut string = str.to_string();
  // 使用&符号将String类型转换成&str类型很廉价， 
  // 但是使用to_string()方法将&str转换到String类型涉及到分配内存， 除非很有必要否则不要这么做。
  let tuple: (i32, &str) = (50, "hello");
  // 元组可以使用==和!=运算符来判断是否相同
  let (fifty, _) = tuple;
  let (_, cont) = tuple;
  let hello = tuple.1;

  println!("str->{:?}, string->{:?},  tuple-> {:?}", str, string, tuple);
  println!("fifty->{:?}, cont->{:?}", fifty, cont);
  println!("hello=>{:?}", hello);

	let x = 5; 
  // Rust不提供原生类型之间的隐式转换，只能使用as关键字显式转换。
	let raw = &x as *const i32;
	let points_at = unsafe{ *raw };
	println!("x ->{:?}, raw->{:?}, *raw->{:?}, points_at->{:?}", x, raw, unsafe{*raw}, points_at);

}
fn main() {
  // set variable type
  let a1=5;  // integer by deafult is i32
	let a2:i32 = 5;
	assert_eq!(a1, a2);

  let b1:u32 = 5;
  //assert_eq!(b1, a1);
 
  // mutable variable 
	let mut a:f64 = 1.123021;
  let b = 2.0f32;
	println!("{}", b);

  let b1:f32 = 3.0;
	println!("{}", b1);

	println!("{:?}", a);
	println!("{}", a);
	a = 2.003;
	println!("{:?}", a);
	println!("{}", a);

  // change back to immutable
  let a =  a;  
  //a=3.0; // error

  let (a, mut b):(bool, bool) = (true, false);
  println!("a= {:?},  b={:?}", a, b);
  // a = false ;
  b = true;
  assert_eq!(a, b);
  let mut a = a;
  a= false;
  println!("a= {:?},  b={:?}", a, b);

  let c:u32 = 123_456;
  let d:f64 = 1.23e+2;
  println!("c->{:?} d->{:?}", c, d); 	
  let zero = d.abs_sub(123.4);
  let bin = 0b1111_0000;
  let oct = 0o7320_1546;  // 数值类型可以使用_分隔符来增加可读性
  //let hex = 0xf23ab049;
	//let hex = 0xabcd_1234;
  let hex = 0o1234_5671;
  println!("zero->{:?}, bin->{:?}, oct->{:?}, hex->{:?}", zero, bin, oct, hex);

  // 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏vec!创建
  // 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的
  let arr = [0, 1, 2, 3, 4];  
  let middle = &arr[0..4];    
  let  ten_zeros : [i64; 10] = [2; 10];
  println!("arr->{:?}, middle->{:?}, ten_zeros->{:?}", arr, middle, ten_zeros);
  callee();
  convert();
}
