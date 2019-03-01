pub trait WriteOnceBitField {
    fn set(&mut self, index: usize) -> Result<(), ()>;
    fn set_bit(&mut self, index: usize, value: bool);

    fn canary(&self) -> Result<usize, ()>;
    fn get_bit(&self, index: usize) -> bool;
}

impl WriteOnceBitField for u32 {
    fn get_bit(&self, index: usize) -> bool {
        (*self & (1 << index)) != 0
    }

    fn set_bit(&mut self, bit: usize, value: bool) {
        if value {
            *self |= 1 << bit;
        } else {
            *self &= !(1 << bit);
        }
    }

    fn set(&mut self, index: usize) -> Result<(), ()> {
        if self.get_bit(index) {
            Err(())
        } else {
            self.set_bit(index, true);
            Ok(())
        }
    }

    fn canary(&self) -> Result<usize, ()> {

        for index in 0..32 {
            if !self.get_bit(index) {
                return Ok(index)
            }
        }

        Err(())
    }
}
