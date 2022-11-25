use std::ops::*;

#[derive(Debug)]
pub struct W<A>(Vec<V<A>>);

#[derive(Debug)]
pub enum V<A> {
    Val(A),
    Op(Op),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

impl<A> W<A> {
    pub fn calc(&self) -> A
    where
        A: Add<A, Output = A>
            + Mul<A, Output = A>
            + Sub<A, Output = A>
            + Div<A, Output = A>
            + Clone,
    {
        let len = self.0.len();
        let V::Val(i) = self.0.last().unwrap() else {
            panic!()
        };
        if len == 1 {
            return i.clone();
        }
        (self.0[..len - 1])
            .chunks(2)
            .into_iter()
            .rev()
            .fold(i.clone(), |acc, v| match v {
                [V::Val(v), V::Op(op)] => match op {
                    Op::Add => v.clone() + acc,
                    Op::Mul => v.clone() * acc,
                    Op::Sub => v.clone() - acc,
                    Op::Div => v.clone() / acc,
                },
                _ => unreachable!(),
            })
    }
}

impl<A> Add for W<A> {
    type Output = W<A>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.push(V::Op(Op::Add));
        self.0.extend(rhs.0);
        self
    }
}
impl<A> Mul for W<A> {
    type Output = W<A>;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.0.push(V::Op(Op::Mul));
        self.0.extend(rhs.0);
        self
    }
}
impl<A> Sub for W<A> {
    type Output = W<A>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0.push(V::Op(Op::Sub));
        self.0.extend(rhs.0);
        self
    }
}
impl<A> Div for W<A> {
    type Output = W<A>;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.0.push(V::Op(Op::Div));
        self.0.extend(rhs.0);
        self
    }
}

pub fn w<A>(a: A) -> W<A> {
    W(vec![V::Val(a)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = w(2) / w(8) - w(2) * w(1) + w(2);

        assert_eq!(a.calc(), 1);
    }
}
