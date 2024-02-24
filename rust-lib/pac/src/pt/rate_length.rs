#[doc = "Register `RATE_LENGTH` reader"]
pub type R = crate::R<RATE_LENGTH_SPEC>;
#[doc = "Register `RATE_LENGTH` writer"]
pub type W = crate::W<RATE_LENGTH_SPEC>;
#[doc = "Field `rate_control` reader - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
pub type RATE_CONTROL_R = crate::FieldReader<u32>;
#[doc = "Field `rate_control` writer - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
pub type RATE_CONTROL_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `mode` reader - Pulse Train Output Mode/Train Length"]
pub type MODE_R = crate::FieldReader<MODE_A>;
#[doc = "Pulse Train Output Mode/Train Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Pulse train, 32 bit pattern."]
    _32_BIT = 0,
    #[doc = "1: Square wave mode."]
    SQUARE_WAVE = 1,
    #[doc = "2: Pulse train, 2 bit pattern."]
    _2_BIT = 2,
    #[doc = "3: Pulse train, 3 bit pattern."]
    _3_BIT = 3,
    #[doc = "4: Pulse train, 4 bit pattern."]
    _4_BIT = 4,
    #[doc = "5: Pulse train, 5 bit pattern."]
    _5_BIT = 5,
    #[doc = "6: Pulse train, 6 bit pattern."]
    _6_BIT = 6,
    #[doc = "7: Pulse train, 7 bit pattern."]
    _7_BIT = 7,
    #[doc = "8: Pulse train, 8 bit pattern."]
    _8_BIT = 8,
    #[doc = "9: Pulse train, 9 bit pattern."]
    _9_BIT = 9,
    #[doc = "10: Pulse train, 10 bit pattern."]
    _10_BIT = 10,
    #[doc = "11: Pulse train, 11 bit pattern."]
    _11_BIT = 11,
    #[doc = "12: Pulse train, 12 bit pattern."]
    _12_BIT = 12,
    #[doc = "13: Pulse train, 13 bit pattern."]
    _13_BIT = 13,
    #[doc = "14: Pulse train, 14 bit pattern."]
    _14_BIT = 14,
    #[doc = "15: Pulse train, 15 bit pattern."]
    _15_BIT = 15,
    #[doc = "16: Pulse train, 16 bit pattern."]
    _16_BIT = 16,
    #[doc = "17: Pulse train, 17 bit pattern."]
    _17_BIT = 17,
    #[doc = "18: Pulse train, 18 bit pattern."]
    _18_BIT = 18,
    #[doc = "19: Pulse train, 19 bit pattern."]
    _19_BIT = 19,
    #[doc = "20: Pulse train, 20 bit pattern."]
    _20_BIT = 20,
    #[doc = "21: Pulse train, 21 bit pattern."]
    _21_BIT = 21,
    #[doc = "22: Pulse train, 22 bit pattern."]
    _22_BIT = 22,
    #[doc = "23: Pulse train, 23 bit pattern."]
    _23_BIT = 23,
    #[doc = "24: Pulse train, 24 bit pattern."]
    _24_BIT = 24,
    #[doc = "25: Pulse train, 25 bit pattern."]
    _25_BIT = 25,
    #[doc = "26: Pulse train, 26 bit pattern."]
    _26_BIT = 26,
    #[doc = "27: Pulse train, 27 bit pattern."]
    _27_BIT = 27,
    #[doc = "28: Pulse train, 28 bit pattern."]
    _28_BIT = 28,
    #[doc = "29: Pulse train, 29 bit pattern."]
    _29_BIT = 29,
    #[doc = "30: Pulse train, 30 bit pattern."]
    _30_BIT = 30,
    #[doc = "31: Pulse train, 31 bit pattern."]
    _31_BIT = 31,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_32_BIT,
            1 => MODE_A::SQUARE_WAVE,
            2 => MODE_A::_2_BIT,
            3 => MODE_A::_3_BIT,
            4 => MODE_A::_4_BIT,
            5 => MODE_A::_5_BIT,
            6 => MODE_A::_6_BIT,
            7 => MODE_A::_7_BIT,
            8 => MODE_A::_8_BIT,
            9 => MODE_A::_9_BIT,
            10 => MODE_A::_10_BIT,
            11 => MODE_A::_11_BIT,
            12 => MODE_A::_12_BIT,
            13 => MODE_A::_13_BIT,
            14 => MODE_A::_14_BIT,
            15 => MODE_A::_15_BIT,
            16 => MODE_A::_16_BIT,
            17 => MODE_A::_17_BIT,
            18 => MODE_A::_18_BIT,
            19 => MODE_A::_19_BIT,
            20 => MODE_A::_20_BIT,
            21 => MODE_A::_21_BIT,
            22 => MODE_A::_22_BIT,
            23 => MODE_A::_23_BIT,
            24 => MODE_A::_24_BIT,
            25 => MODE_A::_25_BIT,
            26 => MODE_A::_26_BIT,
            27 => MODE_A::_27_BIT,
            28 => MODE_A::_28_BIT,
            29 => MODE_A::_29_BIT,
            30 => MODE_A::_30_BIT,
            31 => MODE_A::_31_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Pulse train, 32 bit pattern."]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == MODE_A::_32_BIT
    }
    #[doc = "Square wave mode."]
    #[inline(always)]
    pub fn is_square_wave(&self) -> bool {
        *self == MODE_A::SQUARE_WAVE
    }
    #[doc = "Pulse train, 2 bit pattern."]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == MODE_A::_2_BIT
    }
    #[doc = "Pulse train, 3 bit pattern."]
    #[inline(always)]
    pub fn is_3_bit(&self) -> bool {
        *self == MODE_A::_3_BIT
    }
    #[doc = "Pulse train, 4 bit pattern."]
    #[inline(always)]
    pub fn is_4_bit(&self) -> bool {
        *self == MODE_A::_4_BIT
    }
    #[doc = "Pulse train, 5 bit pattern."]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == MODE_A::_5_BIT
    }
    #[doc = "Pulse train, 6 bit pattern."]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == MODE_A::_6_BIT
    }
    #[doc = "Pulse train, 7 bit pattern."]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == MODE_A::_7_BIT
    }
    #[doc = "Pulse train, 8 bit pattern."]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == MODE_A::_8_BIT
    }
    #[doc = "Pulse train, 9 bit pattern."]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == MODE_A::_9_BIT
    }
    #[doc = "Pulse train, 10 bit pattern."]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == MODE_A::_10_BIT
    }
    #[doc = "Pulse train, 11 bit pattern."]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == MODE_A::_11_BIT
    }
    #[doc = "Pulse train, 12 bit pattern."]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == MODE_A::_12_BIT
    }
    #[doc = "Pulse train, 13 bit pattern."]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == MODE_A::_13_BIT
    }
    #[doc = "Pulse train, 14 bit pattern."]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == MODE_A::_14_BIT
    }
    #[doc = "Pulse train, 15 bit pattern."]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == MODE_A::_15_BIT
    }
    #[doc = "Pulse train, 16 bit pattern."]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == MODE_A::_16_BIT
    }
    #[doc = "Pulse train, 17 bit pattern."]
    #[inline(always)]
    pub fn is_17_bit(&self) -> bool {
        *self == MODE_A::_17_BIT
    }
    #[doc = "Pulse train, 18 bit pattern."]
    #[inline(always)]
    pub fn is_18_bit(&self) -> bool {
        *self == MODE_A::_18_BIT
    }
    #[doc = "Pulse train, 19 bit pattern."]
    #[inline(always)]
    pub fn is_19_bit(&self) -> bool {
        *self == MODE_A::_19_BIT
    }
    #[doc = "Pulse train, 20 bit pattern."]
    #[inline(always)]
    pub fn is_20_bit(&self) -> bool {
        *self == MODE_A::_20_BIT
    }
    #[doc = "Pulse train, 21 bit pattern."]
    #[inline(always)]
    pub fn is_21_bit(&self) -> bool {
        *self == MODE_A::_21_BIT
    }
    #[doc = "Pulse train, 22 bit pattern."]
    #[inline(always)]
    pub fn is_22_bit(&self) -> bool {
        *self == MODE_A::_22_BIT
    }
    #[doc = "Pulse train, 23 bit pattern."]
    #[inline(always)]
    pub fn is_23_bit(&self) -> bool {
        *self == MODE_A::_23_BIT
    }
    #[doc = "Pulse train, 24 bit pattern."]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == MODE_A::_24_BIT
    }
    #[doc = "Pulse train, 25 bit pattern."]
    #[inline(always)]
    pub fn is_25_bit(&self) -> bool {
        *self == MODE_A::_25_BIT
    }
    #[doc = "Pulse train, 26 bit pattern."]
    #[inline(always)]
    pub fn is_26_bit(&self) -> bool {
        *self == MODE_A::_26_BIT
    }
    #[doc = "Pulse train, 27 bit pattern."]
    #[inline(always)]
    pub fn is_27_bit(&self) -> bool {
        *self == MODE_A::_27_BIT
    }
    #[doc = "Pulse train, 28 bit pattern."]
    #[inline(always)]
    pub fn is_28_bit(&self) -> bool {
        *self == MODE_A::_28_BIT
    }
    #[doc = "Pulse train, 29 bit pattern."]
    #[inline(always)]
    pub fn is_29_bit(&self) -> bool {
        *self == MODE_A::_29_BIT
    }
    #[doc = "Pulse train, 30 bit pattern."]
    #[inline(always)]
    pub fn is_30_bit(&self) -> bool {
        *self == MODE_A::_30_BIT
    }
    #[doc = "Pulse train, 31 bit pattern."]
    #[inline(always)]
    pub fn is_31_bit(&self) -> bool {
        *self == MODE_A::_31_BIT
    }
}
#[doc = "Field `mode` writer - Pulse Train Output Mode/Train Length"]
pub type MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulse train, 32 bit pattern."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_32_BIT)
    }
    #[doc = "Square wave mode."]
    #[inline(always)]
    pub fn square_wave(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::SQUARE_WAVE)
    }
    #[doc = "Pulse train, 2 bit pattern."]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_2_BIT)
    }
    #[doc = "Pulse train, 3 bit pattern."]
    #[inline(always)]
    pub fn _3_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_3_BIT)
    }
    #[doc = "Pulse train, 4 bit pattern."]
    #[inline(always)]
    pub fn _4_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_4_BIT)
    }
    #[doc = "Pulse train, 5 bit pattern."]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_5_BIT)
    }
    #[doc = "Pulse train, 6 bit pattern."]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_6_BIT)
    }
    #[doc = "Pulse train, 7 bit pattern."]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_7_BIT)
    }
    #[doc = "Pulse train, 8 bit pattern."]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_8_BIT)
    }
    #[doc = "Pulse train, 9 bit pattern."]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_9_BIT)
    }
    #[doc = "Pulse train, 10 bit pattern."]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_10_BIT)
    }
    #[doc = "Pulse train, 11 bit pattern."]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_11_BIT)
    }
    #[doc = "Pulse train, 12 bit pattern."]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_12_BIT)
    }
    #[doc = "Pulse train, 13 bit pattern."]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_13_BIT)
    }
    #[doc = "Pulse train, 14 bit pattern."]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_14_BIT)
    }
    #[doc = "Pulse train, 15 bit pattern."]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_15_BIT)
    }
    #[doc = "Pulse train, 16 bit pattern."]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_16_BIT)
    }
    #[doc = "Pulse train, 17 bit pattern."]
    #[inline(always)]
    pub fn _17_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_17_BIT)
    }
    #[doc = "Pulse train, 18 bit pattern."]
    #[inline(always)]
    pub fn _18_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_18_BIT)
    }
    #[doc = "Pulse train, 19 bit pattern."]
    #[inline(always)]
    pub fn _19_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_19_BIT)
    }
    #[doc = "Pulse train, 20 bit pattern."]
    #[inline(always)]
    pub fn _20_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_20_BIT)
    }
    #[doc = "Pulse train, 21 bit pattern."]
    #[inline(always)]
    pub fn _21_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_21_BIT)
    }
    #[doc = "Pulse train, 22 bit pattern."]
    #[inline(always)]
    pub fn _22_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_22_BIT)
    }
    #[doc = "Pulse train, 23 bit pattern."]
    #[inline(always)]
    pub fn _23_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_23_BIT)
    }
    #[doc = "Pulse train, 24 bit pattern."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_24_BIT)
    }
    #[doc = "Pulse train, 25 bit pattern."]
    #[inline(always)]
    pub fn _25_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_25_BIT)
    }
    #[doc = "Pulse train, 26 bit pattern."]
    #[inline(always)]
    pub fn _26_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_26_BIT)
    }
    #[doc = "Pulse train, 27 bit pattern."]
    #[inline(always)]
    pub fn _27_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_27_BIT)
    }
    #[doc = "Pulse train, 28 bit pattern."]
    #[inline(always)]
    pub fn _28_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_28_BIT)
    }
    #[doc = "Pulse train, 29 bit pattern."]
    #[inline(always)]
    pub fn _29_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_29_BIT)
    }
    #[doc = "Pulse train, 30 bit pattern."]
    #[inline(always)]
    pub fn _30_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_30_BIT)
    }
    #[doc = "Pulse train, 31 bit pattern."]
    #[inline(always)]
    pub fn _31_bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_31_BIT)
    }
}
impl R {
    #[doc = "Bits 0:26 - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
    #[inline(always)]
    pub fn rate_control(&self) -> RATE_CONTROL_R {
        RATE_CONTROL_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bits 27:31 - Pulse Train Output Mode/Train Length"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:26 - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
    #[inline(always)]
    #[must_use]
    pub fn rate_control(&mut self) -> RATE_CONTROL_W<RATE_LENGTH_SPEC> {
        RATE_CONTROL_W::new(self, 0)
    }
    #[doc = "Bits 27:31 - Pulse Train Output Mode/Train Length"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<RATE_LENGTH_SPEC> {
        MODE_W::new(self, 27)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pulse Train Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rate_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rate_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RATE_LENGTH_SPEC;
impl crate::RegisterSpec for RATE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rate_length::R`](R) reader structure"]
impl crate::Readable for RATE_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rate_length::W`](W) writer structure"]
impl crate::Writable for RATE_LENGTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATE_LENGTH to value 0"]
impl crate::Resettable for RATE_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
