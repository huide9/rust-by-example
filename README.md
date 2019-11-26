# rust-by-example
rust learning practice

# 变量绑定与原生类型

* Rust 通过 let 关键字进行变量绑定
* assert_eq! 宏的作用是判断两个参数是不是相等的，但如果是两个不匹配的类型，就算字面值相等也会报错。
* mut 关键字，变量就会成为可变绑定的变量
##原生类型
* Rust内置的原生类型 (primitive types) 有以下几类：
* 布尔类型：有两个值true和false。
* 字符类型：表示单个Unicode字符，存储为4个字节。
* 数值类型：分为有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 以及浮点数 (f32, f64)。
* 字符串类型：最底层的是不定长类型str，更常用的是字符串切片&str和堆分配字符串String， 其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
* 数组：具有固定大小，并且元素都是同种类型，可表示为[T; N]。
```
  // 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏vec!创建
  // 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的
  let arr = [0, 1, 2, 3, 4];             // 
  let middle = &arr[0..3];               // get from 0 to 3 
  let  ten_zeros : [i64; 10] = [2; 10];  // 
```
* 切片：引用一个数组的部分数据并且不需要拷贝，可表示为&[T]。
* 元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
  ```
  let tuple: (i32, &str(50, "hello");
  let h = tuple.1;
  ```
* 指针：最底层的是裸指针*const T和*mut T，但解引用它们是不安全的，必须放到unsafe块里。
```
  let x = 5;
  // Rust不提供原生类型之间的隐式转换，只能使用as关键字显式转换。
  let raw = &x as *const i32;
  let points_at = unsafe{ *raw };
  println!("x ->{:?}, raw->{:?}, *raw->{:?}, points_at->{:?}", x, raw, unsafe{*raw}, points_at);
```
* 函数：具有函数类型的变量实质上是一个函数指针。
* 元类型：即()，其唯一的值也是()。

需要特别注意的：
* 数值类型可以使用_分隔符来增加可读性。
* Rust还支持单字节字符b'H'以及单字节字符串b"Hello"，仅限制于ASCII字符。 此外，还可以使用r#"..."#标记来表示原始字符串，不需要对特殊字符进行转义。
* 使用&符号将String类型转换成&str类型很廉价， 但是使用to_string()方法将&str转换到String类型涉及到分配内存， 除非很有必要否则不要这么做。
* 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏vec!创建。
* 元组可以使用==和!=运算符来判断是否相同。
* 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的。
* Rust不提供原生类型之间的隐式转换，只能使用as关键字显式转换。
* 可以使用type关键字定义某个类型的别名，并且应该采用驼峰命名法。
```
// explicit conversion
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;

// type aliases
type NanoSecond = u64;
type Point = (u8, u8);
```
