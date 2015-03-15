#![feature(core)]
#![allow(non_camel_case_types)]

#[macro_export]
macro_rules! strenum {
    ($e:ident => $( $t:ident ),*  ) => {
		use std::num::FromPrimitive;
        #[derive(FromPrimitive, Debug)]
        enum $e { $($t,)* }
        impl Strenum for $e {
            fn enumify(command: &str) -> Option<$e> {
                match command {
                    $( stringify!($t) => Some($e::$t), )*
                    _ => None
                }
            }
            
            fn stringify() -> String {
                let mut buf = Vec::<String>::new();
                let mut n: u8 = 0;
                
                while let Some(val) = <Self>::from_u8(n) {
                    buf.push(format!("{:?}", val));
                    n += 1;
                }
                
                buf.connect("\n")
            }
        }
    }
}

pub trait Strenum {
    fn enumify(command: &str) -> Option<Self>;
    fn stringify() -> String;
}


strenum! {
    testenum => a, b
}

#[test]
fn stringify() {
	assert_eq!(&*<testenum>::stringify(), "a\nb");
}

#[test]
fn enumify() {
	match <testenum>::enumify("b").unwrap() {
		testenum::b => "success",
		testenum::a => panic!("Got a instead of b"),
	};
}

