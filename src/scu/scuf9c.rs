#[doc = "Register `SCUF9C` reader"]
pub type R = crate::R<Scuf9cSpec>;
#[doc = "Register `SCUF9C` writer"]
pub type W = crate::W<Scuf9cSpec>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU310` reader - Enable hlinkARMRSTN as reset source of hlinkSCU310"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu310R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU310` writer - Enable hlinkARMRSTN as reset source of hlinkSCU310"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU314` reader - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu314R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU314` writer - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU330` reader - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu330R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU330` writer - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU334` reader - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu334R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU334` writer - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU338` reader - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu338R = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU338` writer - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu338W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU33C` reader - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu33cR = crate::BitReader;
#[doc = "Field `EnblHlinkARMRSTNAsRstSrcOfHlinkSCU33C` writer - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
pub type EnblHlinkArmrstnasRstSrcOfHlinkScu33cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU310"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu310(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu310R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu310R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu314(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu314R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu314R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu330(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu330R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu330R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu334(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu334R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu334R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu338(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu338R {
        EnblHlinkArmrstnasRstSrcOfHlinkScu338R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu33c(
        &self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu33cR {
        EnblHlinkArmrstnasRstSrcOfHlinkScu33cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCU310"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu310(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu310W<Scuf9cSpec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu310W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu314(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu314W<Scuf9cSpec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu314W::new(self, 1)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu330(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu330W<Scuf9cSpec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu330W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu334(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu334W<Scuf9cSpec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu334W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu338(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu338W<Scuf9cSpec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu338W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_hlink_armrstnas_rst_src_of_hlink_scu33c(
        &mut self,
    ) -> EnblHlinkArmrstnasRstSrcOfHlinkScu33cW<Scuf9cSpec> {
        EnblHlinkArmrstnasRstSrcOfHlinkScu33cW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scuf9cSpec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scuf9cSpec> {
        Reserved1W::new(self, 10)
    }
}
#[doc = "Reset Source Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf9c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf9c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf9cSpec;
impl crate::RegisterSpec for Scuf9cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf9c::R`](R) reader structure"]
impl crate::Readable for Scuf9cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf9c::W`](W) writer structure"]
impl crate::Writable for Scuf9cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF9C to value 0"]
impl crate::Resettable for Scuf9cSpec {}
