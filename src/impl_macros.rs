
macro_rules! impl_channel_clamp {
    ($name: ident, $param: ty) => {
        fn clamp(&self, min: T, max: T) -> Self {
            if self.0 > max {
                $name(max)
            } else if self.0 < min {
                $name(min)
            } else {
                self.clone()
            }
        }
    }
}
