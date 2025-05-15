#[doc = "Register `SCUFD0` reader"]
pub type R = crate::R<Scufd0Spec>;
#[doc = "Register `SCUFD0` writer"]
pub type W = crate::W<Scufd0Spec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU830` reader - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
pub type EnblArmrstnasRstSrcOfHlinkScu830R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU830` writer - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
pub type EnblArmrstnasRstSrcOfHlinkScu830W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU834` reader - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
pub type EnblArmrstnasRstSrcOfHlinkScu834R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU834` writer - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
pub type EnblArmrstnasRstSrcOfHlinkScu834W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU850` reader - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
pub type EnblArmrstnasRstSrcOfHlinkScu850R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU850` writer - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
pub type EnblArmrstnasRstSrcOfHlinkScu850W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU854` reader - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
pub type EnblArmrstnasRstSrcOfHlinkScu854R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU854` writer - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
pub type EnblArmrstnasRstSrcOfHlinkScu854W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU858` reader - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
pub type EnblArmrstnasRstSrcOfHlinkScu858R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU858` writer - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
pub type EnblArmrstnasRstSrcOfHlinkScu858W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU85C` reader - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
pub type EnblArmrstnasRstSrcOfHlinkScu85cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU85C` writer - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
pub type EnblArmrstnasRstSrcOfHlinkScu85cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU870` reader - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
pub type EnblArmrstnasRstSrcOfHlinkScu870R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU870` writer - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
pub type EnblArmrstnasRstSrcOfHlinkScu870W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU874` reader - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
pub type EnblArmrstnasRstSrcOfHlinkScu874R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU874` writer - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
pub type EnblArmrstnasRstSrcOfHlinkScu874W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU878` reader - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
pub type EnblArmrstnasRstSrcOfHlinkScu878R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU878` writer - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
pub type EnblArmrstnasRstSrcOfHlinkScu878W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU87C` reader - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
pub type EnblArmrstnasRstSrcOfHlinkScu87cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU87C` writer - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
pub type EnblArmrstnasRstSrcOfHlinkScu87cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu830(&self) -> EnblArmrstnasRstSrcOfHlinkScu830R {
        EnblArmrstnasRstSrcOfHlinkScu830R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu834(&self) -> EnblArmrstnasRstSrcOfHlinkScu834R {
        EnblArmrstnasRstSrcOfHlinkScu834R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu850(&self) -> EnblArmrstnasRstSrcOfHlinkScu850R {
        EnblArmrstnasRstSrcOfHlinkScu850R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu854(&self) -> EnblArmrstnasRstSrcOfHlinkScu854R {
        EnblArmrstnasRstSrcOfHlinkScu854R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu858(&self) -> EnblArmrstnasRstSrcOfHlinkScu858R {
        EnblArmrstnasRstSrcOfHlinkScu858R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu85c(&self) -> EnblArmrstnasRstSrcOfHlinkScu85cR {
        EnblArmrstnasRstSrcOfHlinkScu85cR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu870(&self) -> EnblArmrstnasRstSrcOfHlinkScu870R {
        EnblArmrstnasRstSrcOfHlinkScu870R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu874(&self) -> EnblArmrstnasRstSrcOfHlinkScu874R {
        EnblArmrstnasRstSrcOfHlinkScu874R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu878(&self) -> EnblArmrstnasRstSrcOfHlinkScu878R {
        EnblArmrstnasRstSrcOfHlinkScu878R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu87c(&self) -> EnblArmrstnasRstSrcOfHlinkScu87cR {
        EnblArmrstnasRstSrcOfHlinkScu87cR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu830(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu830W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu830W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu834(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu834W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu834W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu850(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu850W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu850W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable hlinkARMRSTN as reset source of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu854(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu854W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu854W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu858(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu858W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu858W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable hlinkARMRSTN as reset source of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu85c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu85cW<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu85cW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu870(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu870W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu870W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable hlinkARMRSTN as reset source of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu874(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu874W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu874W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable hlinkARMRSTN as reset source of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu878(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu878W<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu878W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu87c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu87cW<Scufd0Spec> {
        EnblArmrstnasRstSrcOfHlinkScu87cW::new(self, 15)
    }
}
#[doc = "Reset Source Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scufd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scufd0Spec;
impl crate::RegisterSpec for Scufd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scufd0::R`](R) reader structure"]
impl crate::Readable for Scufd0Spec {}
#[doc = "`write(|w| ..)` method takes [`scufd0::W`](W) writer structure"]
impl crate::Writable for Scufd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUFD0 to value 0"]
impl crate::Resettable for Scufd0Spec {}
