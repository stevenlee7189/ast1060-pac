#[doc = "Register `SECURE054` reader"]
pub type R = crate::R<Secure054Spec>;
#[doc = "Register `SECURE054` writer"]
pub type W = crate::W<Secure054Spec>;
#[doc = "Field `ExtraReadProtRangeReg` reader - Extra Read Protection Range Register"]
pub type ExtraReadProtRangeRegR = crate::FieldReader;
#[doc = "Field `ExtraReadProtRangeReg` writer - Extra Read Protection Range Register"]
pub type ExtraReadProtRangeRegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WrProtOfThisRegSEC54` reader - Write Protection of this register SEC54"]
pub type WrProtOfThisRegSec54R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC54` writer - Write Protection of this register SEC54"]
pub type WrProtOfThisRegSec54W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Extra Read Protection Range Register"]
    #[inline(always)]
    pub fn extra_read_prot_range_reg(&self) -> ExtraReadProtRangeRegR {
        ExtraReadProtRangeRegR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Write Protection of this register SEC54"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec54(&self) -> WrProtOfThisRegSec54R {
        WrProtOfThisRegSec54R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Extra Read Protection Range Register"]
    #[inline(always)]
    pub fn extra_read_prot_range_reg(&mut self) -> ExtraReadProtRangeRegW<Secure054Spec> {
        ExtraReadProtRangeRegW::new(self, 0)
    }
    #[doc = "Bit 6 - Write Protection of this register SEC54"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec54(&mut self) -> WrProtOfThisRegSec54W<Secure054Spec> {
        WrProtOfThisRegSec54W::new(self, 6)
    }
}
#[doc = "Extra Read Protection Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure054Spec;
impl crate::RegisterSpec for Secure054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure054::R`](R) reader structure"]
impl crate::Readable for Secure054Spec {}
#[doc = "`write(|w| ..)` method takes [`secure054::W`](W) writer structure"]
impl crate::Writable for Secure054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE054 to value 0"]
impl crate::Resettable for Secure054Spec {}
