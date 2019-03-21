struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::drop::CustomSmartPointer;

    #[test]
    fn a() {
        let c = CustomSmartPointer { data: String::from("my stuff") };
        let d = CustomSmartPointer { data: String::from("other stuff") };
        // Dropping a Value Early with std::mem::drop  releases lock
        drop(c);
        println!("CustomSmartPointers created.");
    }
}