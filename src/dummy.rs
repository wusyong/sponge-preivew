use crate::{Sponge, Trit};

#[derive(Clone, Default, Debug)]
pub struct Dummy {
    state: [Trit; 32],
    round: u8,
}

impl Sponge for Dummy {
    fn absorb(&mut self, input: &[Trit]) {
        for chunk in input.chunks(32) {
            self.state[0..chunk.len()].clone_from_slice(chunk);
            // transform
        }
    }

    fn squeeze(&mut self, output: &mut [Trit]) {
        let trit_count = output.len();
        let hash_count = trit_count / 32;

        for i in 0..hash_count {
            output[i * 32..(i + 1) * 32]
                .clone_from_slice(&self.state[0..32]);
            // transform
        }

        let last = trit_count - hash_count * 32;
        output[trit_count - last..].clone_from_slice(&self.state[0..last]);
        // transform
    }

    fn reset(&mut self) {
        self.state = [0; 32];
    }
} 

#[test]
fn dummy_test() {
    let mut x = Dummy::default();
    let input = [1u8; 32];
    let mut output = [2u8; 32];

    x.absorb(&input);
    x.squeeze(&mut output);
    x.reset();

    assert_eq!(output, [1u8; 32]);
    assert_eq!(x.state, [0u8; 32]);

    let mut output = [0u8; 32];
    x.digest(&input, &mut output);

    assert_eq!(output, [1u8; 32]);
}