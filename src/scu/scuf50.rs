#[doc = "Register `SCUF50` reader"]
pub type R = crate::R<Scuf50Spec>;
#[doc = "Register `SCUF50` writer"]
pub type W = crate::W<Scuf50Spec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU830` reader - Enable Write Protection of hlinkSCU830"]
pub type EnblWrProtOfScu830R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU830` writer - Enable Write Protection of hlinkSCU830"]
pub type EnblWrProtOfScu830W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU834` reader - Enable Write Protection of hlinkSCU834"]
pub type EnblWrProtOfScu834R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU834` writer - Enable Write Protection of hlinkSCU834"]
pub type EnblWrProtOfScu834W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU850` reader - Enable Write Protection of hlinkSCU850"]
pub type EnblWrProtOfScu850R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU850` writer - Enable Write Protection of hlinkSCU850"]
pub type EnblWrProtOfScu850W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU854` reader - Enable Write Protection of hlinkSCU854"]
pub type EnblWrProtOfScu854R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU854` writer - Enable Write Protection of hlinkSCU854"]
pub type EnblWrProtOfScu854W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU858` reader - Enable Write Protection of hlinkSCU858"]
pub type EnblWrProtOfScu858R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU858` writer - Enable Write Protection of hlinkSCU858"]
pub type EnblWrProtOfScu858W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU85C` reader - Enable Write Protection of hlinkSCU85C"]
pub type EnblWrProtOfScu85cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU85C` writer - Enable Write Protection of hlinkSCU85C"]
pub type EnblWrProtOfScu85cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU870` reader - Enable Write Protection of hlinkSCU870"]
pub type EnblWrProtOfScu870R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU870` writer - Enable Write Protection of hlinkSCU870"]
pub type EnblWrProtOfScu870W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU874` reader - Enable Write Protection of hlinkSCU874"]
pub type EnblWrProtOfScu874R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU874` writer - Enable Write Protection of hlinkSCU874"]
pub type EnblWrProtOfScu874W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU878` reader - Enable Write Protection of hlinkSCU878"]
pub type EnblWrProtOfScu878R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU878` writer - Enable Write Protection of hlinkSCU878"]
pub type EnblWrProtOfScu878W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU87C` reader - Enable Write Protection of hlinkSCU87C"]
pub type EnblWrProtOfScu87cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU87C` writer - Enable Write Protection of hlinkSCU87C"]
pub type EnblWrProtOfScu87cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu830(&self) -> EnblWrProtOfScu830R {
        EnblWrProtOfScu830R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu834(&self) -> EnblWrProtOfScu834R {
        EnblWrProtOfScu834R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu850(&self) -> EnblWrProtOfScu850R {
        EnblWrProtOfScu850R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu854(&self) -> EnblWrProtOfScu854R {
        EnblWrProtOfScu854R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu858(&self) -> EnblWrProtOfScu858R {
        EnblWrProtOfScu858R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Protection of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu85c(&self) -> EnblWrProtOfScu85cR {
        EnblWrProtOfScu85cR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu870(&self) -> EnblWrProtOfScu870R {
        EnblWrProtOfScu870R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Protection of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu874(&self) -> EnblWrProtOfScu874R {
        EnblWrProtOfScu874R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Write Protection of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu878(&self) -> EnblWrProtOfScu878R {
        EnblWrProtOfScu878R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu87c(&self) -> EnblWrProtOfScu87cR {
        EnblWrProtOfScu87cR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU830"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu830(&mut self) -> EnblWrProtOfScu830W<Scuf50Spec> {
        EnblWrProtOfScu830W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU834"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu834(&mut self) -> EnblWrProtOfScu834W<Scuf50Spec> {
        EnblWrProtOfScu834W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCU850"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu850(&mut self) -> EnblWrProtOfScu850W<Scuf50Spec> {
        EnblWrProtOfScu850W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCU854"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu854(&mut self) -> EnblWrProtOfScu854W<Scuf50Spec> {
        EnblWrProtOfScu854W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCU858"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu858(&mut self) -> EnblWrProtOfScu858W<Scuf50Spec> {
        EnblWrProtOfScu858W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Protection of hlinkSCU85C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu85c(&mut self) -> EnblWrProtOfScu85cW<Scuf50Spec> {
        EnblWrProtOfScu85cW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Protection of hlinkSCU870"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu870(&mut self) -> EnblWrProtOfScu870W<Scuf50Spec> {
        EnblWrProtOfScu870W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Protection of hlinkSCU874"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu874(&mut self) -> EnblWrProtOfScu874W<Scuf50Spec> {
        EnblWrProtOfScu874W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Write Protection of hlinkSCU878"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu878(&mut self) -> EnblWrProtOfScu878W<Scuf50Spec> {
        EnblWrProtOfScu878W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of hlinkSCU87C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu87c(&mut self) -> EnblWrProtOfScu87cW<Scuf50Spec> {
        EnblWrProtOfScu87cW::new(self, 15)
    }
}
#[doc = "Write Protect Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf50Spec;
impl crate::RegisterSpec for Scuf50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf50::R`](R) reader structure"]
impl crate::Readable for Scuf50Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf50::W`](W) writer structure"]
impl crate::Writable for Scuf50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF50 to value 0"]
impl crate::Resettable for Scuf50Spec {}
