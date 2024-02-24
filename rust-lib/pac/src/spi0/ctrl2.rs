#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CLKPHA` reader - Clock Phase."]
pub type CLKPHA_R = crate::BitReader<CLKPHA_A>;
#[doc = "Clock Phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPHA_A {
    #[doc = "0: Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    RISING_EDGE = 0,
    #[doc = "1: Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    FALLING_EDGE = 1,
}
impl From<CLKPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKPHA_A {
        match self.bits {
            false => CLKPHA_A::RISING_EDGE,
            true => CLKPHA_A::FALLING_EDGE,
        }
    }
    #[doc = "Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CLKPHA_A::RISING_EDGE
    }
    #[doc = "Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CLKPHA_A::FALLING_EDGE
    }
}
#[doc = "Field `CLKPHA` writer - Clock Phase."]
pub type CLKPHA_W<'a, REG> = crate::BitWriter<'a, REG, CLKPHA_A>;
impl<'a, REG> CLKPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPHA_A::RISING_EDGE)
    }
    #[doc = "Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPHA_A::FALLING_EDGE)
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity."]
pub type CLKPOL_R = crate::BitReader<CLKPOL_A>;
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPOL_A {
    #[doc = "0: Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    NORMAL = 0,
    #[doc = "1: Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    INVERTED = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::NORMAL,
            true => CLKPOL_A::INVERTED,
        }
    }
    #[doc = "Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CLKPOL_A::NORMAL
    }
    #[doc = "Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == CLKPOL_A::INVERTED
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity."]
pub type CLKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CLKPOL_A>;
impl<'a, REG> CLKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::NORMAL)
    }
    #[doc = "Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(CLKPOL_A::INVERTED)
    }
}
#[doc = "Field `NUMBITS` reader - Number of Bits per character."]
pub type NUMBITS_R = crate::FieldReader<NUMBITS_A>;
#[doc = "Number of Bits per character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUMBITS_A {
    #[doc = "0: 16 bits per character."]
    _16 = 0,
    #[doc = "1: 1 bits per character."]
    _1 = 1,
    #[doc = "2: 2 bits per character."]
    _2 = 2,
    #[doc = "3: 3 bits per character."]
    _3 = 3,
    #[doc = "4: 4 bits per character."]
    _4 = 4,
    #[doc = "5: 5 bits per character."]
    _5 = 5,
    #[doc = "6: 6 bits per character."]
    _6 = 6,
    #[doc = "7: 7 bits per character."]
    _7 = 7,
    #[doc = "8: 8 bits per character."]
    _8 = 8,
    #[doc = "9: 9 bits per character."]
    _9 = 9,
    #[doc = "10: 10 bits per character."]
    _10 = 10,
    #[doc = "11: 11 bits per character."]
    _11 = 11,
    #[doc = "12: 12 bits per character."]
    _12 = 12,
    #[doc = "13: 13 bits per character."]
    _13 = 13,
    #[doc = "14: 14 bits per character."]
    _14 = 14,
    #[doc = "15: 15 bits per character."]
    _15 = 15,
}
impl From<NUMBITS_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMBITS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NUMBITS_A {
    type Ux = u8;
}
impl NUMBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NUMBITS_A {
        match self.bits {
            0 => NUMBITS_A::_16,
            1 => NUMBITS_A::_1,
            2 => NUMBITS_A::_2,
            3 => NUMBITS_A::_3,
            4 => NUMBITS_A::_4,
            5 => NUMBITS_A::_5,
            6 => NUMBITS_A::_6,
            7 => NUMBITS_A::_7,
            8 => NUMBITS_A::_8,
            9 => NUMBITS_A::_9,
            10 => NUMBITS_A::_10,
            11 => NUMBITS_A::_11,
            12 => NUMBITS_A::_12,
            13 => NUMBITS_A::_13,
            14 => NUMBITS_A::_14,
            15 => NUMBITS_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == NUMBITS_A::_16
    }
    #[doc = "1 bits per character."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NUMBITS_A::_1
    }
    #[doc = "2 bits per character."]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == NUMBITS_A::_2
    }
    #[doc = "3 bits per character."]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == NUMBITS_A::_3
    }
    #[doc = "4 bits per character."]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == NUMBITS_A::_4
    }
    #[doc = "5 bits per character."]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == NUMBITS_A::_5
    }
    #[doc = "6 bits per character."]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == NUMBITS_A::_6
    }
    #[doc = "7 bits per character."]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == NUMBITS_A::_7
    }
    #[doc = "8 bits per character."]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == NUMBITS_A::_8
    }
    #[doc = "9 bits per character."]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == NUMBITS_A::_9
    }
    #[doc = "10 bits per character."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NUMBITS_A::_10
    }
    #[doc = "11 bits per character."]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NUMBITS_A::_11
    }
    #[doc = "12 bits per character."]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == NUMBITS_A::_12
    }
    #[doc = "13 bits per character."]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == NUMBITS_A::_13
    }
    #[doc = "14 bits per character."]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == NUMBITS_A::_14
    }
    #[doc = "15 bits per character."]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == NUMBITS_A::_15
    }
}
#[doc = "Field `NUMBITS` writer - Number of Bits per character."]
pub type NUMBITS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, NUMBITS_A>;
impl<'a, REG> NUMBITS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_16)
    }
    #[doc = "1 bits per character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_1)
    }
    #[doc = "2 bits per character."]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_2)
    }
    #[doc = "3 bits per character."]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_3)
    }
    #[doc = "4 bits per character."]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_4)
    }
    #[doc = "5 bits per character."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_5)
    }
    #[doc = "6 bits per character."]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_6)
    }
    #[doc = "7 bits per character."]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_7)
    }
    #[doc = "8 bits per character."]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_8)
    }
    #[doc = "9 bits per character."]
    #[inline(always)]
    pub fn _9(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_9)
    }
    #[doc = "10 bits per character."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_10)
    }
    #[doc = "11 bits per character."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_11)
    }
    #[doc = "12 bits per character."]
    #[inline(always)]
    pub fn _12(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_12)
    }
    #[doc = "13 bits per character."]
    #[inline(always)]
    pub fn _13(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_13)
    }
    #[doc = "14 bits per character."]
    #[inline(always)]
    pub fn _14(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_14)
    }
    #[doc = "15 bits per character."]
    #[inline(always)]
    pub fn _15(self) -> &'a mut crate::W<REG> {
        self.variant(NUMBITS_A::_15)
    }
}
#[doc = "Field `DATA_WIDTH` reader - SPI Data width."]
pub type DATA_WIDTH_R = crate::FieldReader<DATA_WIDTH_A>;
#[doc = "SPI Data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: 1 data pin."]
    MONO = 0,
    #[doc = "1: 2 data pins."]
    DUAL = 1,
    #[doc = "2: 4 data pins."]
    QUAD = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATA_WIDTH_A {
    type Ux = u8;
}
impl DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DATA_WIDTH_A> {
        match self.bits {
            0 => Some(DATA_WIDTH_A::MONO),
            1 => Some(DATA_WIDTH_A::DUAL),
            2 => Some(DATA_WIDTH_A::QUAD),
            _ => None,
        }
    }
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == DATA_WIDTH_A::MONO
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DATA_WIDTH_A::DUAL
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DATA_WIDTH_A::QUAD
    }
}
#[doc = "Field `DATA_WIDTH` writer - SPI Data width."]
pub type DATA_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATA_WIDTH_A>;
impl<'a, REG> DATA_WIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::MONO)
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::DUAL)
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_WIDTH_A::QUAD)
    }
}
#[doc = "Field `THREE_WIRE` reader - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
pub type THREE_WIRE_R = crate::BitReader<THREE_WIRE_A>;
#[doc = "Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THREE_WIRE_A {
    #[doc = "0: Use four wire mode (Mono only)."]
    DIS = 0,
    #[doc = "1: Use three wire mode."]
    EN = 1,
}
impl From<THREE_WIRE_A> for bool {
    #[inline(always)]
    fn from(variant: THREE_WIRE_A) -> Self {
        variant as u8 != 0
    }
}
impl THREE_WIRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THREE_WIRE_A {
        match self.bits {
            false => THREE_WIRE_A::DIS,
            true => THREE_WIRE_A::EN,
        }
    }
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == THREE_WIRE_A::DIS
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == THREE_WIRE_A::EN
    }
}
#[doc = "Field `THREE_WIRE` writer - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
pub type THREE_WIRE_W<'a, REG> = crate::BitWriter<'a, REG, THREE_WIRE_A>;
impl<'a, REG> THREE_WIRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_WIRE_A::DIS)
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(THREE_WIRE_A::EN)
    }
}
#[doc = "Field `SS_POL` reader - Slave Select Polarity, each Slave Select can have unique polarity."]
pub type SS_POL_R = crate::FieldReader<SS_POL_A>;
#[doc = "Slave Select Polarity, each Slave Select can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SS_POL_A {
    #[doc = "1: SS0 active high."]
    SS0_HIGH = 1,
    #[doc = "2: SS1 active high."]
    SS1_HIGH = 2,
    #[doc = "4: SS2 active high."]
    SS2_HIGH = 4,
    #[doc = "8: SS3 active high."]
    SS3_HIGH = 8,
}
impl From<SS_POL_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_POL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SS_POL_A {
    type Ux = u8;
}
impl SS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SS_POL_A> {
        match self.bits {
            1 => Some(SS_POL_A::SS0_HIGH),
            2 => Some(SS_POL_A::SS1_HIGH),
            4 => Some(SS_POL_A::SS2_HIGH),
            8 => Some(SS_POL_A::SS3_HIGH),
            _ => None,
        }
    }
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn is_ss0_high(&self) -> bool {
        *self == SS_POL_A::SS0_HIGH
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn is_ss1_high(&self) -> bool {
        *self == SS_POL_A::SS1_HIGH
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn is_ss2_high(&self) -> bool {
        *self == SS_POL_A::SS2_HIGH
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn is_ss3_high(&self) -> bool {
        *self == SS_POL_A::SS3_HIGH
    }
}
#[doc = "Field `SS_POL` writer - Slave Select Polarity, each Slave Select can have unique polarity."]
pub type SS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, SS_POL_A>;
impl<'a, REG> SS_POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn ss0_high(self) -> &'a mut crate::W<REG> {
        self.variant(SS_POL_A::SS0_HIGH)
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn ss1_high(self) -> &'a mut crate::W<REG> {
        self.variant(SS_POL_A::SS1_HIGH)
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn ss2_high(self) -> &'a mut crate::W<REG> {
        self.variant(SS_POL_A::SS2_HIGH)
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn ss3_high(self) -> &'a mut crate::W<REG> {
        self.variant(SS_POL_A::SS3_HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&self) -> NUMBITS_R {
        NUMBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&self) -> THREE_WIRE_R {
        THREE_WIRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&self) -> SS_POL_R {
        SS_POL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL2_SPEC> {
        CLKPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL2_SPEC> {
        CLKPOL_W::new(self, 1)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    #[must_use]
    pub fn numbits(&mut self) -> NUMBITS_W<CTRL2_SPEC> {
        NUMBITS_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<CTRL2_SPEC> {
        DATA_WIDTH_W::new(self, 12)
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    #[must_use]
    pub fn three_wire(&mut self) -> THREE_WIRE_W<CTRL2_SPEC> {
        THREE_WIRE_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    #[must_use]
    pub fn ss_pol(&mut self) -> SS_POL_W<CTRL2_SPEC> {
        SS_POL_W::new(self, 16)
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
#[doc = "Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
