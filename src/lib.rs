#![no_std]

pub trait Sponge
where
    Self: Default + Clone,
{
    type Item;

    /// Absorb trits into the sponge
    fn absorb(&mut self, input: &[Self::Item]);
    
    /// Squeeze trits out of the sponge and copy them into `out`
    fn squeeze(&mut self, output: &mut [Self::Item]);

    /// Reset the sponge to initial state
    fn reset(&mut self);

    /// Digest inputs and then compute the hash with length of provided output slice
    fn digest (&mut self, input: &[Self::Item], output: &mut [Self::Item]) {
        self.absorb(input);
        self.squeeze(output);
    }
}
