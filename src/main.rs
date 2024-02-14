// TODO: compatibility with narray?

// TODO: SIMD support, somehow?

// TODO: support iterators in addition to slices? maybe dasp_signal::Signal?

// TODO: support both dynamic and const size for blocks/slices?

// TODO: better name? View, Slice, Span, ...?
#[derive(Copy, Clone)]
struct Slice<'a> {
    // This doesn't really have to be a slice for now (could be any IntoIterator),
    // but we might want to implement (multi-channel) indexing/slicing at some point,
    // which will not work efficiently with interators.
    data: &'a [f32],
}

impl<'a> IntoIterator for Slice<'a> {
    type Item = &'a f32;
    type IntoIter = <&'a [f32] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.data.iter()
    }
}

struct MutSlice<'a> {
    data: &'a mut [f32],
}

// VS: vector, scalar
struct MulVS<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> std::ops::Deref for MulVS<L, R> {
    type Target = Expr<MulVS<L, R>>;
    fn deref(&self) -> Self::Target {
        Expr(MulVS { lhs: self.lhs, rhs: self.rhs})
    }
}

impl<'a> std::ops::Mul<f32> for Slice<'a> {
    type Output = MulVS<Slice<'a>, f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            lhs: self,
            rhs,
        }
    }
}

// TODO: Copy, Clone? pub?
struct Expr<T>(T);

struct AddOp<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> std::ops::Add<Expr<R>> for Expr<L> {
    type Output = Expr<AddOp<Expr<L>, Expr<R>>>;
    fn add(self, rhs: Expr<R>) -> Self::Output {
        Expr(AddOp{ lhs: self, rhs})
    }
}

impl<'r, L, R> std::ops::AddAssign<MulVS<L, R>> for MutSlice<'_>
where
    R: 'r + Copy,
    L: IntoIterator<Item = &'r R>,
    R: std::ops::Mul<R>,
    f32: std::ops::AddAssign<R::Output>,
{
    fn add_assign(&mut self, rhs: MulVS<L, R>) {
        // TODO: assert same length (via size hint?)

        for (src, target) in rhs.lhs.into_iter().zip(self.data.iter_mut()) {
            *target += *src * rhs.rhs;
        }
    }
}

fn main() {
    //c << a * 3_f32 + b * 2_f32;
    //let intermediate = a * 3_f32 + b * 2_f32;
    //c << intermediate;

    // TODO: is it possible to use target slice as source?
    //c << a * 3_f32 + c * 2_f32;

    //let init = uninit << foo * bar;

    //a << 2.5; // fill (memset?)

    //a *= 4.0;

    // If a and b are slices, this is less efficient than copy_from_slice():
    //a << b;

    let mut target = [0., 0., 0., 0.];

    let mut d = MutSlice { data: &mut target };

    {
        let vec = vec![1., 2., 3., 4.];
        let e = Slice { data: &vec };

        d += e * 2.0;
        dbg!(&d.data);
        //dbg!(target);
        d += e * 10.0;
        // NB: target can live longer than the source slice
    }
    dbg!(target);
    let a = Slice { data: &[0.1, 0.2, 0.3] };
    let b = Slice { data: &[0.4, 0.5, 0.6] };
    let mut g = MutSlice { data: &mut target };
    g += a * 100_f32 + b * 10_f32;
    dbg!(target);
}
