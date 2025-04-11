#[doc = "Register `GPIO08C` reader"]
pub type R = crate::R<Gpio08cSpec>;
#[doc = "Register `GPIO08C` writer"]
pub type W = crate::W<Gpio08cSpec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "GPIO\\_U Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio08cSpec;
impl crate::RegisterSpec for Gpio08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio08c::R`](R) reader structure"]
impl crate::Readable for Gpio08cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio08c::W`](W) writer structure"]
impl crate::Writable for Gpio08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO08C to value 0"]
impl crate::Resettable for Gpio08cSpec {}
