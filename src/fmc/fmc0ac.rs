#[doc = "Register `FMC0AC` reader"]
pub type R = crate::R<Fmc0acSpec>;
#[doc = "Register `FMC0AC` writer"]
pub type W = crate::W<Fmc0acSpec>;
#[doc = "Field `LockFMC00FromWrUntilRstByWatchdog` reader - Lock FMC00 from write until reset by watchdog"]
pub type LockFmc00fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC00FromWrUntilRstByWatchdog` writer - Lock FMC00 from write until reset by watchdog"]
pub type LockFmc00fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC04FromWrUntilRstByWatchdog` reader - Lock FMC04 from write until reset by watchdog"]
pub type LockFmc04fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC04FromWrUntilRstByWatchdog` writer - Lock FMC04 from write until reset by watchdog"]
pub type LockFmc04fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC10FromWrUntilRstByWatchdog` reader - Lock FMC10 from write until reset by watchdog"]
pub type LockFmc10fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC10FromWrUntilRstByWatchdog` writer - Lock FMC10 from write until reset by watchdog"]
pub type LockFmc10fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC14FromWrUntilRstByWatchdog` reader - Lock FMC14 from write until reset by watchdog"]
pub type LockFmc14fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC14FromWrUntilRstByWatchdog` writer - Lock FMC14 from write until reset by watchdog"]
pub type LockFmc14fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC18FromWrUntilRstByWatchdog` reader - Lock FMC18 from write until reset by watchdog"]
pub type LockFmc18fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC18FromWrUntilRstByWatchdog` writer - Lock FMC18 from write until reset by watchdog"]
pub type LockFmc18fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC30FromWrUntilRstByWatchdog` reader - Lock FMC30 from write until reset by watchdog"]
pub type LockFmc30fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC30FromWrUntilRstByWatchdog` writer - Lock FMC30 from write until reset by watchdog"]
pub type LockFmc30fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC34FromWrUntilRstByWatchdog` reader - Lock FMC34 from write until reset by watchdog"]
pub type LockFmc34fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC34FromWrUntilRstByWatchdog` writer - Lock FMC34 from write until reset by watchdog"]
pub type LockFmc34fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC38FromWrUntilRstByWatchdog` reader - Lock FMC38 from write until reset by watchdog"]
pub type LockFmc38fromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC38FromWrUntilRstByWatchdog` writer - Lock FMC38 from write until reset by watchdog"]
pub type LockFmc38fromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC94FMC98FMC9CFromWrUntilRstByWatchdog` reader - Lock FMC94/FMC98/FMC9C from write until reset by watchdog"]
pub type LockFmc94fmc98fmc9cfromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMC94FMC98FMC9CFromWrUntilRstByWatchdog` writer - Lock FMC94/FMC98/FMC9C from write until reset by watchdog"]
pub type LockFmc94fmc98fmc9cfromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMCA0AndFQCDAQCDFromWrUntilRstByWatchdog` reader - Lock FMCA0 and FQCD/AQCD from write until reset by watchdog"]
pub type LockFmca0andFqcdaqcdfromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMCA0AndFQCDAQCDFromWrUntilRstByWatchdog` writer - Lock FMCA0 and FQCD/AQCD from write until reset by watchdog"]
pub type LockFmca0andFqcdaqcdfromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMCA4AndAddrFilterFromWrUntilRstByWatchdog` reader - Lock FMCA4 and Address Filter from write until reset by watchdog"]
pub type LockFmca4andAddrFilterFromWrUntilRstByWatchdogR = crate::BitReader;
#[doc = "Field `LockFMCA4AndAddrFilterFromWrUntilRstByWatchdog` writer - Lock FMCA4 and Address Filter from write until reset by watchdog"]
pub type LockFmca4andAddrFilterFromWrUntilRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock FMC00 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc00from_wr_until_rst_by_watchdog(&self) -> LockFmc00fromWrUntilRstByWatchdogR {
        LockFmc00fromWrUntilRstByWatchdogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock FMC04 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc04from_wr_until_rst_by_watchdog(&self) -> LockFmc04fromWrUntilRstByWatchdogR {
        LockFmc04fromWrUntilRstByWatchdogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock FMC10 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc10from_wr_until_rst_by_watchdog(&self) -> LockFmc10fromWrUntilRstByWatchdogR {
        LockFmc10fromWrUntilRstByWatchdogR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock FMC14 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc14from_wr_until_rst_by_watchdog(&self) -> LockFmc14fromWrUntilRstByWatchdogR {
        LockFmc14fromWrUntilRstByWatchdogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock FMC18 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc18from_wr_until_rst_by_watchdog(&self) -> LockFmc18fromWrUntilRstByWatchdogR {
        LockFmc18fromWrUntilRstByWatchdogR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock FMC30 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc30from_wr_until_rst_by_watchdog(&self) -> LockFmc30fromWrUntilRstByWatchdogR {
        LockFmc30fromWrUntilRstByWatchdogR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock FMC34 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc34from_wr_until_rst_by_watchdog(&self) -> LockFmc34fromWrUntilRstByWatchdogR {
        LockFmc34fromWrUntilRstByWatchdogR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock FMC38 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc38from_wr_until_rst_by_watchdog(&self) -> LockFmc38fromWrUntilRstByWatchdogR {
        LockFmc38fromWrUntilRstByWatchdogR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock FMC94/FMC98/FMC9C from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc94fmc98fmc9cfrom_wr_until_rst_by_watchdog(
        &self,
    ) -> LockFmc94fmc98fmc9cfromWrUntilRstByWatchdogR {
        LockFmc94fmc98fmc9cfromWrUntilRstByWatchdogR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock FMCA0 and FQCD/AQCD from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmca0and_fqcdaqcdfrom_wr_until_rst_by_watchdog(
        &self,
    ) -> LockFmca0andFqcdaqcdfromWrUntilRstByWatchdogR {
        LockFmca0andFqcdaqcdfromWrUntilRstByWatchdogR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock FMCA4 and Address Filter from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmca4and_addr_filter_from_wr_until_rst_by_watchdog(
        &self,
    ) -> LockFmca4andAddrFilterFromWrUntilRstByWatchdogR {
        LockFmca4andAddrFilterFromWrUntilRstByWatchdogR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock FMC00 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc00from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc00fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc00fromWrUntilRstByWatchdogW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock FMC04 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc04from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc04fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc04fromWrUntilRstByWatchdogW::new(self, 1)
    }
    #[doc = "Bit 2 - Lock FMC10 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc10from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc10fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc10fromWrUntilRstByWatchdogW::new(self, 2)
    }
    #[doc = "Bit 3 - Lock FMC14 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc14from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc14fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc14fromWrUntilRstByWatchdogW::new(self, 3)
    }
    #[doc = "Bit 4 - Lock FMC18 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc18from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc18fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc18fromWrUntilRstByWatchdogW::new(self, 4)
    }
    #[doc = "Bit 5 - Lock FMC30 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc30from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc30fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc30fromWrUntilRstByWatchdogW::new(self, 5)
    }
    #[doc = "Bit 6 - Lock FMC34 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc34from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc34fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc34fromWrUntilRstByWatchdogW::new(self, 6)
    }
    #[doc = "Bit 7 - Lock FMC38 from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc38from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc38fromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc38fromWrUntilRstByWatchdogW::new(self, 7)
    }
    #[doc = "Bit 8 - Lock FMC94/FMC98/FMC9C from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmc94fmc98fmc9cfrom_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmc94fmc98fmc9cfromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmc94fmc98fmc9cfromWrUntilRstByWatchdogW::new(self, 8)
    }
    #[doc = "Bit 9 - Lock FMCA0 and FQCD/AQCD from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmca0and_fqcdaqcdfrom_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmca0andFqcdaqcdfromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmca0andFqcdaqcdfromWrUntilRstByWatchdogW::new(self, 9)
    }
    #[doc = "Bit 10 - Lock FMCA4 and Address Filter from write until reset by watchdog"]
    #[inline(always)]
    pub fn lock_fmca4and_addr_filter_from_wr_until_rst_by_watchdog(
        &mut self,
    ) -> LockFmca4andAddrFilterFromWrUntilRstByWatchdogW<Fmc0acSpec> {
        LockFmca4andAddrFilterFromWrUntilRstByWatchdogW::new(self, 10)
    }
}
#[doc = "Register Lock Control Register (Watchdog)\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0acSpec;
impl crate::RegisterSpec for Fmc0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0ac::R`](R) reader structure"]
impl crate::Readable for Fmc0acSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc0ac::W`](W) writer structure"]
impl crate::Writable for Fmc0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0AC to value 0"]
impl crate::Resettable for Fmc0acSpec {}
