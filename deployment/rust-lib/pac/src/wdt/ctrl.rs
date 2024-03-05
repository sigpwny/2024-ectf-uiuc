#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `INT_LATE_VAL` reader - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type INT_LATE_VAL_R = crate::FieldReader<INT_LATE_VAL_A>;
#[doc = "Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT_LATE_VAL_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<INT_LATE_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_LATE_VAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INT_LATE_VAL_A {
    type Ux = u8;
}
impl INT_LATE_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_LATE_VAL_A {
        match self.bits {
            0 => INT_LATE_VAL_A::WDT2POW31,
            1 => INT_LATE_VAL_A::WDT2POW30,
            2 => INT_LATE_VAL_A::WDT2POW29,
            3 => INT_LATE_VAL_A::WDT2POW28,
            4 => INT_LATE_VAL_A::WDT2POW27,
            5 => INT_LATE_VAL_A::WDT2POW26,
            6 => INT_LATE_VAL_A::WDT2POW25,
            7 => INT_LATE_VAL_A::WDT2POW24,
            8 => INT_LATE_VAL_A::WDT2POW23,
            9 => INT_LATE_VAL_A::WDT2POW22,
            10 => INT_LATE_VAL_A::WDT2POW21,
            11 => INT_LATE_VAL_A::WDT2POW20,
            12 => INT_LATE_VAL_A::WDT2POW19,
            13 => INT_LATE_VAL_A::WDT2POW18,
            14 => INT_LATE_VAL_A::WDT2POW17,
            15 => INT_LATE_VAL_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == INT_LATE_VAL_A::WDT2POW16
    }
}
#[doc = "Field `INT_LATE_VAL` writer - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type INT_LATE_VAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, INT_LATE_VAL_A>;
impl<'a, REG> INT_LATE_VAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_VAL_A::WDT2POW16)
    }
}
#[doc = "Field `RST_LATE_VAL` reader - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RST_LATE_VAL_R = crate::FieldReader<RST_LATE_VAL_A>;
#[doc = "Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RST_LATE_VAL_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<RST_LATE_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: RST_LATE_VAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RST_LATE_VAL_A {
    type Ux = u8;
}
impl RST_LATE_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_LATE_VAL_A {
        match self.bits {
            0 => RST_LATE_VAL_A::WDT2POW31,
            1 => RST_LATE_VAL_A::WDT2POW30,
            2 => RST_LATE_VAL_A::WDT2POW29,
            3 => RST_LATE_VAL_A::WDT2POW28,
            4 => RST_LATE_VAL_A::WDT2POW27,
            5 => RST_LATE_VAL_A::WDT2POW26,
            6 => RST_LATE_VAL_A::WDT2POW25,
            7 => RST_LATE_VAL_A::WDT2POW24,
            8 => RST_LATE_VAL_A::WDT2POW23,
            9 => RST_LATE_VAL_A::WDT2POW22,
            10 => RST_LATE_VAL_A::WDT2POW21,
            11 => RST_LATE_VAL_A::WDT2POW20,
            12 => RST_LATE_VAL_A::WDT2POW19,
            13 => RST_LATE_VAL_A::WDT2POW18,
            14 => RST_LATE_VAL_A::WDT2POW17,
            15 => RST_LATE_VAL_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == RST_LATE_VAL_A::WDT2POW16
    }
}
#[doc = "Field `RST_LATE_VAL` writer - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RST_LATE_VAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, RST_LATE_VAL_A>;
impl<'a, REG> RST_LATE_VAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_VAL_A::WDT2POW16)
    }
}
#[doc = "Field `EN` reader - Windowed Watchdog Timer Enable."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Windowed Watchdog Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DIS,
            true => EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
}
#[doc = "Field `EN` writer - Windowed Watchdog Timer Enable."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::EN)
    }
}
#[doc = "Field `INT_LATE` reader - Windowed Watchdog Timer Interrupt Flag Too Late."]
pub type INT_LATE_R = crate::BitReader<INT_LATE_A>;
#[doc = "Windowed Watchdog Timer Interrupt Flag Too Late.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_LATE_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<INT_LATE_A> for bool {
    #[inline(always)]
    fn from(variant: INT_LATE_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_LATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_LATE_A {
        match self.bits {
            false => INT_LATE_A::INACTIVE,
            true => INT_LATE_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == INT_LATE_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INT_LATE_A::PENDING
    }
}
#[doc = "Field `INT_LATE` writer - Windowed Watchdog Timer Interrupt Flag Too Late."]
pub type INT_LATE_W<'a, REG> = crate::BitWriter<'a, REG, INT_LATE_A>;
impl<'a, REG> INT_LATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(INT_LATE_A::PENDING)
    }
}
#[doc = "Field `WDT_INT_EN` reader - Windowed Watchdog Timer Interrupt Enable."]
pub type WDT_INT_EN_R = crate::BitReader<WDT_INT_EN_A>;
#[doc = "Windowed Watchdog Timer Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_INT_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<WDT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_INT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_INT_EN_A {
        match self.bits {
            false => WDT_INT_EN_A::DIS,
            true => WDT_INT_EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WDT_INT_EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WDT_INT_EN_A::EN
    }
}
#[doc = "Field `WDT_INT_EN` writer - Windowed Watchdog Timer Interrupt Enable."]
pub type WDT_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG, WDT_INT_EN_A>;
impl<'a, REG> WDT_INT_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_INT_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_INT_EN_A::EN)
    }
}
#[doc = "Field `WDT_RST_EN` reader - Windowed Watchdog Timer Reset Enable."]
pub type WDT_RST_EN_R = crate::BitReader<WDT_RST_EN_A>;
#[doc = "Windowed Watchdog Timer Reset Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_RST_EN_A {
    #[doc = "0: Disable."]
    DIS = 0,
    #[doc = "1: Enable."]
    EN = 1,
}
impl From<WDT_RST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_RST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_RST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_RST_EN_A {
        match self.bits {
            false => WDT_RST_EN_A::DIS,
            true => WDT_RST_EN_A::EN,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WDT_RST_EN_A::DIS
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WDT_RST_EN_A::EN
    }
}
#[doc = "Field `WDT_RST_EN` writer - Windowed Watchdog Timer Reset Enable."]
pub type WDT_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG, WDT_RST_EN_A>;
impl<'a, REG> WDT_RST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_RST_EN_A::DIS)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_RST_EN_A::EN)
    }
}
#[doc = "Field `INT_EARLY` reader - Windowed Watchdog Timer Interrupt Flag Too Soon."]
pub type INT_EARLY_R = crate::BitReader<INT_EARLY_A>;
#[doc = "Windowed Watchdog Timer Interrupt Flag Too Soon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_EARLY_A {
    #[doc = "0: No interrupt is pending."]
    INACTIVE = 0,
    #[doc = "1: An interrupt is pending."]
    PENDING = 1,
}
impl From<INT_EARLY_A> for bool {
    #[inline(always)]
    fn from(variant: INT_EARLY_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_EARLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_EARLY_A {
        match self.bits {
            false => INT_EARLY_A::INACTIVE,
            true => INT_EARLY_A::PENDING,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == INT_EARLY_A::INACTIVE
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == INT_EARLY_A::PENDING
    }
}
#[doc = "Field `INT_EARLY` writer - Windowed Watchdog Timer Interrupt Flag Too Soon."]
pub type INT_EARLY_W<'a, REG> = crate::BitWriter<'a, REG, INT_EARLY_A>;
impl<'a, REG> INT_EARLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_A::INACTIVE)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_A::PENDING)
    }
}
#[doc = "Field `INT_EARLY_VAL` reader - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type INT_EARLY_VAL_R = crate::FieldReader<INT_EARLY_VAL_A>;
#[doc = "Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT_EARLY_VAL_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<INT_EARLY_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_EARLY_VAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INT_EARLY_VAL_A {
    type Ux = u8;
}
impl INT_EARLY_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INT_EARLY_VAL_A {
        match self.bits {
            0 => INT_EARLY_VAL_A::WDT2POW31,
            1 => INT_EARLY_VAL_A::WDT2POW30,
            2 => INT_EARLY_VAL_A::WDT2POW29,
            3 => INT_EARLY_VAL_A::WDT2POW28,
            4 => INT_EARLY_VAL_A::WDT2POW27,
            5 => INT_EARLY_VAL_A::WDT2POW26,
            6 => INT_EARLY_VAL_A::WDT2POW25,
            7 => INT_EARLY_VAL_A::WDT2POW24,
            8 => INT_EARLY_VAL_A::WDT2POW23,
            9 => INT_EARLY_VAL_A::WDT2POW22,
            10 => INT_EARLY_VAL_A::WDT2POW21,
            11 => INT_EARLY_VAL_A::WDT2POW20,
            12 => INT_EARLY_VAL_A::WDT2POW19,
            13 => INT_EARLY_VAL_A::WDT2POW18,
            14 => INT_EARLY_VAL_A::WDT2POW17,
            15 => INT_EARLY_VAL_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == INT_EARLY_VAL_A::WDT2POW16
    }
}
#[doc = "Field `INT_EARLY_VAL` writer - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type INT_EARLY_VAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, INT_EARLY_VAL_A>;
impl<'a, REG> INT_EARLY_VAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(INT_EARLY_VAL_A::WDT2POW16)
    }
}
#[doc = "Field `RST_EARLY_VAL` reader - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RST_EARLY_VAL_R = crate::FieldReader<RST_EARLY_VAL_A>;
#[doc = "Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RST_EARLY_VAL_A {
    #[doc = "0: 2**31 clock cycles."]
    WDT2POW31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    WDT2POW30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    WDT2POW29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    WDT2POW28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    WDT2POW27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    WDT2POW26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    WDT2POW25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    WDT2POW24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    WDT2POW23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    WDT2POW22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    WDT2POW21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    WDT2POW20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    WDT2POW19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    WDT2POW18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    WDT2POW17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    WDT2POW16 = 15,
}
impl From<RST_EARLY_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: RST_EARLY_VAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RST_EARLY_VAL_A {
    type Ux = u8;
}
impl RST_EARLY_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_EARLY_VAL_A {
        match self.bits {
            0 => RST_EARLY_VAL_A::WDT2POW31,
            1 => RST_EARLY_VAL_A::WDT2POW30,
            2 => RST_EARLY_VAL_A::WDT2POW29,
            3 => RST_EARLY_VAL_A::WDT2POW28,
            4 => RST_EARLY_VAL_A::WDT2POW27,
            5 => RST_EARLY_VAL_A::WDT2POW26,
            6 => RST_EARLY_VAL_A::WDT2POW25,
            7 => RST_EARLY_VAL_A::WDT2POW24,
            8 => RST_EARLY_VAL_A::WDT2POW23,
            9 => RST_EARLY_VAL_A::WDT2POW22,
            10 => RST_EARLY_VAL_A::WDT2POW21,
            11 => RST_EARLY_VAL_A::WDT2POW20,
            12 => RST_EARLY_VAL_A::WDT2POW19,
            13 => RST_EARLY_VAL_A::WDT2POW18,
            14 => RST_EARLY_VAL_A::WDT2POW17,
            15 => RST_EARLY_VAL_A::WDT2POW16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == RST_EARLY_VAL_A::WDT2POW16
    }
}
#[doc = "Field `RST_EARLY_VAL` writer - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RST_EARLY_VAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, RST_EARLY_VAL_A>;
impl<'a, REG> RST_EARLY_VAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_VAL_A::WDT2POW16)
    }
}
#[doc = "Field `CLKRDY_IE` reader - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
pub type CLKRDY_IE_R = crate::BitReader;
#[doc = "Field `CLKRDY_IE` writer - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
pub type CLKRDY_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY` reader - Clock Status."]
pub type CLKRDY_R = crate::BitReader;
#[doc = "Field `CLKRDY` writer - Clock Status."]
pub type CLKRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN_EN` reader - Enables the Windowed Watchdog Function."]
pub type WIN_EN_R = crate::BitReader<WIN_EN_A>;
#[doc = "Enables the Windowed Watchdog Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIN_EN_A {
    #[doc = "0: Windowed Mode Disabled (i.e. Compatibility Mode)."]
    DIS = 0,
    #[doc = "1: Windowed Mode Enabled."]
    EN = 1,
}
impl From<WIN_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WIN_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl WIN_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WIN_EN_A {
        match self.bits {
            false => WIN_EN_A::DIS,
            true => WIN_EN_A::EN,
        }
    }
    #[doc = "Windowed Mode Disabled (i.e. Compatibility Mode)."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WIN_EN_A::DIS
    }
    #[doc = "Windowed Mode Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WIN_EN_A::EN
    }
}
#[doc = "Field `WIN_EN` writer - Enables the Windowed Watchdog Function."]
pub type WIN_EN_W<'a, REG> = crate::BitWriter<'a, REG, WIN_EN_A>;
impl<'a, REG> WIN_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Windowed Mode Disabled (i.e. Compatibility Mode)."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WIN_EN_A::DIS)
    }
    #[doc = "Windowed Mode Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WIN_EN_A::EN)
    }
}
#[doc = "Field `RST_EARLY` reader - Windowed Watchdog Timer Reset Flag Too Soon."]
pub type RST_EARLY_R = crate::BitReader<RST_EARLY_A>;
#[doc = "Windowed Watchdog Timer Reset Flag Too Soon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_EARLY_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<RST_EARLY_A> for bool {
    #[inline(always)]
    fn from(variant: RST_EARLY_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_EARLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_EARLY_A {
        match self.bits {
            false => RST_EARLY_A::NO_EVENT,
            true => RST_EARLY_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST_EARLY_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RST_EARLY_A::OCCURRED
    }
}
#[doc = "Field `RST_EARLY` writer - Windowed Watchdog Timer Reset Flag Too Soon."]
pub type RST_EARLY_W<'a, REG> = crate::BitWriter<'a, REG, RST_EARLY_A>;
impl<'a, REG> RST_EARLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(RST_EARLY_A::OCCURRED)
    }
}
#[doc = "Field `RST_LATE` reader - Windowed Watchdog Timer Reset Flag Too Late."]
pub type RST_LATE_R = crate::BitReader<RST_LATE_A>;
#[doc = "Windowed Watchdog Timer Reset Flag Too Late.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_LATE_A {
    #[doc = "0: The event has not occurred."]
    NO_EVENT = 0,
    #[doc = "1: The event has occurred."]
    OCCURRED = 1,
}
impl From<RST_LATE_A> for bool {
    #[inline(always)]
    fn from(variant: RST_LATE_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_LATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_LATE_A {
        match self.bits {
            false => RST_LATE_A::NO_EVENT,
            true => RST_LATE_A::OCCURRED,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST_LATE_A::NO_EVENT
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RST_LATE_A::OCCURRED
    }
}
#[doc = "Field `RST_LATE` writer - Windowed Watchdog Timer Reset Flag Too Late."]
pub type RST_LATE_W<'a, REG> = crate::BitWriter<'a, REG, RST_LATE_A>;
impl<'a, REG> RST_LATE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_A::NO_EVENT)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(RST_LATE_A::OCCURRED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_late_val(&self) -> INT_LATE_VAL_R {
        INT_LATE_VAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_late_val(&self) -> RST_LATE_VAL_R {
        RST_LATE_VAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Windowed Watchdog Timer Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Windowed Watchdog Timer Interrupt Flag Too Late."]
    #[inline(always)]
    pub fn int_late(&self) -> INT_LATE_R {
        INT_LATE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Windowed Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    pub fn wdt_int_en(&self) -> WDT_INT_EN_R {
        WDT_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Windowed Watchdog Timer Reset Enable."]
    #[inline(always)]
    pub fn wdt_rst_en(&self) -> WDT_RST_EN_R {
        WDT_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Windowed Watchdog Timer Interrupt Flag Too Soon."]
    #[inline(always)]
    pub fn int_early(&self) -> INT_EARLY_R {
        INT_EARLY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_early_val(&self) -> INT_EARLY_VAL_R {
        INT_EARLY_VAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_early_val(&self) -> RST_EARLY_VAL_R {
        RST_EARLY_VAL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
    #[inline(always)]
    pub fn clkrdy_ie(&self) -> CLKRDY_IE_R {
        CLKRDY_IE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Clock Status."]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the Windowed Watchdog Function."]
    #[inline(always)]
    pub fn win_en(&self) -> WIN_EN_R {
        WIN_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Windowed Watchdog Timer Reset Flag Too Soon."]
    #[inline(always)]
    pub fn rst_early(&self) -> RST_EARLY_R {
        RST_EARLY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Windowed Watchdog Timer Reset Flag Too Late."]
    #[inline(always)]
    pub fn rst_late(&self) -> RST_LATE_R {
        RST_LATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    #[must_use]
    pub fn int_late_val(&mut self) -> INT_LATE_VAL_W<CTRL_SPEC> {
        INT_LATE_VAL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst_late_val(&mut self) -> RST_LATE_VAL_W<CTRL_SPEC> {
        RST_LATE_VAL_W::new(self, 4)
    }
    #[doc = "Bit 8 - Windowed Watchdog Timer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Windowed Watchdog Timer Interrupt Flag Too Late."]
    #[inline(always)]
    #[must_use]
    pub fn int_late(&mut self) -> INT_LATE_W<CTRL_SPEC> {
        INT_LATE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Windowed Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_en(&mut self) -> WDT_INT_EN_W<CTRL_SPEC> {
        WDT_INT_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Windowed Watchdog Timer Reset Enable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_rst_en(&mut self) -> WDT_RST_EN_W<CTRL_SPEC> {
        WDT_RST_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Windowed Watchdog Timer Interrupt Flag Too Soon."]
    #[inline(always)]
    #[must_use]
    pub fn int_early(&mut self) -> INT_EARLY_W<CTRL_SPEC> {
        INT_EARLY_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    #[must_use]
    pub fn int_early_val(&mut self) -> INT_EARLY_VAL_W<CTRL_SPEC> {
        INT_EARLY_VAL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst_early_val(&mut self) -> RST_EARLY_VAL_W<CTRL_SPEC> {
        RST_EARLY_VAL_W::new(self, 20)
    }
    #[doc = "Bit 27 - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy_ie(&mut self) -> CLKRDY_IE_W<CTRL_SPEC> {
        CLKRDY_IE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clock Status."]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy(&mut self) -> CLKRDY_W<CTRL_SPEC> {
        CLKRDY_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enables the Windowed Watchdog Function."]
    #[inline(always)]
    #[must_use]
    pub fn win_en(&mut self) -> WIN_EN_W<CTRL_SPEC> {
        WIN_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Windowed Watchdog Timer Reset Flag Too Soon."]
    #[inline(always)]
    #[must_use]
    pub fn rst_early(&mut self) -> RST_EARLY_W<CTRL_SPEC> {
        RST_EARLY_W::new(self, 30)
    }
    #[doc = "Bit 31 - Windowed Watchdog Timer Reset Flag Too Late."]
    #[inline(always)]
    #[must_use]
    pub fn rst_late(&mut self) -> RST_LATE_W<CTRL_SPEC> {
        RST_LATE_W::new(self, 31)
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
#[doc = "Watchdog Timer Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
