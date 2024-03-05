#[doc = "Register `SYSCTRL` reader"]
pub type R = crate::R<SYSCTRL_SPEC>;
#[doc = "Register `SYSCTRL` writer"]
pub type W = crate::W<SYSCTRL_SPEC>;
#[doc = "Field `BSTAPEN` reader - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
pub type BSTAPEN_R = crate::BitReader;
#[doc = "Field `BSTAPEN` writer - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
pub type BSTAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PAGE_FLIP` reader - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub type FLASH_PAGE_FLIP_R = crate::BitReader<FLASH_PAGE_FLIP_A>;
#[doc = "Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_PAGE_FLIP_A {
    #[doc = "0: Physical layout matches logical layout."]
    NORMAL = 0,
    #[doc = "1: Bottom half mapped to logical top half and vice versa."]
    SWAPPED = 1,
}
impl From<FLASH_PAGE_FLIP_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PAGE_FLIP_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_PAGE_FLIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASH_PAGE_FLIP_A {
        match self.bits {
            false => FLASH_PAGE_FLIP_A::NORMAL,
            true => FLASH_PAGE_FLIP_A::SWAPPED,
        }
    }
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::NORMAL
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == FLASH_PAGE_FLIP_A::SWAPPED
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` writer - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub type FLASH_PAGE_FLIP_W<'a, REG> = crate::BitWriter<'a, REG, FLASH_PAGE_FLIP_A>;
impl<'a, REG> FLASH_PAGE_FLIP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_PAGE_FLIP_A::NORMAL)
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_PAGE_FLIP_A::SWAPPED)
    }
}
#[doc = "Field `ICC0_FLUSH` reader - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub type ICC0_FLUSH_R = crate::BitReader<ICC0_FLUSH_A>;
#[doc = "Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICC0_FLUSH_A {
    #[doc = "0: Normal Code Cache Operation"]
    NORMAL = 0,
    #[doc = "1: Code Caches and CPU instruction buffer are flushed"]
    FLUSH = 1,
}
impl From<ICC0_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: ICC0_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl ICC0_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ICC0_FLUSH_A {
        match self.bits {
            false => ICC0_FLUSH_A::NORMAL,
            true => ICC0_FLUSH_A::FLUSH,
        }
    }
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ICC0_FLUSH_A::NORMAL
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == ICC0_FLUSH_A::FLUSH
    }
}
#[doc = "Field `ICC0_FLUSH` writer - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub type ICC0_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG, ICC0_FLUSH_A>;
impl<'a, REG> ICC0_FLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ICC0_FLUSH_A::NORMAL)
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(ICC0_FLUSH_A::FLUSH)
    }
}
#[doc = "Field `ROMDONE` reader - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
pub type ROMDONE_R = crate::BitReader;
#[doc = "Field `ROMDONE` writer - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
pub type ROMDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCHK` reader - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
pub type CCHK_R = crate::BitReader<CCHK_A>;
#[doc = "Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCHK_A {
    #[doc = "0: No operation/complete."]
    COMPLETE = 0,
    #[doc = "1: Start operation."]
    START = 1,
}
impl From<CCHK_A> for bool {
    #[inline(always)]
    fn from(variant: CCHK_A) -> Self {
        variant as u8 != 0
    }
}
impl CCHK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCHK_A {
        match self.bits {
            false => CCHK_A::COMPLETE,
            true => CCHK_A::START,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCHK_A::COMPLETE
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CCHK_A::START
    }
}
#[doc = "Field `CCHK` writer - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
pub type CCHK_W<'a, REG> = crate::BitWriter<'a, REG, CCHK_A>;
impl<'a, REG> CCHK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(CCHK_A::COMPLETE)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CCHK_A::START)
    }
}
#[doc = "Field `SWD_DIS` reader - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
pub type SWD_DIS_R = crate::BitReader;
#[doc = "Field `SWD_DIS` writer - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
pub type SWD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHKRES` reader - ROM Checksum Result. This bit is only valid when CHKRD=1."]
pub type CHKRES_R = crate::BitReader<CHKRES_A>;
#[doc = "ROM Checksum Result. This bit is only valid when CHKRD=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHKRES_A {
    #[doc = "0: ROM Checksum Correct."]
    PASS = 0,
    #[doc = "1: ROM Checksum Fail."]
    FAIL = 1,
}
impl From<CHKRES_A> for bool {
    #[inline(always)]
    fn from(variant: CHKRES_A) -> Self {
        variant as u8 != 0
    }
}
impl CHKRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHKRES_A {
        match self.bits {
            false => CHKRES_A::PASS,
            true => CHKRES_A::FAIL,
        }
    }
    #[doc = "ROM Checksum Correct."]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CHKRES_A::PASS
    }
    #[doc = "ROM Checksum Fail."]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == CHKRES_A::FAIL
    }
}
#[doc = "Field `CHKRES` writer - ROM Checksum Result. This bit is only valid when CHKRD=1."]
pub type CHKRES_W<'a, REG> = crate::BitWriter<'a, REG, CHKRES_A>;
impl<'a, REG> CHKRES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ROM Checksum Correct."]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(CHKRES_A::PASS)
    }
    #[doc = "ROM Checksum Fail."]
    #[inline(always)]
    pub fn fail(self) -> &'a mut crate::W<REG> {
        self.variant(CHKRES_A::FAIL)
    }
}
#[doc = "Field `OVR` reader - Operating Voltage Range."]
pub type OVR_R = crate::FieldReader;
#[doc = "Field `OVR` writer - Operating Voltage Range."]
pub type OVR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
    #[inline(always)]
    pub fn bstapen(&self) -> BSTAPEN_R {
        BSTAPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&self) -> FLASH_PAGE_FLIP_R {
        FLASH_PAGE_FLIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn icc0_flush(&self) -> ICC0_FLUSH_R {
        ICC0_FLUSH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
    #[inline(always)]
    pub fn romdone(&self) -> ROMDONE_R {
        ROMDONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    pub fn cchk(&self) -> CCHK_R {
        CCHK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
    #[inline(always)]
    pub fn swd_dis(&self) -> SWD_DIS_R {
        SWD_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    pub fn chkres(&self) -> CHKRES_R {
        CHKRES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
    #[inline(always)]
    #[must_use]
    pub fn bstapen(&mut self) -> BSTAPEN_W<SYSCTRL_SPEC> {
        BSTAPEN_W::new(self, 1)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    #[must_use]
    pub fn flash_page_flip(&mut self) -> FLASH_PAGE_FLIP_W<SYSCTRL_SPEC> {
        FLASH_PAGE_FLIP_W::new(self, 4)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    #[must_use]
    pub fn icc0_flush(&mut self) -> ICC0_FLUSH_W<SYSCTRL_SPEC> {
        ICC0_FLUSH_W::new(self, 6)
    }
    #[doc = "Bit 12 - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
    #[inline(always)]
    #[must_use]
    pub fn romdone(&mut self) -> ROMDONE_W<SYSCTRL_SPEC> {
        ROMDONE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    #[must_use]
    pub fn cchk(&mut self) -> CCHK_W<SYSCTRL_SPEC> {
        CCHK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
    #[inline(always)]
    #[must_use]
    pub fn swd_dis(&mut self) -> SWD_DIS_W<SYSCTRL_SPEC> {
        SWD_DIS_W::new(self, 14)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    #[must_use]
    pub fn chkres(&mut self) -> CHKRES_W<SYSCTRL_SPEC> {
        CHKRES_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<SYSCTRL_SPEC> {
        OVR_W::new(self, 16)
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
#[doc = "System Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCTRL_SPEC;
impl crate::RegisterSpec for SYSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctrl::R`](R) reader structure"]
impl crate::Readable for SYSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysctrl::W`](W) writer structure"]
impl crate::Writable for SYSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCTRL to value 0"]
impl crate::Resettable for SYSCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
