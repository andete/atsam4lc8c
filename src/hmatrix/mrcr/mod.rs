#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MRCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RCB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB0R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB0R::_0 => false,
            RCB0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB0R {
        match value {
            false => RCB0R::_0,
            true => RCB0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB0R::_1
    }
}
#[doc = "Possible values of the field `RCB1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB1R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB1R::_0 => false,
            RCB1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB1R {
        match value {
            false => RCB1R::_0,
            true => RCB1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB1R::_1
    }
}
#[doc = "Possible values of the field `RCB2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB2R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB2R::_0 => false,
            RCB2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB2R {
        match value {
            false => RCB2R::_0,
            true => RCB2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB2R::_1
    }
}
#[doc = "Possible values of the field `RCB3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB3R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB3R::_0 => false,
            RCB3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB3R {
        match value {
            false => RCB3R::_0,
            true => RCB3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB3R::_1
    }
}
#[doc = "Possible values of the field `RCB4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB4R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB4R::_0 => false,
            RCB4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB4R {
        match value {
            false => RCB4R::_0,
            true => RCB4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB4R::_1
    }
}
#[doc = "Possible values of the field `RCB5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB5R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB5R::_0 => false,
            RCB5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB5R {
        match value {
            false => RCB5R::_0,
            true => RCB5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB5R::_1
    }
}
#[doc = "Possible values of the field `RCB6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB6R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB6R::_0 => false,
            RCB6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB6R {
        match value {
            false => RCB6R::_0,
            true => RCB6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB6R::_1
    }
}
#[doc = "Possible values of the field `RCB7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB7R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB7R::_0 => false,
            RCB7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB7R {
        match value {
            false => RCB7R::_0,
            true => RCB7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB7R::_1
    }
}
#[doc = "Possible values of the field `RCB8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB8R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB8R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB8R::_0 => false,
            RCB8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB8R {
        match value {
            false => RCB8R::_0,
            true => RCB8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB8R::_1
    }
}
#[doc = "Possible values of the field `RCB9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB9R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB9R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB9R::_0 => false,
            RCB9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB9R {
        match value {
            false => RCB9R::_0,
            true => RCB9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB9R::_1
    }
}
#[doc = "Possible values of the field `RCB10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB10R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB10R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB10R::_0 => false,
            RCB10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB10R {
        match value {
            false => RCB10R::_0,
            true => RCB10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB10R::_1
    }
}
#[doc = "Possible values of the field `RCB11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB11R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB11R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB11R::_0 => false,
            RCB11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB11R {
        match value {
            false => RCB11R::_0,
            true => RCB11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB11R::_1
    }
}
#[doc = "Possible values of the field `RCB12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB12R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB12R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB12R::_0 => false,
            RCB12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB12R {
        match value {
            false => RCB12R::_0,
            true => RCB12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB12R::_1
    }
}
#[doc = "Possible values of the field `RCB13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB13R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB13R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB13R::_0 => false,
            RCB13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB13R {
        match value {
            false => RCB13R::_0,
            true => RCB13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB13R::_1
    }
}
#[doc = "Possible values of the field `RCB14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB14R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB14R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB14R::_0 => false,
            RCB14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB14R {
        match value {
            false => RCB14R::_0,
            true => RCB14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB14R::_1
    }
}
#[doc = "Possible values of the field `RCB15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCB15R {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB15R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RCB15R::_0 => false,
            RCB15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCB15R {
        match value {
            false => RCB15R::_0,
            true => RCB15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCB15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCB15R::_1
    }
}
#[doc = "Values that can be written to the field `RCB0`"]
pub enum RCB0W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB0W::_0 => false,
            RCB0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB0W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB0W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB1`"]
pub enum RCB1W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB1W::_0 => false,
            RCB1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB1W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB1W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB2`"]
pub enum RCB2W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB2W::_0 => false,
            RCB2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB2W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB2W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB2W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB3`"]
pub enum RCB3W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB3W::_0 => false,
            RCB3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB3W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB3W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB3W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB4`"]
pub enum RCB4W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB4W::_0 => false,
            RCB4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB4W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB4W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB4W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB5`"]
pub enum RCB5W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB5W::_0 => false,
            RCB5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB5W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB5W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB5W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB6`"]
pub enum RCB6W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB6W::_0 => false,
            RCB6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB6W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB6W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB6W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB7`"]
pub enum RCB7W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB7W::_0 => false,
            RCB7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB7W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB7W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB7W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB8`"]
pub enum RCB8W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB8W::_0 => false,
            RCB8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB8W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB8W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB8W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB9`"]
pub enum RCB9W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB9W::_0 => false,
            RCB9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB9W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB9W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB9W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB10`"]
pub enum RCB10W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB10W::_0 => false,
            RCB10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB10W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB10W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB10W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB11`"]
pub enum RCB11W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB11W::_0 => false,
            RCB11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB11W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB11W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB11W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB12`"]
pub enum RCB12W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB12W::_0 => false,
            RCB12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB12W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB12W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB12W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB13`"]
pub enum RCB13W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB13W::_0 => false,
            RCB13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB13W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB13W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB13W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB14`"]
