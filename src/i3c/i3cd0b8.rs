#[doc = "Register `I3CD0B8` reader"]
pub type R = crate::R<I3cd0b8Spec>;
#[doc = "Register `I3CD0B8` writer"]
pub type W = crate::W<I3cd0b8Spec>;
#[doc = "Field `I3CPPLCNT` reader - I3C_PP_LCNT"]
pub type I3cpplcntR = crate::FieldReader;
#[doc = "Field `I3CPPLCNT` writer - I3C_PP_LCNT"]
pub type I3cpplcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD158` reader - These bits in SCL I3C Push Pull Timing Register are reserved."]
pub type Rsvd158R = crate::FieldReader;
#[doc = "Field `I3CPPHCNT` reader - I3C_PP_HCNT"]
pub type I3cpphcntR = crate::FieldReader;
#[doc = "Field `I3CPPHCNT` writer - I3C_PP_HCNT"]
pub type I3cpphcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD3124` reader - These bits in SCL I3C Push Pull Timing Register are reserved."]
pub type Rsvd3124R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - I3C_PP_LCNT"]
    #[inline(always)]
    pub fn i3cpplcnt(&self) -> I3cpplcntR {
        I3cpplcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits in SCL I3C Push Pull Timing Register are reserved."]
    #[inline(always)]
    pub fn rsvd158(&self) -> Rsvd158R {
        Rsvd158R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I3C_PP_HCNT"]
    #[inline(always)]
    pub fn i3cpphcnt(&self) -> I3cpphcntR {
        I3cpphcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - These bits in SCL I3C Push Pull Timing Register are reserved."]
    #[inline(always)]
    pub fn rsvd3124(&self) -> Rsvd3124R {
        Rsvd3124R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I3C_PP_LCNT"]
    #[inline(always)]
    pub fn i3cpplcnt(&mut self) -> I3cpplcntW<I3cd0b8Spec> {
        I3cpplcntW::new(self, 0)
    }
    #[doc = "Bits 16:23 - I3C_PP_HCNT"]
    #[inline(always)]
    pub fn i3cpphcnt(&mut self) -> I3cpphcntW<I3cd0b8Spec> {
        I3cpphcntW::new(self, 16)
    }
}
#[doc = "SCL I3C Push Pull Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0b8Spec;
impl crate::RegisterSpec for I3cd0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0b8::R`](R) reader structure"]
impl crate::Readable for I3cd0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0b8::W`](W) writer structure"]
impl crate::Writable for I3cd0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0B8 to value 0"]
impl crate::Resettable for I3cd0b8Spec {}
