use crate::expression::*;

impl std::fmt::Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fn(name, params) => {
                write!(f, "{}(", name)?;
                if params.len() == 0 {
                    write!(f, ")")?;
                }
                for (i, arg) in params.iter().enumerate() {
                    if i < params.len() - 1 {
                        write!(f, "{},", arg)?;
                    } else {
                        write!(f, "{})", arg)?;
                    }
                }
                Ok(())
            },

            Self::Var(var) => {
                write!(f, "{} ", var)
            },

            Self::Op(op) => {
                write!(f, "{} ", op)
            },

            Self::Int(int) => {
                write!(f, "{} ", int)
            },

            Self::Cap(cap) => {
                write!(f, "{}", cap)
            },

            Self::Exponent(int) => {
                write!(f, "{} ", int)
            }

            Self::Nil => {write!(f, "")}
        }
    }
}