pub enum RCB14W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB14W::_0 => false,
            RCB14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB14W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB14W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB14W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RCB15`"]
pub enum RCB15W {
    #[doc = "Disable remapped address decoding for master"]
    _0,
    #[doc = "Enable remapped address decoding for master"]
    _1,
}
impl RCB15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RCB15W::_0 => false,
            RCB15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RCB15W<'a> {
    w: &'a mut W,
}
impl<'a> _RCB15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RCB15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB15W::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB15W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Remap Command bit for Master 0"]
    #[inline]
    pub fn rcb0(&self) -> RCB0R {
        RCB0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Remap Command bit for Master 1"]
    #[inline]
    pub fn rcb1(&self) -> RCB1R {
        RCB1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Remap Command bit for Master 2"]
    #[inline]
    pub fn rcb2(&self) -> RCB2R {
        RCB2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Remap Command bit for Master 3"]
    #[inline]
    pub fn rcb3(&self) -> RCB3R {
        RCB3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Remap Command bit for Master 4"]
    #[inline]
    pub fn rcb4(&self) -> RCB4R {
        RCB4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Remap Command bit for Master 5"]
    #[inline]
    pub fn rcb5(&self) -> RCB5R {
        RCB5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Remap Command bit for Master 6"]
    #[inline]
    pub fn rcb6(&self) -> RCB6R {
        RCB6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Remap Command bit for Master 7"]
    #[inline]
    pub fn rcb7(&self) -> RCB7R {
        RCB7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Remap Command bit for Master 8"]
    #[inline]
    pub fn rcb8(&self) -> RCB8R {
        RCB8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Remap Command bit for Master 9"]
    #[inline]
    pub fn rcb9(&self) -> RCB9R {
        RCB9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Remap Command bit for Master 10"]
    #[inline]
    pub fn rcb10(&self) -> RCB10R {
        RCB10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Remap Command bit for Master 11"]
    #[inline]
    pub fn rcb11(&self) -> RCB11R {
        RCB11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Remap Command bit for Master 12"]
    #[inline]
    pub fn rcb12(&self) -> RCB12R {
        RCB12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Remap Command bit for Master 13"]
    #[inline]
    pub fn rcb13(&self) -> RCB13R {
        RCB13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Remap Command bit for Master 14"]
    #[inline]
    pub fn rcb14(&self) -> RCB14R {
        RCB14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Remap Command bit for Master 15"]
    #[inline]
    pub fn rcb15(&self) -> RCB15R {
        RCB15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Remap Command bit for Master 0"]
    #[inline]
    pub fn rcb0(&mut self) -> _RCB0W {
        _RCB0W { w: self }
    }
    #[doc = "Bit 1 - Remap Command bit for Master 1"]
    #[inline]
    pub fn rcb1(&mut self) -> _RCB1W {
        _RCB1W { w: self }
    }
    #[doc = "Bit 2 - Remap Command bit for Master 2"]
    #[inline]
    pub fn rcb2(&mut self) -> _RCB2W {
        _RCB2W { w: self }
    }
    #[doc = "Bit 3 - Remap Command bit for Master 3"]
    #[inline]
    pub fn rcb3(&mut self) -> _RCB3W {
        _RCB3W { w: self }
    }
    #[doc = "Bit 4 - Remap Command bit for Master 4"]
    #[inline]
    pub fn rcb4(&mut self) -> _RCB4W {
        _RCB4W { w: self }
    }
    #[doc = "Bit 5 - Remap Command bit for Master 5"]
    #[inline]
    pub fn rcb5(&mut self) -> _RCB5W {
        _RCB5W { w: self }
    }
    #[doc = "Bit 6 - Remap Command bit for Master 6"]
    #[inline]
    pub fn rcb6(&mut self) -> _RCB6W {
        _RCB6W { w: self }
    }
    #[doc = "Bit 7 - Remap Command bit for Master 7"]
    #[inline]
    pub fn rcb7(&mut self) -> _RCB7W {
        _RCB7W { w: self }
    }
    #[doc = "Bit 8 - Remap Command bit for Master 8"]
    #[inline]
    pub fn rcb8(&mut self) -> _RCB8W {
        _RCB8W { w: self }
    }
    #[doc = "Bit 9 - Remap Command bit for Master 9"]
    #[inline]
    pub fn rcb9(&mut self) -> _RCB9W {
        _RCB9W { w: self }
    }
    #[doc = "Bit 10 - Remap Command bit for Master 10"]
    #[inline]
    pub fn rcb10(&mut self) -> _RCB10W {
        _RCB10W { w: self }
    }
    #[doc = "Bit 11 - Remap Command bit for Master 11"]
    #[inline]
    pub fn rcb11(&mut self) -> _RCB11W {
        _RCB11W { w: self }
    }
    #[doc = "Bit 12 - Remap Command bit for Master 12"]
    #[inline]
    pub fn rcb12(&mut self) -> _RCB12W {
        _RCB12W { w: self }
    }
    #[doc = "Bit 13 - Remap Command bit for Master 13"]
    #[inline]
    pub fn rcb13(&mut self) -> _RCB13W {
        _RCB13W { w: self }
    }
    #[doc = "Bit 14 - Remap Command bit for Master 14"]
    #[inline]
    pub fn rcb14(&mut self) -> _RCB14W {
        _RCB14W { w: self }
    }
    #[doc = "Bit 15 - Remap Command bit for Master 15"]
    #[inline]
    pub fn rcb15(&mut self) -> _RCB15W {
        _RCB15W { w: self }
    }
}
