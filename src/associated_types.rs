
#[derive(Debug)]
struct Foo<T>(T);

// Parametric types are bound during method calls
trait Bar<T> {
    // Associated types are bound during Trait implementation
    type AT;

    fn associated_type(&self) -> Self::AT {
        todo!()
    }

    fn parametric_type(&self) -> T {
        todo!()
    }
}

impl Bar<String> for Foo<String> {
    type AT = bool;

    fn associated_type(&self) -> Self::AT {
        std::todo!()
    }

    fn parametric_type(&self) -> String {
        std::todo!()
    }
}

impl Bar<i32> for Foo<i32> {
    type AT = &'static str;

    fn associated_type(&self) -> Self::AT {
        std::todo!()
    }

    fn parametric_type(&self) -> i32 {
        std::todo!()
    }
}


