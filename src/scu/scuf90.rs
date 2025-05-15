#[doc = "Register `SCUF90` reader"]
pub type R = crate::R<Scuf90Spec>;
#[doc = "Register `SCUF90` writer"]
pub type W = crate::W<Scuf90Spec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU010HlinkSCU000` reader - Enable hlinkARMRSTN as reset source of hlinkSCU010 hlinkSCU000"]
pub type EnblArmrstnasRstSrcOfHlinkScu010hlinkScu000R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU010HlinkSCU000` writer - Enable hlinkARMRSTN as reset source of hlinkSCU010 hlinkSCU000"]
pub type EnblArmrstnasRstSrcOfHlinkScu010hlinkScu000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU050` reader - Enable hlinkARMRSTN as reset source of hlinkSCU050"]
pub type EnblArmrstnasRstSrcOfHlinkScu050R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU050` writer - Enable hlinkARMRSTN as reset source of hlinkSCU050"]
pub type EnblArmrstnasRstSrcOfHlinkScu050W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU070` reader - Enable hlinkARMRSTN as reset source of hlinkSCU070"]
pub type EnblArmrstnasRstSrcOfHlinkScu070R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU070` writer - Enable hlinkARMRSTN as reset source of hlinkSCU070"]
pub type EnblArmrstnasRstSrcOfHlinkScu070W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU074` reader - Enable hlinkARMRSTN as reset source of hlinkSCU074"]
pub type EnblArmrstnasRstSrcOfHlinkScu074R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU074` writer - Enable hlinkARMRSTN as reset source of hlinkSCU074"]
pub type EnblArmrstnasRstSrcOfHlinkScu074W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU090` reader - Enable hlinkARMRSTN as reset source of hlinkSCU090"]
pub type EnblArmrstnasRstSrcOfHlinkScu090R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU090` writer - Enable hlinkARMRSTN as reset source of hlinkSCU090"]
pub type EnblArmrstnasRstSrcOfHlinkScu090W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU094` reader - Enable hlinkARMRSTN as reset source of hlinkSCU094"]
pub type EnblArmrstnasRstSrcOfHlinkScu094R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU094` writer - Enable hlinkARMRSTN as reset source of hlinkSCU094"]
pub type EnblArmrstnasRstSrcOfHlinkScu094W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0D0` reader - Enable hlinkARMRSTN as reset source of hlinkSCU0D0"]
pub type EnblArmrstnasRstSrcOfHlinkScu0d0R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0D0` writer - Enable hlinkARMRSTN as reset source of hlinkSCU0D0"]
pub type EnblArmrstnasRstSrcOfHlinkScu0d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0D4` reader - Enable hlinkARMRSTN as reset source of hlinkSCU0D4"]
pub type EnblArmrstnasRstSrcOfHlinkScu0d4R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0D4` writer - Enable hlinkARMRSTN as reset source of hlinkSCU0D4"]
pub type EnblArmrstnasRstSrcOfHlinkScu0d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0D8` reader - Enable hlinkARMRSTN as reset source of hlinkSCU0D8"]
pub type EnblArmrstnasRstSrcOfHlinkScu0d8R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0D8` writer - Enable hlinkARMRSTN as reset source of hlinkSCU0D8"]
pub type EnblArmrstnasRstSrcOfHlinkScu0d8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0FC` reader - Enable hlinkARMRSTN as reset source of hlinkSCU0FC"]
pub type EnblArmrstnasRstSrcOfHlinkScu0fcR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU0FC` writer - Enable hlinkARMRSTN as reset source of hlinkSCU0FC"]
pub type EnblArmrstnasRstSrcOfHlinkScu0fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU010 hlinkSCU000"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu010hlink_scu000(
        &self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu010hlinkScu000R {
        EnblArmrstnasRstSrcOfHlinkScu010hlinkScu000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU050"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu050(&self) -> EnblArmrstnasRstSrcOfHlinkScu050R {
        EnblArmrstnasRstSrcOfHlinkScu050R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU070"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu070(&self) -> EnblArmrstnasRstSrcOfHlinkScu070R {
        EnblArmrstnasRstSrcOfHlinkScu070R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable hlinkARMRSTN as reset source of hlinkSCU074"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu074(&self) -> EnblArmrstnasRstSrcOfHlinkScu074R {
        EnblArmrstnasRstSrcOfHlinkScu074R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Enable hlinkARMRSTN as reset source of hlinkSCU090"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu090(&self) -> EnblArmrstnasRstSrcOfHlinkScu090R {
        EnblArmrstnasRstSrcOfHlinkScu090R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable hlinkARMRSTN as reset source of hlinkSCU094"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu094(&self) -> EnblArmrstnasRstSrcOfHlinkScu094R {
        EnblArmrstnasRstSrcOfHlinkScu094R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Enable hlinkARMRSTN as reset source of hlinkSCU0D0"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0d0(&self) -> EnblArmrstnasRstSrcOfHlinkScu0d0R {
        EnblArmrstnasRstSrcOfHlinkScu0d0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable hlinkARMRSTN as reset source of hlinkSCU0D4"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0d4(&self) -> EnblArmrstnasRstSrcOfHlinkScu0d4R {
        EnblArmrstnasRstSrcOfHlinkScu0d4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable hlinkARMRSTN as reset source of hlinkSCU0D8"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0d8(&self) -> EnblArmrstnasRstSrcOfHlinkScu0d8R {
        EnblArmrstnasRstSrcOfHlinkScu0d8R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable hlinkARMRSTN as reset source of hlinkSCU0FC"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0fc(&self) -> EnblArmrstnasRstSrcOfHlinkScu0fcR {
        EnblArmrstnasRstSrcOfHlinkScu0fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU010 hlinkSCU000"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu010hlink_scu000(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu010hlinkScu000W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu010hlinkScu000W::new(self, 0)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCU050"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu050(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu050W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu050W::new(self, 8)
    }
    #[doc = "Bit 12 - Enable hlinkARMRSTN as reset source of hlinkSCU070"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu070(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu070W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu070W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable hlinkARMRSTN as reset source of hlinkSCU074"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu074(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu074W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu074W::new(self, 13)
    }
    #[doc = "Bit 16 - Enable hlinkARMRSTN as reset source of hlinkSCU090"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu090(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu090W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu090W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable hlinkARMRSTN as reset source of hlinkSCU094"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu094(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu094W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu094W::new(self, 17)
    }
    #[doc = "Bit 24 - Enable hlinkARMRSTN as reset source of hlinkSCU0D0"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0d0(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu0d0W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu0d0W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable hlinkARMRSTN as reset source of hlinkSCU0D4"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0d4(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu0d4W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu0d4W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable hlinkARMRSTN as reset source of hlinkSCU0D8"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0d8(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu0d8W<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu0d8W::new(self, 26)
    }
    #[doc = "Bit 31 - Enable hlinkARMRSTN as reset source of hlinkSCU0FC"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu0fc(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu0fcW<Scuf90Spec> {
        EnblArmrstnasRstSrcOfHlinkScu0fcW::new(self, 31)
    }
}
#[doc = "Reset Source Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf90::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf90::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf90Spec;
impl crate::RegisterSpec for Scuf90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf90::R`](R) reader structure"]
impl crate::Readable for Scuf90Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf90::W`](W) writer structure"]
impl crate::Writable for Scuf90Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF90 to value 0"]
impl crate::Resettable for Scuf90Spec {}
