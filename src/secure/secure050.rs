#[doc = "Register `SECURE050` reader"]
pub type R = crate::R<Secure050Spec>;
#[doc = "Register `SECURE050` writer"]
pub type W = crate::W<Secure050Spec>;
#[doc = "Field `ExtraProgrammingProtRangeReg` reader - Extra Programming Protection Range Register"]
pub type ExtraProgrammingProtRangeRegR = crate::FieldReader;
#[doc = "Field `ExtraProgrammingProtRangeReg` writer - Extra Programming Protection Range Register"]
pub type ExtraProgrammingProtRangeRegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `WrProtOfThisRegSEC50` reader - Write Protection of this register SEC50"]
pub type WrProtOfThisRegSec50R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC50` writer - Write Protection of this register SEC50"]
pub type WrProtOfThisRegSec50W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Extra Programming Protection Range Register"]
    #[inline(always)]
    pub fn extra_programming_prot_range_reg(&self) -> ExtraProgrammingProtRangeRegR {
        ExtraProgrammingProtRangeRegR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
    #[doc = "Bit 6 - Write Protection of this register SEC50"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec50(&self) -> WrProtOfThisRegSec50R {
        WrProtOfThisRegSec50R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Extra Programming Protection Range Register"]
    #[inline(always)]
    pub fn extra_programming_prot_range_reg(
        &mut self,
    ) -> ExtraProgrammingProtRangeRegW<Secure050Spec> {
        ExtraProgrammingProtRangeRegW::new(self, 0)
    }
    #[doc = "Bit 6 - Write Protection of this register SEC50"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec50(&mut self) -> WrProtOfThisRegSec50W<Secure050Spec> {
        WrProtOfThisRegSec50W::new(self, 6)
    }
}
#[doc = "Extra Programming Protection Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure050Spec;
impl crate::RegisterSpec for Secure050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure050::R`](R) reader structure"]
impl crate::Readable for Secure050Spec {}
#[doc = "`write(|w| ..)` method takes [`secure050::W`](W) writer structure"]
impl crate::Writable for Secure050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE050 to value 0"]
impl crate::Resettable for Secure050Spec {}
