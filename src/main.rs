#![allow(dead_code)]

// функция double_int32 принимает 32-х битное целое беззнаковое число и возвращает 32-х битное целое беззнаковое число, равное удвоенному входному.
fn double_int32(a: u32) -> u32 {
    a.wrapping_mul(2_u32)
}

// функция double_int64 принимает 32-х битное целое беззнаковое число и возвращает 64-х битное целое беззнаковое число, равное удвоенному входному.
fn double_int64(a: u32) -> u64 {
    a as u64 * 2
}

// функция double_float32 принимает 32-х битное число с плавающей точкой и возвращает 32-х битное число с плавающей точкой, равное удвоенному входному.
fn double_float32(a: f32) -> f32 {
    a * 2_f32
}

// функция double_float64 принимает 32-х битное число с плавающей точкой и возвращает 64-х битное число с плавающей точкой, равное удвоенному входному.
fn double_float64(a: f32) -> f64 {
    a as f64 * 2_f64
}

// функция int_plus_float_to_float принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой.
// Возвращает 64-х битное число с плавающей точкой, равное сумме входных.
fn int_plus_float_to_float(a: u32, b: f32) -> f64 {
    (a as f64) + (b as f64)
}

// функция tuple_sum принимает кортеж из двух целых чисел. Возвращает целое число, равное сумме чисел во входном кортеже.
fn tuple_sum(tuple: (i64, i64)) -> i64 {
    let (a, b) = tuple;
    a.wrapping_add(b)
}

// функция array_sum принимает массив из трёх целых чисел. Возвращает целое число, равное сумме ччисел во входном массиве.
fn array_sum(array: [i64; 3]) -> i64 {
    array[0].wrapping_add(array[1]).wrapping_add(array[2])
}

// функция int_plus_float_to_int принимает 32-х битное целое беззнаковое число и 32-х битное число с плавающей точкой.
// Возвращает 64-х битное целое беззнаковое число, равное сумме входных.
fn int_plus_float_to_int(a: u32, b: f32) -> u64 {
    let first = a as f64;
    let second = b.trunc() as f64;
    let sum = first + second;
    sum.abs() as u64
}

fn main() {
    // println!("{}", array_sum([1, 2, 3]));
    // println!("{}", double_int32(u32::MAX));
    // println!("{}", double_int64(u32::MAX));
    // println!("{}", double_float32(f32::MAX));
    // println!("{}", double_float64(f32::MAX));
    // println!("{}", int_plus_float_to_float(1_u32, 1.2_f32));
    // println!("{}", tuple_sum((i64::MAX, 2)));
    // println!("{}", array_sum([i64::MAX, i64::MAX, i64::MAX]));

    println!("{}", int_plus_float_to_int(10_u32, -5_f32));
    println!("{}", int_plus_float_to_int(10_u32, -1.5_f32));
    println!("{}", int_plus_float_to_int(1_u32, -2_f32));
    println!("{}", int_plus_float_to_int(1_u32, -5.7_f32)); 

}