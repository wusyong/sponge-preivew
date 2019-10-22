#![no_std]

pub type Trit = i8;

pub trait Sponge
where
    Self: Default + Clone + Send + 'static,
{
    /// Absorb trits into the sponge
    fn absorb<T: AsRef<[Trit]>>(&mut self, trits: T);
    /// Squeeze trits out of the sponge and copy them into `out`
    fn squeeze<T: AsMut<[Trit]>>(&mut self, out: T);
    /// Reset the sponge to initial state
    fn reset(&mut self);
}
