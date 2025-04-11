#[doc = "Register `UDMA1A4` reader"]
pub type R = crate::R<Udma1a4Spec>;
#[doc = "Register `UDMA1A4` writer"]
pub type W = crate::W<Udma1a4Spec>;
#[doc = "Field `UART12TXWrPointer` reader - UART12 TX write pointer"]
pub type Uart12txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART12TXWrPointer` writer - UART12 TX write pointer"]
pub type Uart12txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART12 TX write pointer"]
    #[inline(always)]
    pub fn uart12txwr_pointer(&self) -> Uart12txwrPointerR {
        Uart12txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART12 TX write pointer"]
    #[inline(always)]
    pub fn uart12txwr_pointer(&mut self) -> Uart12txwrPointerW<Udma1a4Spec> {
        Uart12txwrPointerW::new(self, 0)
    }
}
#[doc = "UART12 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1a4Spec;
impl crate::RegisterSpec for Udma1a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1a4::R`](R) reader structure"]
impl crate::Readable for Udma1a4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1a4::W`](W) writer structure"]
impl crate::Writable for Udma1a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1A4 to value 0"]
impl crate::Resettable for Udma1a4Spec {}
