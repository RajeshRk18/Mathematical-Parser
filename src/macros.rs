/*#[macro_export]
macro_rules! sym_gen {
    ($sym:gen_exp) => {
        Exp::Var(stringify!($sym).to_string())
    };
}*/

/*#[macro_export]
macro_rules! fn_gen {
    ($name:ident,$($arg:gen_exp),*) => {
        Exp::Fun(stringify!($name).to_string(), vec![$($arg),*])
    };
    ($name:ident) => {
        Exp::Fun(stringify!().to_string, vec![])
    };
}*/

/*#[macro_export]
macro_rules! fn_params {
    () => {};
    ($param:ident) => {
        vec![gen_exp!($param)]
    };
}
*/

// Import *only* the `X!` macro.
//#[macro_use(X)] extern crate macs;

/*#[macro_export]
macro_rules! gen_fn_params {
    () => {
        vec![]
    };

    ($name:ident) => {
        vec![gen_exp!($name)]
    };

    //params inside a fn. A recursive macro rule that parses every tokens in tt as params each passed and then appended
    ($name:ident, $($params:tt)*) => {
        {
            let mut inner = vec![gen_exp!($name)];
            inner.append(&mut gen_fn_params!($($params)*));
            inner
        }
    };

    ($name:ident($($params:tt)*)) => {
        vec![gen_exp!($name($($params)*))]
    };

    ($name:ident($($params:tt)*), $($other_params:tt)*) => {
        {
            let mut inner = vec![gen_exp!($name($($params)*))];
            inner.append(&mut gen_fn_params!($($other_params)*));
            inner
        }
    }
}

#[macro_export]
macro_rules! gen_exp {
    () => {};

    ($name:ident) => {
        Exp::Var(stringify!($name).to_string())
    };

    ($($name:ident($($params:tt)*)*)) => {
        $(Exp::Fn(stringify!($name).to_string(), gen_fn_params!($($params)*)))*
    }

    ($name:ident($($params:tt)*)) => {
        Exp::Fn(stringify!($name).to_string(), gen_fn_params!($($params)*))
    };

}


/*#[macro_export]
macro_rules! create_enum {
    ($name:ident, $($variant:ident$($type:ty)*),*) => {
        enum $name {
            $(type_match!($variant$(($type)*))*
        }
    };

    }

*/

#[allow(unused_macros)]
macro_rules! fun_args {
    () => { vec![] };
    ($name:ident) => { vec![gen_exp!($name)] };
    ($name:ident,$($rest:tt)*) => {
        {
            let mut t = vec![gen_exp!($name)];
            t.append(&mut fun_args!($($rest)*));
            t
        }
    };
    ($name:ident($($args:tt)*)) => {
        vec![gen_exp!($name($($args)*))]
    };
    ($name:ident($($args:tt)*),$($rest:tt)*) => {
        {
            let mut t = vec![gen_exp!($name($($args)*))];
            t.append(&mut fun_args!($($rest)*));
            t
        }
    }
}


#[allow(unused_macros)]
macro_rules! gen_exp {
    ($name:ident) => {
        Expr::parse_ident(stringify!($name))
    };
    ($name:ident($($args:tt)*)) => {
        Expr::Fun(Box::new(Expr::parse_ident(stringify!($name))), fun_args!($($args)*))
    };
}*/

#[macro_export]
macro_rules! gen_fn_params {
    () => { vec![] };
    ($name:ident) => { vec![gen_exp!($name)] };
    ($name:ident,$($rest:tt)*) => {
        {
            let mut t = vec![gen_exp!($name)];
            t.append(&mut gen_fn_params!($($rest)*));
            t
        }
    };
    
    ($name:ident($($args:tt)*)) => {
        vec![gen_exp!($name($($args)*))]
    };
    ($name:ident($($args:tt)*),$($rest:tt)*) => {
        {
            let mut t = vec![gen_exp!($name($($args)*))];
            t.append(&mut gen_fn_params!($($rest)*));
            t
        }
    }
}

#[macro_export]
macro_rules! gen_exp {
    ($name:ident) => {
        Exp::Var(stringify!($name).to_string())
    };
    ($name:ident($($args:tt)*)) => {
        Exp::Fn(stringify!($name).to_string(), gen_fn_params!($($args)*))
    };
}



