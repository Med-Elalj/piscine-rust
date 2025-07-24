use scalar::*;

fn main() {
    // sum
    
    println!("sum: {:?}", sum(234, 2));   // Ok(236)
    println!("sum: {:?}", sum(1, 255));   // Err("ERROR: attempt to add with overflow")

    // diff
    println!("diff: {:?}", diff(234, 2));       // Ok(232)
    println!("diff: {:?}", diff(-32768, 32766)); // Err("ERROR: attempt to subtract with overflow")

    // pro
    println!("pro: {:?}", pro(23, 2));    // Ok(46)
    println!("pro: {:?}", pro(-128, 2));  // Err("ERROR: attempt to multiply with overflow")

    // quo
    println!("quo: {}", quo(22.0, 2.0));         // 11.0
    println!("quo: {}", quo(-128.23, 2.0));      // -64.115

    // rem
    println!("rem: {}", rem(22.0, 2.0));         // 0.0
    println!("rem: {}", rem(-128.23, 2.0));      // -0.22999573
}
