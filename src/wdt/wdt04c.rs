#[doc = "Register `WDT04C` reader"]
pub type R = crate::R<Wdt04cSpec>;
#[doc = "Register `WDT04C` writer"]
pub type W = crate::W<Wdt04cSpec>;
#[doc = "Field `EnblWrProtOfWDT2Cn` reader - Enable Write Protection of WDT2C\\[n\\]"]
pub type EnblWrProtOfWdt2cnR = crate::FieldReader<u32>;
#[doc = "Field `EnblWrProtOfWDT2Cn` writer - Enable Write Protection of WDT2C\\[n\\]"]
pub type EnblWrProtOfWdt2cnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT2C\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt2cn(&self) -> EnblWrProtOfWdt2cnR {
        EnblWrProtOfWdt2cnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Write Protection of WDT2C\\[n\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_wdt2cn(&mut self) -> EnblWrProtOfWdt2cnW<Wdt04cSpec> {
        EnblWrProtOfWdt2cnW::new(self, 0)
    }
}
#[doc = "WDTn Software Mode Reset Mask Write Protection Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt04cSpec;
impl crate::RegisterSpec for Wdt04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt04c::R`](R) reader structure"]
impl crate::Readable for Wdt04cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt04c::W`](W) writer structure"]
impl crate::Writable for Wdt04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT04C to value 0"]
impl crate::Resettable for Wdt04cSpec {}
