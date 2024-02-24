#[doc = "Register `FCTRL0` reader"]
pub type R = crate::R<FCTRL0_SPEC>;
#[doc = "Register `FCTRL0` writer"]
pub type W = crate::W<FCTRL0_SPEC>;
#[doc = "Field `I2C0DGEN0` reader - I2C0 SDA Pad Deglitcher enable."]
pub type I2C0DGEN0_R = crate::BitReader<I2C0DGEN0_A>;
#[doc = "I2C0 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C0DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0DGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C0DGEN0_A {
        match self.bits {
            false => I2C0DGEN0_A::DIS,
            true => I2C0DGEN0_A::EN,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0DGEN0_A::DIS
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0DGEN0_A::EN
    }
}
#[doc = "Field `I2C0DGEN0` writer - I2C0 SDA Pad Deglitcher enable."]
pub type I2C0DGEN0_W<'a, REG> = crate::BitWriter<'a, REG, I2C0DGEN0_A>;
impl<'a, REG> I2C0DGEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0DGEN0_A::EN)
    }
}
#[doc = "Field `I2C0DGEN1` reader - I2C0 SCL Pad Deglitcher enable."]
pub type I2C0DGEN1_R = crate::BitReader<I2C0DGEN1_A>;
#[doc = "I2C0 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C0DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0DGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C0DGEN1_A {
        match self.bits {
            false => I2C0DGEN1_A::DIS,
            true => I2C0DGEN1_A::EN,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C0DGEN1_A::DIS
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C0DGEN1_A::EN
    }
}
#[doc = "Field `I2C0DGEN1` writer - I2C0 SCL Pad Deglitcher enable."]
pub type I2C0DGEN1_W<'a, REG> = crate::BitWriter<'a, REG, I2C0DGEN1_A>;
impl<'a, REG> I2C0DGEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2C0DGEN1_A::EN)
    }
}
#[doc = "Field `I2C1DGEN0` reader - I2C1 SDA Pad Deglitcher enable."]
pub type I2C1DGEN0_R = crate::BitReader<I2C1DGEN0_A>;
#[doc = "I2C1 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C1DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1DGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1DGEN0_A {
        match self.bits {
            false => I2C1DGEN0_A::DIS,
            true => I2C1DGEN0_A::EN,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1DGEN0_A::DIS
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1DGEN0_A::EN
    }
}
#[doc = "Field `I2C1DGEN0` writer - I2C1 SDA Pad Deglitcher enable."]
pub type I2C1DGEN0_W<'a, REG> = crate::BitWriter<'a, REG, I2C1DGEN0_A>;
impl<'a, REG> I2C1DGEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1DGEN0_A::EN)
    }
}
#[doc = "Field `I2C1DGEN1` reader - I2C1 SCL Pad Deglitcher enable."]
pub type I2C1DGEN1_R = crate::BitReader<I2C1DGEN1_A>;
#[doc = "I2C1 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C1DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1DGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C1DGEN1_A {
        match self.bits {
            false => I2C1DGEN1_A::DIS,
            true => I2C1DGEN1_A::EN,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C1DGEN1_A::DIS
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C1DGEN1_A::EN
    }
}
#[doc = "Field `I2C1DGEN1` writer - I2C1 SCL Pad Deglitcher enable."]
pub type I2C1DGEN1_W<'a, REG> = crate::BitWriter<'a, REG, I2C1DGEN1_A>;
impl<'a, REG> I2C1DGEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1DGEN1_A::EN)
    }
}
#[doc = "Field `I2C2DGEN0` reader - I2C2 SDA Pad Deglitcher enable."]
pub type I2C2DGEN0_R = crate::BitReader<I2C2DGEN0_A>;
#[doc = "I2C2 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2DGEN0_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C2DGEN0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2DGEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2DGEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2DGEN0_A {
        match self.bits {
            false => I2C2DGEN0_A::DIS,
            true => I2C2DGEN0_A::EN,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C2DGEN0_A::DIS
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C2DGEN0_A::EN
    }
}
#[doc = "Field `I2C2DGEN0` writer - I2C2 SDA Pad Deglitcher enable."]
pub type I2C2DGEN0_W<'a, REG> = crate::BitWriter<'a, REG, I2C2DGEN0_A>;
impl<'a, REG> I2C2DGEN0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2DGEN0_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2DGEN0_A::EN)
    }
}
#[doc = "Field `I2C2DGEN1` reader - I2C2 SCL Pad Deglitcher enable."]
pub type I2C2DGEN1_R = crate::BitReader<I2C2DGEN1_A>;
#[doc = "I2C2 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2DGEN1_A {
    #[doc = "0: Deglitcher disabled."]
    DIS = 0,
    #[doc = "1: Deglitcher enabled."]
    EN = 1,
}
impl From<I2C2DGEN1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2DGEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2DGEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C2DGEN1_A {
        match self.bits {
            false => I2C2DGEN1_A::DIS,
            true => I2C2DGEN1_A::EN,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2C2DGEN1_A::DIS
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2C2DGEN1_A::EN
    }
}
#[doc = "Field `I2C2DGEN1` writer - I2C2 SCL Pad Deglitcher enable."]
pub type I2C2DGEN1_W<'a, REG> = crate::BitWriter<'a, REG, I2C2DGEN1_A>;
impl<'a, REG> I2C2DGEN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2DGEN1_A::DIS)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2DGEN1_A::EN)
    }
}
impl R {
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen0(&self) -> I2C0DGEN0_R {
        I2C0DGEN0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen1(&self) -> I2C0DGEN1_R {
        I2C0DGEN1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen0(&self) -> I2C1DGEN0_R {
        I2C1DGEN0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen1(&self) -> I2C1DGEN1_R {
        I2C1DGEN1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C2 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen0(&self) -> I2C2DGEN0_R {
        I2C2DGEN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C2 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen1(&self) -> I2C2DGEN1_R {
        I2C2DGEN1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0dgen0(&mut self) -> I2C0DGEN0_W<FCTRL0_SPEC> {
        I2C0DGEN0_W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0dgen1(&mut self) -> I2C0DGEN1_W<FCTRL0_SPEC> {
        I2C0DGEN1_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1dgen0(&mut self) -> I2C1DGEN0_W<FCTRL0_SPEC> {
        I2C1DGEN0_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c1dgen1(&mut self) -> I2C1DGEN1_W<FCTRL0_SPEC> {
        I2C1DGEN1_W::new(self, 23)
    }
    #[doc = "Bit 24 - I2C2 SDA Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2dgen0(&mut self) -> I2C2DGEN0_W<FCTRL0_SPEC> {
        I2C2DGEN0_W::new(self, 24)
    }
    #[doc = "Bit 25 - I2C2 SCL Pad Deglitcher enable."]
    #[inline(always)]
    #[must_use]
    pub fn i2c2dgen1(&mut self) -> I2C2DGEN1_W<FCTRL0_SPEC> {
        I2C2DGEN1_W::new(self, 25)
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
#[doc = "Function Control 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTRL0_SPEC;
impl crate::RegisterSpec for FCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl0::R`](R) reader structure"]
impl crate::Readable for FCTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctrl0::W`](W) writer structure"]
impl crate::Writable for FCTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTRL0 to value 0"]
impl crate::Resettable for FCTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
