// The table S given below is a permutation of 0...255 constructed
// from the digits of pi.  It is a ``random'' nonlinear byte
// substitution operation.
static S: [u32; 256] = [
    41, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6, 19,
    98, 167, 5, 243, 192, 199, 115, 140, 152, 147, 43, 217, 188, 76, 130, 202,
    30, 155, 87, 60, 253, 212, 224, 22, 103, 66, 111, 24, 138, 23, 229, 18,
    190, 78, 196, 214, 218, 158, 222, 73, 160, 251, 245, 142, 187, 47, 238, 122,
    169, 104, 121, 145, 21, 178, 7, 63, 148, 194, 16, 137, 11, 34, 95, 33,
    128, 127, 93, 154, 90, 144, 50, 39, 53, 62, 204, 231, 191, 247, 151, 3,
    255, 25, 48, 179, 72, 165, 181, 209, 215, 94, 146, 42, 172, 86, 170, 198,
    79, 184, 56, 210, 150, 164, 125, 182, 118, 252, 107, 226, 156, 116, 4, 241,
    69, 157, 112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230, 45, 168, 2,
    27, 96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15,
    85, 71, 163, 35, 221, 81, 175, 58, 195, 92, 249, 206, 186, 197, 234, 38,
    44, 83, 13, 110, 133, 40, 132, 9, 211, 223, 205, 244, 65, 129, 77, 82,
    106, 220, 55, 200, 108, 193, 171, 250, 36, 225, 123, 8, 12, 189, 177, 74,
    120, 136, 149, 139, 227, 99, 232, 109, 233, 203, 213, 254, 59, 0, 29, 57,
    242, 239, 183, 14, 102, 88, 208, 228, 166, 119, 114, 248, 235, 117, 75, 10,
    49, 68, 80, 180, 143, 237, 31, 26, 219, 153, 141, 51, 159, 17, 131, 20,
];

/// An MDCTX structure is a context buffer for a message digest
/// computation; it holds the current "state" of a message digest
/// computation
pub struct MDContext {
    pub D: Vec<u32>,     // buffer to store digest
    pub C: Vec<u32>,     // checksum register
    pub i: u32,          // number of bytes handled, modulo 16
    pub L: u32,          // last checksum char saved
}

impl MDContext {
    pub fn new() -> MDContext {
        let mut D = vec![0; 48];//: [u8; 48] = [0];
        let mut C = vec![0; 16];//: [u8; 16] = [0];
        let i: u32 = 0;
        let L: u32 = 0;

        for i in 0..16 {
            D[i] = 0;
            C[i] = 0;
        }

        MDContext { D, C, i, L }
    }

    /// The routine MDUPDATE updates the message digest context buffer to
    /// account for the presence of the character c in the message whose
    /// digest is being computed.  This routine will be called for each
    /// message byte in turn.
    pub fn update(&mut self, c: u32) {
        // add new byte to the buffer
        let mut i = self.i as usize;
        self.D[16+i] = c;
        self.D[32+i] = c ^ self.D[i];

        // update checksum and L
        self.C[i] ^= S[0xff & ((c as usize) ^ (self.L as usize))];
        self.L = self.C[i];

        // inc i by 1 modulo 16
        i = (i + 1) & 15;
        self.i = i as u32;

        // transform D if i=0
        if i == 0 {
            let mut t = 0;
            for j in 0..18 {
                for k in 0..48 {
                    self.D[k] = self.D[k] ^ S[t];
                    t = self.D[k] as usize;
                }
                t = t + j;
            }
        }
    }

    /// The routine MDFINAL terminates the message digest computation and
    /// ends with the desired message digest being in md->D[0...15].
    pub fn finalize(&mut self) {
        let padlen = 16 - self.i;
        for _i in 0..padlen {
            let x: u32 = padlen;
            self.update(x);
        }

        // extend with checksum
        for i in 0..16 {
            let x: u32 = self.C[i];
            self.update(x);
        }
    }

    pub fn display(&self) {
        println!("Hello, world!");
        println!("Checksum: {:?}", self.C);
        println!("Digest buffer: {:?}", self.D);
        println!("Last checksum char saved {:?}", self.L);
        println!("n-bytes, modulo 16 {:?}", self.i);
        println!("");
    }

}
