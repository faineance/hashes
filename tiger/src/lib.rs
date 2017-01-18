//! The [Tiger][1] hash function.
//!
//! [1]: https://en.wikipedia.org/wiki/MD2_(cryptography)

#![no_std]
extern crate byte_tools;
extern crate digest;
extern crate digest_buffer;
extern crate generic_array;

pub use digest::Digest;
use byte_tools::copy_memory;
use digest_buffer::DigestBuffer;
use generic_array::GenericArray;
use generic_array::typenum::{U16, U64, U48};

// fn round(state: Tiger, x: , mul: usize) -> Tiger  {
    
// }




fn pass(state: Tiger, mul: usize) -> Tiger {
    round(state)
}
type BlockSize = U64;
type Block = GenericArray<u8, BlockSize>;

#[derive(Copy, Clone)]
struct Tiger {
    a: u64,
    b: u64,
    c: u64,
}

impl Tiger {
    pub fn new() -> Tiger {
        Tiger {
            a: 0x0123456789ABCDEF,
            b: 0xFEDCBA9876543210,
            c: 0xF096A5B4C3B2E187,
        }
    }
}

impl Default for Tiger  {        

    fn default() -> Self { Self::new() }
}

    




// impl Md2State {
//     fn process_block(&mut self, input: &Block) {
//         // Update state
//         for j in 0..16 {
//             self.x[16 + j] = input[j];
//             self.x[32 + j] = self.x[16 + j] ^ self.x[j];
//         }

//         let mut t = 0u8;
//         for j in 0..18u8 {
//             for k in 0..48 {
//                 self.x[k] ^= S[t as usize];
//                 t = self.x[k];
//             }
//             t = t.wrapping_add(j);
//         }

//         // Update checksum
//         let mut l = self.checksum[15];
//         for j in 0..16 {
//             self.checksum[j] ^= S[(input[j] ^ l) as usize];
//             l = self.checksum[j];
//         }
//     }
// }

// impl Md2 {
//     pub fn new() -> Md2 {
//         Default::default()
//     }

//     fn finalize(&mut self) {
//         let self_state = &mut self.state;
//         {
//             // Padding
//             let rem = self.buffer.remaining();
//             let mut buffer_end = self.buffer.next(rem);
//             for idx in 0..rem {
//                 buffer_end[idx] = rem as u8;
//             }
//         }
//         self_state.process_block(self.buffer.full_buffer());
//         let checksum = self_state.checksum;
//         self_state.process_block(&checksum);
//     }
// }

// impl Digest for Md2 {
//     type OutputSize = U16;
//     type BlockSize = BlockSize;

//     fn input(&mut self, input: &[u8]) {
//         let self_state = &mut self.state;
//         self.buffer.input(input, |d: &Block| {
//             self_state.process_block(d);
//         });
//     }

//     fn result(mut self) -> GenericArray<u8, Self::OutputSize> {
//         self.finalize();

//         let mut out = GenericArray::default();
//         copy_memory(&self.state.x[0..16], &mut out);
//         out
//     }
// }
