#![no_std]

pub mod dummy;

pub type Trit = u8;

pub trait Sponge
where
    Self: Default + Clone + Send + 'static,
{
    /// Absorb trits into the sponge
    fn absorb(&mut self, input: &[Trit]);
    /// Squeeze trits out of the sponge and copy them into `out`
    fn squeeze(&mut self, output: &mut [Trit]);
    /// Reset the sponge to initial state
    fn reset(&mut self);
    /// Digest inputs and then compute the hash with length of provided output slice
    fn digest (&mut self, input: &[Trit], output: &mut [Trit]) {
        self.absorb(input);
        self.squeeze(output);
    }
}
