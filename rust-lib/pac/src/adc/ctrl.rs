#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `start` reader - Start ADC Conversion"]
pub type START_R = crate::BitReader;
#[doc = "Field `start` writer - Start ADC Conversion"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwr` reader - ADC Power Up"]
pub type PWR_R = crate::BitReader;
#[doc = "Field `pwr` writer - ADC Power Up"]
pub type PWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refbuf_pwr` reader - ADC Reference Buffer Power Up"]
pub type REFBUF_PWR_R = crate::BitReader;
#[doc = "Field `refbuf_pwr` writer - ADC Reference Buffer Power Up"]
pub type REFBUF_PWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ref_sel` reader - ADC Reference Select"]
pub type REF_SEL_R = crate::BitReader;
#[doc = "Field `ref_sel` writer - ADC Reference Select"]
pub type REF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ref_scale` reader - ADC Reference Scale"]
pub type REF_SCALE_R = crate::BitReader;
#[doc = "Field `ref_scale` writer - ADC Reference Scale"]
pub type REF_SCALE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scale` reader - ADC Scale"]
pub type SCALE_R = crate::BitReader;
#[doc = "Field `scale` writer - ADC Scale"]
pub type SCALE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_en` reader - ADC Clock Enable"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `clk_en` writer - ADC Clock Enable"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_sel` reader - ADC Channel Select"]
pub type CH_SEL_R = crate::FieldReader<CH_SEL_A>;
#[doc = "ADC Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_SEL_A {
    #[doc = "0: `0`"]
    AIN0 = 0,
    #[doc = "1: `1`"]
    AIN1 = 1,
    #[doc = "2: `10`"]
    AIN2 = 2,
    #[doc = "3: `11`"]
    AIN3 = 3,
    #[doc = "4: `100`"]
    AIN4 = 4,
    #[doc = "5: `101`"]
    AIN5 = 5,
    #[doc = "6: `110`"]
    AIN6 = 6,
    #[doc = "7: `111`"]
    AIN7 = 7,
    #[doc = "8: `1000`"]
    VCORE_A = 8,
    #[doc = "9: `1001`"]
    VCORE_B = 9,
    #[doc = "10: `1010`"]
    VRXOUT = 10,
    #[doc = "11: `1011`"]
    VTXOUT = 11,
    #[doc = "12: `1100`"]
    VDD_A = 12,
    #[doc = "13: VddB/4"]
    VDD_B = 13,
    #[doc = "14: Vddio/4"]
    VDDIO = 14,
    #[doc = "15: Vddioh/4"]
    VDDIOH = 15,
    #[doc = "16: VregI/4"]
    VREG_I = 16,
}
impl From<CH_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH_SEL_A {
    type Ux = u8;
}
impl CH_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH_SEL_A> {
        match self.bits {
            0 => Some(CH_SEL_A::AIN0),
            1 => Some(CH_SEL_A::AIN1),
            2 => Some(CH_SEL_A::AIN2),
            3 => Some(CH_SEL_A::AIN3),
            4 => Some(CH_SEL_A::AIN4),
            5 => Some(CH_SEL_A::AIN5),
            6 => Some(CH_SEL_A::AIN6),
            7 => Some(CH_SEL_A::AIN7),
            8 => Some(CH_SEL_A::VCORE_A),
            9 => Some(CH_SEL_A::VCORE_B),
            10 => Some(CH_SEL_A::VRXOUT),
            11 => Some(CH_SEL_A::VTXOUT),
            12 => Some(CH_SEL_A::VDD_A),
            13 => Some(CH_SEL_A::VDD_B),
            14 => Some(CH_SEL_A::VDDIO),
            15 => Some(CH_SEL_A::VDDIOH),
            16 => Some(CH_SEL_A::VREG_I),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == CH_SEL_A::AIN0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == CH_SEL_A::AIN1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == CH_SEL_A::AIN2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == CH_SEL_A::AIN3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == CH_SEL_A::AIN4
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == CH_SEL_A::AIN5
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == CH_SEL_A::AIN6
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == CH_SEL_A::AIN7
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_vcore_a(&self) -> bool {
        *self == CH_SEL_A::VCORE_A
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_vcore_b(&self) -> bool {
        *self == CH_SEL_A::VCORE_B
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_vrxout(&self) -> bool {
        *self == CH_SEL_A::VRXOUT
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_vtxout(&self) -> bool {
        *self == CH_SEL_A::VTXOUT
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_vdd_a(&self) -> bool {
        *self == CH_SEL_A::VDD_A
    }
    #[doc = "VddB/4"]
    #[inline(always)]
    pub fn is_vdd_b(&self) -> bool {
        *self == CH_SEL_A::VDD_B
    }
    #[doc = "Vddio/4"]
    #[inline(always)]
    pub fn is_vddio(&self) -> bool {
        *self == CH_SEL_A::VDDIO
    }
    #[doc = "Vddioh/4"]
    #[inline(always)]
    pub fn is_vddioh(&self) -> bool {
        *self == CH_SEL_A::VDDIOH
    }
    #[doc = "VregI/4"]
    #[inline(always)]
    pub fn is_vreg_i(&self) -> bool {
        *self == CH_SEL_A::VREG_I
    }
}
#[doc = "Field `ch_sel` writer - ADC Channel Select"]
pub type CH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, CH_SEL_A>;
impl<'a, REG> CH_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::AIN7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn vcore_a(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VCORE_A)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn vcore_b(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VCORE_B)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn vrxout(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VRXOUT)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn vtxout(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VTXOUT)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn vdd_a(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VDD_A)
    }
    #[doc = "VddB/4"]
    #[inline(always)]
    pub fn vdd_b(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VDD_B)
    }
    #[doc = "Vddio/4"]
    #[inline(always)]
    pub fn vddio(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VDDIO)
    }
    #[doc = "Vddioh/4"]
    #[inline(always)]
    pub fn vddioh(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VDDIOH)
    }
    #[doc = "VregI/4"]
    #[inline(always)]
    pub fn vreg_i(self) -> &'a mut crate::W<REG> {
        self.variant(CH_SEL_A::VREG_I)
    }
}
#[doc = "Field `adc_divsel` reader - Scales the external inputs, all inputs are scaled the same"]
pub type ADC_DIVSEL_R = crate::FieldReader<ADC_DIVSEL_A>;
#[doc = "Scales the external inputs, all inputs are scaled the same\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC_DIVSEL_A {
    #[doc = "0: `0`"]
    DIV1 = 0,
    #[doc = "1: `1`"]
    DIV2 = 1,
    #[doc = "2: `10`"]
    DIV3 = 2,
    #[doc = "3: `11`"]
    DIV4 = 3,
}
impl From<ADC_DIVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_DIVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC_DIVSEL_A {
    type Ux = u8;
}
impl ADC_DIVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC_DIVSEL_A {
        match self.bits {
            0 => ADC_DIVSEL_A::DIV1,
            1 => ADC_DIVSEL_A::DIV2,
            2 => ADC_DIVSEL_A::DIV3,
            3 => ADC_DIVSEL_A::DIV4,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ADC_DIVSEL_A::DIV1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADC_DIVSEL_A::DIV2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == ADC_DIVSEL_A::DIV3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADC_DIVSEL_A::DIV4
    }
}
#[doc = "Field `adc_divsel` writer - Scales the external inputs, all inputs are scaled the same"]
pub type ADC_DIVSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ADC_DIVSEL_A>;
impl<'a, REG> ADC_DIVSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DIVSEL_A::DIV1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DIVSEL_A::DIV2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DIVSEL_A::DIV3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC_DIVSEL_A::DIV4)
    }
}
#[doc = "Field `data_align` reader - ADC Data Alignment Select"]
pub type DATA_ALIGN_R = crate::BitReader;
#[doc = "Field `data_align` writer - ADC Data Alignment Select"]
pub type DATA_ALIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn pwr(&self) -> PWR_R {
        PWR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn refbuf_pwr(&self) -> REFBUF_PWR_R {
        REFBUF_PWR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Reference Select"]
    #[inline(always)]
    pub fn ref_sel(&self) -> REF_SEL_R {
        REF_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn ref_scale(&self) -> REF_SCALE_R {
        REF_SCALE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - Scales the external inputs, all inputs are scaled the same"]
    #[inline(always)]
    pub fn adc_divsel(&self) -> ADC_DIVSEL_R {
        ADC_DIVSEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn data_align(&self) -> DATA_ALIGN_R {
        DATA_ALIGN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTRL_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    #[must_use]
    pub fn pwr(&mut self) -> PWR_W<CTRL_SPEC> {
        PWR_W::new(self, 1)
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    #[must_use]
    pub fn refbuf_pwr(&mut self) -> REFBUF_PWR_W<CTRL_SPEC> {
        REFBUF_PWR_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn ref_sel(&mut self) -> REF_SEL_W<CTRL_SPEC> {
        REF_SEL_W::new(self, 4)
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    #[must_use]
    pub fn ref_scale(&mut self) -> REF_SCALE_W<CTRL_SPEC> {
        REF_SCALE_W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<CTRL_SPEC> {
        SCALE_W::new(self, 9)
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTRL_SPEC> {
        CLK_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:16 - ADC Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch_sel(&mut self) -> CH_SEL_W<CTRL_SPEC> {
        CH_SEL_W::new(self, 12)
    }
    #[doc = "Bits 17:18 - Scales the external inputs, all inputs are scaled the same"]
    #[inline(always)]
    #[must_use]
    pub fn adc_divsel(&mut self) -> ADC_DIVSEL_W<CTRL_SPEC> {
        ADC_DIVSEL_W::new(self, 17)
    }
    #[doc = "Bit 20 - ADC Data Alignment Select"]
    #[inline(always)]
    #[must_use]
    pub fn data_align(&mut self) -> DATA_ALIGN_W<CTRL_SPEC> {
        DATA_ALIGN_W::new(self, 20)
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
#[doc = "ADC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
