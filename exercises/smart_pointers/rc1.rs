// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.



use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

// summary 
// 不知道什么特性 使用clone会+1
// 使用drop(name);会让计数器-1  切换顺序不影响

// 这里的计数器是由 Rc（引用计数）类型来维护的，它会自动增加和减少。
// 当你调用 Rc::clone() 函数来创建 Rc 的克隆时，它会增加引用计数。
// 当 Rc 类型超出作用域或者被显式 drop 掉时，引用计数会减少。
// 引用计数会跟踪 Rc 实例的数量，当计数为零时，它会自动释放分配的内存，确保不会造成内存泄漏。
fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    // 注意 后面都是new了一个Sun
    // TODO
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    // TODO
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    // TODO
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    drop(earth);
    // TODO
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    
    drop(mars);
    // TODO
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    drop(mercury);
    // TODO
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
