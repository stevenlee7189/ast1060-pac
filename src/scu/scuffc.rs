#[doc = "Register `SCUFFC` reader"]
pub type R = crate::R<ScuffcSpec>;
#[doc = "Register `SCUFFC` writer"]
pub type W = crate::W<ScuffcSpec>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF10` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF10"]
pub type EnblArmrstnasRstSrcOfHlinkScuf10R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF10` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF10"]
pub type EnblArmrstnasRstSrcOfHlinkScuf10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF18` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF18"]
pub type EnblArmrstnasRstSrcOfHlinkScuf18R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF18` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF18"]
pub type EnblArmrstnasRstSrcOfHlinkScuf18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF1C` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF1C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf1cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF1C` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF1C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF30` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF30"]
pub type EnblArmrstnasRstSrcOfHlinkScuf30R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF30` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF30"]
pub type EnblArmrstnasRstSrcOfHlinkScuf30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF34` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF34"]
pub type EnblArmrstnasRstSrcOfHlinkScuf34R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF34` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF34"]
pub type EnblArmrstnasRstSrcOfHlinkScuf34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF38` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF38"]
pub type EnblArmrstnasRstSrcOfHlinkScuf38R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF38` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF38"]
pub type EnblArmrstnasRstSrcOfHlinkScuf38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF3C` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF3C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf3cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF3C` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF3C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf3cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF50` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF50"]
pub type EnblArmrstnasRstSrcOfHlinkScuf50R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF50` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF50"]
pub type EnblArmrstnasRstSrcOfHlinkScuf50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF58` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF58"]
pub type EnblArmrstnasRstSrcOfHlinkScuf58R = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF58` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF58"]
pub type EnblArmrstnasRstSrcOfHlinkScuf58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF7C` reader - Enable hlinkARMRSTN as reset source of hlinkSCUF7C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf7cR = crate::BitReader;
#[doc = "Field `EnblARMRSTNAsRstSrcOfHlinkSCUF7C` writer - Enable hlinkARMRSTN as reset source of hlinkSCUF7C"]
pub type EnblArmrstnasRstSrcOfHlinkScuf7cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCUF10"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf10(&self) -> EnblArmrstnasRstSrcOfHlinkScuf10R {
        EnblArmrstnasRstSrcOfHlinkScuf10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable hlinkARMRSTN as reset source of hlinkSCUF18"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf18(&self) -> EnblArmrstnasRstSrcOfHlinkScuf18R {
        EnblArmrstnasRstSrcOfHlinkScuf18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable hlinkARMRSTN as reset source of hlinkSCUF1C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf1c(&self) -> EnblArmrstnasRstSrcOfHlinkScuf1cR {
        EnblArmrstnasRstSrcOfHlinkScuf1cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCUF30"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf30(&self) -> EnblArmrstnasRstSrcOfHlinkScuf30R {
        EnblArmrstnasRstSrcOfHlinkScuf30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCUF34"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf34(&self) -> EnblArmrstnasRstSrcOfHlinkScuf34R {
        EnblArmrstnasRstSrcOfHlinkScuf34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable hlinkARMRSTN as reset source of hlinkSCUF38"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf38(&self) -> EnblArmrstnasRstSrcOfHlinkScuf38R {
        EnblArmrstnasRstSrcOfHlinkScuf38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable hlinkARMRSTN as reset source of hlinkSCUF3C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf3c(&self) -> EnblArmrstnasRstSrcOfHlinkScuf3cR {
        EnblArmrstnasRstSrcOfHlinkScuf3cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCUF50"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf50(&self) -> EnblArmrstnasRstSrcOfHlinkScuf50R {
        EnblArmrstnasRstSrcOfHlinkScuf50R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCUF58"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf58(&self) -> EnblArmrstnasRstSrcOfHlinkScuf58R {
        EnblArmrstnasRstSrcOfHlinkScuf58R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCUF7C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf7c(&self) -> EnblArmrstnasRstSrcOfHlinkScuf7cR {
        EnblArmrstnasRstSrcOfHlinkScuf7cR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hlinkARMRSTN as reset source of hlinkSCUF10"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf10(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf10W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf10W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable hlinkARMRSTN as reset source of hlinkSCUF18"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf18(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf18W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf18W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable hlinkARMRSTN as reset source of hlinkSCUF1C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf1c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf1cW<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf1cW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable hlinkARMRSTN as reset source of hlinkSCUF30"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf30(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf30W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf30W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable hlinkARMRSTN as reset source of hlinkSCUF34"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf34(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf34W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf34W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable hlinkARMRSTN as reset source of hlinkSCUF38"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf38(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf38W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf38W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable hlinkARMRSTN as reset source of hlinkSCUF3C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf3c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf3cW<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf3cW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable hlinkARMRSTN as reset source of hlinkSCUF50"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf50(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf50W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf50W::new(self, 8)
    }
    #[doc = "Bit 10 - Enable hlinkARMRSTN as reset source of hlinkSCUF58"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf58(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf58W<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf58W::new(self, 10)
    }
    #[doc = "Bit 15 - Enable hlinkARMRSTN as reset source of hlinkSCUF7C"]
    #[inline(always)]
    pub fn enbl_armrstnas_rst_src_of_hlink_scuf7c(
        &mut self,
    ) -> EnblArmrstnasRstSrcOfHlinkScuf7cW<ScuffcSpec> {
        EnblArmrstnasRstSrcOfHlinkScuf7cW::new(self, 15)
    }
}
#[doc = "Reset Source Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scuffc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuffc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScuffcSpec;
impl crate::RegisterSpec for ScuffcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuffc::R`](R) reader structure"]
impl crate::Readable for ScuffcSpec {}
#[doc = "`write(|w| ..)` method takes [`scuffc::W`](W) writer structure"]
impl crate::Writable for ScuffcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUFFC to value 0"]
impl crate::Resettable for ScuffcSpec {}
