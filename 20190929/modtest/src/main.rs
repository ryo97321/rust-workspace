pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_module() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_module();
}