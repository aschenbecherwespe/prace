fn main() {

    println!("Hello, world!");
    let point = Tuple{ x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
    let vector = Tuple{ x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
    println!("{:?}", point);
    println!("{:?}", vector);
}

#[derive(Debug)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }
    fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

#[test]
fn tuple_point() {
    let a = Tuple{ x: 4.3, y: -4.2, z: 3.1, w: 1.0 };
    assert_eq!(4.3, a.x);
    assert_eq!(-4.2, a.y);
    assert_eq!(3.1, a.z);
    assert_eq!(1.0, a.w);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn tuple_vector() {
    let a = Tuple{ x: 4.3, y: -4.2, z: 3.1, w: 0.0 };
    assert_eq!(4.3, a.x);
    assert_eq!(-4.2, a.y);
    assert_eq!(3.1, a.z);
    assert_eq!(0.0, a.w);
    assert!(!a.is_point());
    assert!(a.is_vector());
}
