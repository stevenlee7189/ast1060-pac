#[doc = "Register `WDT040` reader"]
pub type R = crate::R<Wdt040Spec>;
#[doc = "Register `WDT040` writer"]
pub type W = crate::W<Wdt040Spec>;
#[doc = "Field `EnblWrProtOfWDT1Cn` reader - Enable Write Protection of WDT1C\\[n\\]"]
pub type EnblWrProtOfWdt1cnR = crate::FieldReader<u32>;
#[doc = "Field `EnblWrProtOfWDT1Cn` writer - Enable Write Protection of WDT1C\\[n\\]"]
pub type EnblWrProtOfWdt1cnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT1C\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt1cn(&self) -> EnblWrProtOfWdt1cnR {
        EnblWrProtOfWdt1cnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT1C\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt1cn(&mut self) -> EnblWrProtOfWdt1cnW<Wdt040Spec> {
        EnblWrProtOfWdt1cnW::new(self, 0)
    }
}
#[doc = "WDTn Reset Mask Write Protection Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt040Spec;
impl crate::RegisterSpec for Wdt040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt040::R`](R) reader structure"]
impl crate::Readable for Wdt040Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt040::W`](W) writer structure"]
impl crate::Writable for Wdt040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT040 to value 0"]
impl crate::Resettable for Wdt040Spec {}
