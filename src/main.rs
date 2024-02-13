use std::ops::{AddAssign, Mul};

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

impl<'a> AddAssign<LazySlice<'a>> for MutSlice<'_> {
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
    let mut target = [0., 0., 0., 0.];

    let mut d = MutSlice { data: &mut target };

    {
        let e = Slice { data: &array };

        d += e * 2.0;
        dbg!(&d.data);
        //dbg!(target);
        d += e * 10.0;
        // NB: target can live longer than the source slice
    }
    dbg!(target);
    let mut g = MutSlice { data: &mut target };
    g += Slice { data: &array } * 100.0;
    dbg!(target);
}
