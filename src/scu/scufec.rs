#[doc = "Register `SCUFEC` reader"]
pub type R = crate::R<ScufecSpec>;
#[doc = "Register `SCUFEC` writer"]
pub type W = crate::W<ScufecSpec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF00` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF00"]
pub type EnblArmrstnasRstSrcOfHlinkScuf00R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF00` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF00"]
pub type EnblArmrstnasRstSrcOfHlinkScuf00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF08` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF08"]
pub type EnblArmrstnasRstSrcOfHlinkScuf08R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF08` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF08"]
pub type EnblArmrstnasRstSrcOfHlinkScuf08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF6C` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF6C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf6cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF6C` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF6C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf6cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCUF00"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf00(&self) -> EnblArmrstnasRstSrcOfHlinkScuf00R {
        EnblArmrstnasRstSrcOfHlinkScuf00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable hlinkARMRSTN as reset source of hlinkSCUF08"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf08(&self) -> EnblArmrstnasRstSrcOfHlinkScuf08R {
        EnblArmrstnasRstSrcOfHlinkScuf08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:14 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCUF6C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf6c(&self) -> EnblArmrstnasRstSrcOfHlinkScuf6cR {
        EnblArmrstnasRstSrcOfHlinkScuf6cR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCUF00"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf00(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf00W<ScufecSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf00W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable hlinkARMRSTN as reset source of hlinkSCUF08"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf08(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf08W<ScufecSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf08W::new(self, 2)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCUF6C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf6c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf6cW<ScufecSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf6cW::new(self, 15)
    }
}
#[doc = "Reset Source Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scufec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScufecSpec;
impl crate::RegisterSpec for ScufecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scufec::R`](R) reader structure"]
impl crate::Readable for ScufecSpec {}
#[doc = "`write(|w| ..)` method takes [`scufec::W`](W) writer structure"]
impl crate::Writable for ScufecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUFEC to value 0"]
impl crate::Resettable for ScufecSpec {}
