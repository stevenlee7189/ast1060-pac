#[doc = "Register `SCUF6C` reader"]
pub type R = crate::R<Scuf6cSpec>;
#[doc = "Register `SCUF6C` writer"]
pub type W = crate::W<Scuf6cSpec>;
#[doc = "Field `EnblWrProtOfSCUF80` reader - Enable Write Protection of hlinkSCUF80"]
pub type EnblWrProtOfScuf80R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF80` writer - Enable Write Protection of hlinkSCUF80"]
pub type EnblWrProtOfScuf80W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF88` reader - Enable Write Protection of hlinkSCUF88"]
pub type EnblWrProtOfScuf88R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF88` writer - Enable Write Protection of hlinkSCUF88"]
pub type EnblWrProtOfScuf88W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFEC` reader - Enable Write Protection of hlinkSCUFEC"]
pub type EnblWrProtOfScufecR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFEC` writer - Enable Write Protection of hlinkSCUFEC"]
pub type EnblWrProtOfScufecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF80"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf80(&self) -> EnblWrProtOfScuf80R {
        EnblWrProtOfScuf80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF88"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf88(&self) -> EnblWrProtOfScuf88R {
        EnblWrProtOfScuf88R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFEC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufec(&self) -> EnblWrProtOfScufecR {
        EnblWrProtOfScufecR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF80"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf80(&mut self) -> EnblWrProtOfScuf80W<Scuf6cSpec> {
        EnblWrProtOfScuf80W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF88"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf88(&mut self) -> EnblWrProtOfScuf88W<Scuf6cSpec> {
        EnblWrProtOfScuf88W::new(self, 18)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFEC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufec(&mut self) -> EnblWrProtOfScufecW<Scuf6cSpec> {
        EnblWrProtOfScufecW::new(self, 31)
    }
}
#[doc = "Write Protect Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf6cSpec;
impl crate::RegisterSpec for Scuf6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf6c::R`](R) reader structure"]
impl crate::Readable for Scuf6cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf6c::W`](W) writer structure"]
impl crate::Writable for Scuf6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF6C to value 0"]
impl crate::Resettable for Scuf6cSpec {}
