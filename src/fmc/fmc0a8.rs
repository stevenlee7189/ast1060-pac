#[doc = "Register `FMC0A8` reader"]
pub type R = crate::R<Fmc0a8Spec>;
#[doc = "Register `FMC0A8` writer"]
pub type W = crate::W<Fmc0a8Spec>;
#[doc = "Field `LockFMC00FromWrUntilRstBySRST` reader - Lock FMC00 from write until reset by SRST#"]
pub type LockFmc00fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC00FromWrUntilRstBySRST` writer - Lock FMC00 from write until reset by SRST#"]
pub type LockFmc00fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC04FromWrUntilRstBySRST` reader - Lock FMC04 from write until reset by SRST#"]
pub type LockFmc04fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC04FromWrUntilRstBySRST` writer - Lock FMC04 from write until reset by SRST#"]
pub type LockFmc04fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC10FromWrUntilRstBySRST` reader - Lock FMC10 from write until reset by SRST#"]
pub type LockFmc10fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC10FromWrUntilRstBySRST` writer - Lock FMC10 from write until reset by SRST#"]
pub type LockFmc10fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC14FromWrUntilRstBySRST` reader - Lock FMC14 from write until reset by SRST#"]
pub type LockFmc14fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC14FromWrUntilRstBySRST` writer - Lock FMC14 from write until reset by SRST#"]
pub type LockFmc14fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC18FromWrUntilRstBySRST` reader - Lock FMC18 from write until reset by SRST#"]
pub type LockFmc18fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC18FromWrUntilRstBySRST` writer - Lock FMC18 from write until reset by SRST#"]
pub type LockFmc18fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC30FromWrUntilRstBySRST` reader - Lock FMC30 from write until reset by SRST#"]
pub type LockFmc30fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC30FromWrUntilRstBySRST` writer - Lock FMC30 from write until reset by SRST#"]
pub type LockFmc30fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC34FromWrUntilRstBySRST` reader - Lock FMC34 from write until reset by SRST#"]
pub type LockFmc34fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC34FromWrUntilRstBySRST` writer - Lock FMC34 from write until reset by SRST#"]
pub type LockFmc34fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC38FromWrUntilRstBySRST` reader - Lock FMC38 from write until reset by SRST#"]
pub type LockFmc38fromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC38FromWrUntilRstBySRST` writer - Lock FMC38 from write until reset by SRST#"]
pub type LockFmc38fromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMC94FMC98FMC9CFromWrUntilRstBySRST` reader - Lock FMC94/FMC98/FMC9C from write until reset by SRST#"]
pub type LockFmc94fmc98fmc9cfromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMC94FMC98FMC9CFromWrUntilRstBySRST` writer - Lock FMC94/FMC98/FMC9C from write until reset by SRST#"]
pub type LockFmc94fmc98fmc9cfromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMCA0AndFQCDAQCDFromWrUntilRstBySRST` reader - Lock FMCA0 and FQCD/AQCD from write until reset by SRST#"]
pub type LockFmca0andFqcdaqcdfromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMCA0AndFQCDAQCDFromWrUntilRstBySRST` writer - Lock FMCA0 and FQCD/AQCD from write until reset by SRST#"]
pub type LockFmca0andFqcdaqcdfromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LockFMCA4AndAddrFilterFromWrUntilRstBySRST` reader - Lock FMCA4 and Address Filter from write until reset by SRST#"]
pub type LockFmca4andAddrFilterFromWrUntilRstBySrstR = crate::BitReader;
#[doc = "Field `LockFMCA4AndAddrFilterFromWrUntilRstBySRST` writer - Lock FMCA4 and Address Filter from write until reset by SRST#"]
pub type LockFmca4andAddrFilterFromWrUntilRstBySrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock FMC00 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc00from_wr_until_rst_by_srst(&self) -> LockFmc00fromWrUntilRstBySrstR {
        LockFmc00fromWrUntilRstBySrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock FMC04 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc04from_wr_until_rst_by_srst(&self) -> LockFmc04fromWrUntilRstBySrstR {
        LockFmc04fromWrUntilRstBySrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock FMC10 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc10from_wr_until_rst_by_srst(&self) -> LockFmc10fromWrUntilRstBySrstR {
        LockFmc10fromWrUntilRstBySrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock FMC14 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc14from_wr_until_rst_by_srst(&self) -> LockFmc14fromWrUntilRstBySrstR {
        LockFmc14fromWrUntilRstBySrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock FMC18 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc18from_wr_until_rst_by_srst(&self) -> LockFmc18fromWrUntilRstBySrstR {
        LockFmc18fromWrUntilRstBySrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock FMC30 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc30from_wr_until_rst_by_srst(&self) -> LockFmc30fromWrUntilRstBySrstR {
        LockFmc30fromWrUntilRstBySrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock FMC34 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc34from_wr_until_rst_by_srst(&self) -> LockFmc34fromWrUntilRstBySrstR {
        LockFmc34fromWrUntilRstBySrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock FMC38 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc38from_wr_until_rst_by_srst(&self) -> LockFmc38fromWrUntilRstBySrstR {
        LockFmc38fromWrUntilRstBySrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock FMC94/FMC98/FMC9C from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc94fmc98fmc9cfrom_wr_until_rst_by_srst(
        &self,
    ) -> LockFmc94fmc98fmc9cfromWrUntilRstBySrstR {
        LockFmc94fmc98fmc9cfromWrUntilRstBySrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock FMCA0 and FQCD/AQCD from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmca0and_fqcdaqcdfrom_wr_until_rst_by_srst(
        &self,
    ) -> LockFmca0andFqcdaqcdfromWrUntilRstBySrstR {
        LockFmca0andFqcdaqcdfromWrUntilRstBySrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock FMCA4 and Address Filter from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmca4and_addr_filter_from_wr_until_rst_by_srst(
        &self,
    ) -> LockFmca4andAddrFilterFromWrUntilRstBySrstR {
        LockFmca4andAddrFilterFromWrUntilRstBySrstR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock FMC00 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc00from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc00fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc00fromWrUntilRstBySrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock FMC04 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc04from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc04fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc04fromWrUntilRstBySrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Lock FMC10 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc10from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc10fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc10fromWrUntilRstBySrstW::new(self, 2)
    }
    #[doc = "Bit 3 - Lock FMC14 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc14from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc14fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc14fromWrUntilRstBySrstW::new(self, 3)
    }
    #[doc = "Bit 4 - Lock FMC18 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc18from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc18fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc18fromWrUntilRstBySrstW::new(self, 4)
    }
    #[doc = "Bit 5 - Lock FMC30 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc30from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc30fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc30fromWrUntilRstBySrstW::new(self, 5)
    }
    #[doc = "Bit 6 - Lock FMC34 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc34from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc34fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc34fromWrUntilRstBySrstW::new(self, 6)
    }
    #[doc = "Bit 7 - Lock FMC38 from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc38from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc38fromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc38fromWrUntilRstBySrstW::new(self, 7)
    }
    #[doc = "Bit 8 - Lock FMC94/FMC98/FMC9C from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmc94fmc98fmc9cfrom_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmc94fmc98fmc9cfromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmc94fmc98fmc9cfromWrUntilRstBySrstW::new(self, 8)
    }
    #[doc = "Bit 9 - Lock FMCA0 and FQCD/AQCD from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmca0and_fqcdaqcdfrom_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmca0andFqcdaqcdfromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmca0andFqcdaqcdfromWrUntilRstBySrstW::new(self, 9)
    }
    #[doc = "Bit 10 - Lock FMCA4 and Address Filter from write until reset by SRST#"]
    #[inline(always)]
    pub fn lock_fmca4and_addr_filter_from_wr_until_rst_by_srst(
        &mut self,
    ) -> LockFmca4andAddrFilterFromWrUntilRstBySrstW<Fmc0a8Spec> {
        LockFmca4andAddrFilterFromWrUntilRstBySrstW::new(self, 10)
    }
}
#[doc = "Register Lock Control Register (SRST\\#)\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0a8Spec;
impl crate::RegisterSpec for Fmc0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0a8::R`](R) reader structure"]
impl crate::Readable for Fmc0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc0a8::W`](W) writer structure"]
impl crate::Writable for Fmc0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0A8 to value 0"]
impl crate::Resettable for Fmc0a8Spec {}
