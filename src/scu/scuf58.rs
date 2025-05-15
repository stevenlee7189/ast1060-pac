#[doc = "Register `SCUF58` reader"]
pub type R = crate::R<Scuf58Spec>;
#[doc = "Register `SCUF58` writer"]
pub type W = crate::W<Scuf58Spec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA14` reader - Enable Write Protection of hlinkSCUA14"]
pub type EnblWrProtOfScua14R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA14` writer - Enable Write Protection of hlinkSCUA14"]
pub type EnblWrProtOfScua14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUA18` reader - Enable Write Protection of hlinkSCUA18"]
pub type EnblWrProtOfScua18R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA18` writer - Enable Write Protection of hlinkSCUA18"]
pub type EnblWrProtOfScua18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUA1C` reader - Enable Write Protection of hlinkSCUA1C"]
pub type EnblWrProtOfScua1cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA1C` writer - Enable Write Protection of hlinkSCUA1C"]
pub type EnblWrProtOfScua1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCUA50` reader - Enable Write Protection of hlinkSCUA50"]
pub type EnblWrProtOfScua50R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA50` writer - Enable Write Protection of hlinkSCUA50"]
pub type EnblWrProtOfScua50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUA54` reader - Enable Write Protection of hlinkSCUA54"]
pub type EnblWrProtOfScua54R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA54` writer - Enable Write Protection of hlinkSCUA54"]
pub type EnblWrProtOfScua54W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUA58` reader - Enable Write Protection of hlinkSCUA58"]
pub type EnblWrProtOfScua58R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUA58` writer - Enable Write Protection of hlinkSCUA58"]
pub type EnblWrProtOfScua58W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCUA14"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua14(&self) -> EnblWrProtOfScua14R {
        EnblWrProtOfScua14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCUA18"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua18(&self) -> EnblWrProtOfScua18R {
        EnblWrProtOfScua18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Protection of hlinkSCUA1C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua1c(&self) -> EnblWrProtOfScua1cR {
        EnblWrProtOfScua1cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCUA50"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua50(&self) -> EnblWrProtOfScua50R {
        EnblWrProtOfScua50R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCUA54"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua54(&self) -> EnblWrProtOfScua54R {
        EnblWrProtOfScua54R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCUA58"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua58(&self) -> EnblWrProtOfScua58R {
        EnblWrProtOfScua58R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCUA14"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua14(&mut self) -> EnblWrProtOfScua14W<Scuf58Spec> {
        EnblWrProtOfScua14W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Protection of hlinkSCUA18"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua18(&mut self) -> EnblWrProtOfScua18W<Scuf58Spec> {
        EnblWrProtOfScua18W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Protection of hlinkSCUA1C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua1c(&mut self) -> EnblWrProtOfScua1cW<Scuf58Spec> {
        EnblWrProtOfScua1cW::new(self, 3)
    }
    #[doc = "Bit 8 - Enable Write Protection of hlinkSCUA50"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua50(&mut self) -> EnblWrProtOfScua50W<Scuf58Spec> {
        EnblWrProtOfScua50W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Protection of hlinkSCUA54"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua54(&mut self) -> EnblWrProtOfScua54W<Scuf58Spec> {
        EnblWrProtOfScua54W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Protection of hlinkSCUA58"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scua58(&mut self) -> EnblWrProtOfScua58W<Scuf58Spec> {
        EnblWrProtOfScua58W::new(self, 10)
    }
}
#[doc = "Write Protect Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf58Spec;
impl crate::RegisterSpec for Scuf58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf58::R`](R) reader structure"]
impl crate::Readable for Scuf58Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf58::W`](W) writer structure"]
impl crate::Writable for Scuf58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF58 to value 0"]
impl crate::Resettable for Scuf58Spec {}
