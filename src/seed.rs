#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seed {
    state: u64,
}

impl Seed {
    pub fn from_label(label: &str) -> Self {
        let mut state = 0xcbf2_9ce4_8422_2325u64;
        for byte in label.bytes() {
            state ^= u64::from(byte);
            state = state.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self { state }
    }

    pub fn from_u64(value: u64) -> Self {
        Self { state: value }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.state >> 32) as u32
    }

    pub fn next_bounded(&mut self, upper_exclusive: u32) -> u32 {
        if upper_exclusive == 0 {
            return 0;
        }
        let threshold = upper_exclusive.wrapping_neg() % upper_exclusive;
        loop {
            let value = self.next_u32();
            if value >= threshold {
                return value % upper_exclusive;
            }
        }
    }

    pub fn choose_index(&mut self, len: usize) -> Option<usize> {
        (len > 0).then(|| self.next_bounded(len as u32) as usize)
    }
}
