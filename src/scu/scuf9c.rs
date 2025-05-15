#[doc = "Register `SCUF9C` reader"]
pub type R = crate::R<Scuf9cSpec>;
#[doc = "Register `SCUF9C` writer"]
pub type W = crate::W<Scuf9cSpec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU310` reader - Enable hlinkARMRSTN as reset source of hlinkSCU310"]
pub type EnblArmrstnasRstSrcOfHlinkScu310R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU310` writer - Enable hlinkARMRSTN as reset source of hlinkSCU310"]
pub type EnblArmrstnasRstSrcOfHlinkScu310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU314` reader - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
pub type EnblArmrstnasRstSrcOfHlinkScu314R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU314` writer - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
pub type EnblArmrstnasRstSrcOfHlinkScu314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU330` reader - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
pub type EnblArmrstnasRstSrcOfHlinkScu330R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU330` writer - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
pub type EnblArmrstnasRstSrcOfHlinkScu330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU334` reader - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
pub type EnblArmrstnasRstSrcOfHlinkScu334R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU334` writer - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
pub type EnblArmrstnasRstSrcOfHlinkScu334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU338` reader - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
pub type EnblArmrstnasRstSrcOfHlinkScu338R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU338` writer - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
pub type EnblArmrstnasRstSrcOfHlinkScu338W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU33C` reader - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
pub type EnblArmrstnasRstSrcOfHlinkScu33cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCU33C` writer - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
pub type EnblArmrstnasRstSrcOfHlinkScu33cW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn enbl_armrstnas_rst_src_of_hlink_scu310(&self) -> EnblArmrstnasRstSrcOfHlinkScu310R {
        EnblArmrstnasRstSrcOfHlinkScu310R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu314(&self) -> EnblArmrstnasRstSrcOfHlinkScu314R {
        EnblArmrstnasRstSrcOfHlinkScu314R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu330(&self) -> EnblArmrstnasRstSrcOfHlinkScu330R {
        EnblArmrstnasRstSrcOfHlinkScu330R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu334(&self) -> EnblArmrstnasRstSrcOfHlinkScu334R {
        EnblArmrstnasRstSrcOfHlinkScu334R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu338(&self) -> EnblArmrstnasRstSrcOfHlinkScu338R {
        EnblArmrstnasRstSrcOfHlinkScu338R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu33c(&self) -> EnblArmrstnasRstSrcOfHlinkScu33cR {
        EnblArmrstnasRstSrcOfHlinkScu33cR::new(((self.bits >> 7) & 1) != 0)
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
    pub fn enbl_armrstnas_rst_src_of_hlink_scu310(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu310W<Scuf9cSpec> {
        EnblArmrstnasRstSrcOfHlinkScu310W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable hlinkARMRSTN as reset source of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu314(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu314W<Scuf9cSpec> {
        EnblArmrstnasRstSrcOfHlinkScu314W::new(self, 1)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu330(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu330W<Scuf9cSpec> {
        EnblArmrstnasRstSrcOfHlinkScu330W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu334(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu334W<Scuf9cSpec> {
        EnblArmrstnasRstSrcOfHlinkScu334W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable hlinkARMRSTN as reset source of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu338(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu338W<Scuf9cSpec> {
        EnblArmrstnasRstSrcOfHlinkScu338W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable hlinkARMRSTN as reset source of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scu33c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScu33cW<Scuf9cSpec> {
        EnblArmrstnasRstSrcOfHlinkScu33cW::new(self, 7)
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
