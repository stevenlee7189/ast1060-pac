#[doc = "Register `SCUFB4` reader"]
pub type R = crate::R<Scufb4Spec>;
#[doc = "Register `SCUFB4` writer"]
pub type W = crate::W<Scufb4Spec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU510` reader - Enable hlinkARMRSTN as reset source of hlinkSCU510"]
pub type EnblArmrstnasRstSrcOfHlinkScu510R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU510` writer - Enable hlinkARMRSTN as reset source of hlinkSCU510"]
pub type EnblArmrstnasRstSrcOfHlinkScu510W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU514` reader - Enable hlinkARMRSTN as reset source of hlinkSCU514"]
pub type EnblArmrstnasRstSrcOfHlinkScu514R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU514` writer - Enable hlinkARMRSTN as reset source of hlinkSCU514"]
pub type EnblArmrstnasRstSrcOfHlinkScu514W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU518` reader - Enable hlinkARMRSTN as reset source of hlinkSCU518"]
pub type EnblArmrstnasRstSrcOfHlinkScu518R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU518` writer - Enable hlinkARMRSTN as reset source of hlinkSCU518"]
pub type EnblArmrstnasRstSrcOfHlinkScu518W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU530` reader - Enable hlinkARMRSTN as reset source of hlinkSCU530"]
pub type EnblArmrstnasRstSrcOfHlinkScu530R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU530` writer - Enable hlinkARMRSTN as reset source of hlinkSCU530"]
pub type EnblArmrstnasRstSrcOfHlinkScu530W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU550` reader - Enable hlinkARMRSTN as reset source of hlinkSCU550"]
pub type EnblArmrstnasRstSrcOfHlinkScu550R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU550` writer - Enable hlinkARMRSTN as reset source of hlinkSCU550"]
pub type EnblArmrstnasRstSrcOfHlinkScu550W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU590` reader - Enable hlinkARMRSTN as reset source of hlinkSCU590"]
pub type EnblArmrstnasRstSrcOfHlinkScu590R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU590` writer - Enable hlinkARMRSTN as reset source of hlinkSCU590"]
pub type EnblArmrstnasRstSrcOfHlinkScu590W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5B0` reader - Enable hlinkARMRSTN as reset source of hlinkSCU5B0"]
pub type EnblArmrstnasRstSrcOfHlinkScu5b0R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5B0` writer - Enable hlinkARMRSTN as reset source of hlinkSCU5B0"]
pub type EnblArmrstnasRstSrcOfHlinkScu5b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5B4` reader - Enable hlinkARMRSTN as reset source of hlinkSCU5B4"]
pub type EnblArmrstnasRstSrcOfHlinkScu5b4R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5B4` writer - Enable hlinkARMRSTN as reset source of hlinkSCU5B4"]
pub type EnblArmrstnasRstSrcOfHlinkScu5b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5B8` reader - Enable hlinkARMRSTN as reset source of hlinkSCU5B8"]
pub type EnblArmrstnasRstSrcOfHlinkScu5b8R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5B8` writer - Enable hlinkARMRSTN as reset source of hlinkSCU5B8"]
pub type EnblArmrstnasRstSrcOfHlinkScu5b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5BC` reader - Enable hlinkARMRSTN as reset source of hlinkSCU5BC"]
pub type EnblArmrstnasRstSrcOfHlinkScu5bcR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU5BC` writer - Enable hlinkARMRSTN as reset source of hlinkSCU5BC"]
pub type EnblArmrstnasRstSrcOfHlinkScu5bcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU510"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu510(&self) -> EnblArmrstnasRstSrcOfHlinkScu510R {
        EnblArmrstnasRstSrcOfHlinkScu510R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU514"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu514(&self) -> EnblArmrstnasRstSrcOfHlinkScu514R {
        EnblArmrstnasRstSrcOfHlinkScu514R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable hlinkARMRSTN as reset source of hlinkSCU518"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu518(&self) -> EnblArmrstnasRstSrcOfHlinkScu518R {
        EnblArmrstnasRstSrcOfHlinkScu518R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU530"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu530(&self) -> EnblArmrstnasRstSrcOfHlinkScu530R {
        EnblArmrstnasRstSrcOfHlinkScu530R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU550"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu550(&self) -> EnblArmrstnasRstSrcOfHlinkScu550R {
        EnblArmrstnasRstSrcOfHlinkScu550R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Enable hlinkARMRSTN as reset source of hlinkSCU590"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu590(&self) -> EnblArmrstnasRstSrcOfHlinkScu590R {
        EnblArmrstnasRstSrcOfHlinkScu590R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Enable hlinkARMRSTN as reset source of hlinkSCU5B0"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5b0(&self) -> EnblArmrstnasRstSrcOfHlinkScu5b0R {
        EnblArmrstnasRstSrcOfHlinkScu5b0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable hlinkARMRSTN as reset source of hlinkSCU5B4"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5b4(&self) -> EnblArmrstnasRstSrcOfHlinkScu5b4R {
        EnblArmrstnasRstSrcOfHlinkScu5b4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable hlinkARMRSTN as reset source of hlinkSCU5B8"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5b8(&self) -> EnblArmrstnasRstSrcOfHlinkScu5b8R {
        EnblArmrstnasRstSrcOfHlinkScu5b8R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable hlinkARMRSTN as reset source of hlinkSCU5BC"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5bc(&self) -> EnblArmrstnasRstSrcOfHlinkScu5bcR {
        EnblArmrstnasRstSrcOfHlinkScu5bcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU510"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu510(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu510W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu510W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU514"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu514(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu514W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu514W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable hlinkARMRSTN as reset source of hlinkSCU518"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu518(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu518W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu518W::new(self, 2)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU530"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu530(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu530W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu530W::new(self, 4)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU550"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu550(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu550W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu550W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable hlinkARMRSTN as reset source of hlinkSCU590"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu590(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu590W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu590W::new(self, 16)
    }
    #[doc = "Bit 20 - Enable hlinkARMRSTN as reset source of hlinkSCU5B0"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5b0(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu5b0W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu5b0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable hlinkARMRSTN as reset source of hlinkSCU5B4"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5b4(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu5b4W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu5b4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable hlinkARMRSTN as reset source of hlinkSCU5B8"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5b8(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu5b8W<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu5b8W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable hlinkARMRSTN as reset source of hlinkSCU5BC"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu5bc(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu5bcW<Scufb4Spec> {
        EnblArmrstnasRstSrcOfHlinkScu5bcW::new(self, 23)
    }
}
#[doc = "Reset Source Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scufb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scufb4Spec;
impl crate::RegisterSpec for Scufb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scufb4::R`](R) reader structure"]
impl crate::Readable for Scufb4Spec {}
#[doc = "`write(|w| ..)` method takes [`scufb4::W`](W) writer structure"]
impl crate::Writable for Scufb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUFB4 to value 0"]
impl crate::Resettable for Scufb4Spec {}
