use std::ops::{Mul, AddAssign};

#[derive(Copy, Clone)]
struct Slice<'a> {
    data: &'a [f32],
}

struct MutSlice<'a> {
    data: &'a mut [f32],
}

// TODO: generalize
struct LazySlice<'a> {
    lhs: Slice<'a>,
    rhs: f32,
}

impl<'a> Mul<f32> for Slice<'a> {
    type Output = LazySlice<'a>;

    fn mul(self, rhs: f32) -> Self::Output {
        LazySlice { lhs: self, rhs }
    }
}

// TODO: LazySlice must outlive MutSlice?
impl<'a, 'b> AddAssign<LazySlice<'a>> for MutSlice<'b> {
    fn add_assign(&mut self, rhs: LazySlice<'a>) {
        // TODO: assert same length

        for (src, target) in rhs.lhs.data.iter().zip(self.data.iter_mut()) {
            *target += src * rhs.rhs;
        }
    }
}

fn main() {
    //c += a * 3_f32 + b * 2_f32;
    //c << a * 3_f32 + b * 2_f32;
    //let intermediate = a * 3_f32 + b * 2_f32;
    //c << intermediate;

    //let init = uninit << foo * bar;

    //a *= 4.0;

    let array = [1., 2., 3., 4.];
    // TODO: allow uninitialized data?
    let mut target = [0., 0., 0., 0.];

    let e = Slice { data: &array };
    let mut d = MutSlice { data: &mut target };

    d += e * 2.0;
    //dbg!(target);
    d += e * 10.0;
    dbg!(target);
}
