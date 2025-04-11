#[doc = "Register `SCU0D4` reader"]
pub type R = crate::R<Scu0d4Spec>;
#[doc = "Register `SCU0D4` writer"]
pub type W = crate::W<Scu0d4Spec>;
#[doc = "Field `Reserved1` reader - Reserved (0)"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - Reserved (0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GPIOINTSel` reader - GPIO Interrupt Selection"]
pub type GpiointselR = crate::FieldReader;
#[doc = "Field `GPIOINTSel` writer - GPIO Interrupt Selection"]
pub type GpiointselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPIOINTEnbl` reader - GPIO Interrupt Enable"]
pub type GpiointenblR = crate::BitReader;
#[doc = "Field `GPIOINTEnbl` writer - GPIO Interrupt Enable"]
pub type GpiointenblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - GPIO Interrupt Selection"]
    #[inline(always)]
    pub fn gpiointsel(&self) -> GpiointselR {
        GpiointselR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - GPIO Interrupt Enable"]
    #[inline(always)]
    pub fn gpiointenbl(&self) -> GpiointenblR {
        GpiointenblR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu0d4Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - GPIO Interrupt Selection"]
    #[inline(always)]
    pub fn gpiointsel(&mut self) -> GpiointselW<Scu0d4Spec> {
        GpiointselW::new(self, 16)
    }
    #[doc = "Bit 21 - GPIO Interrupt Enable"]
    #[inline(always)]
    pub fn gpiointenbl(&mut self) -> GpiointenblW<Scu0d4Spec> {
        GpiointenblW::new(self, 21)
    }
}
#[doc = "Misc. 4 Control Register\\label{SCUREG:MISC4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0d4Spec;
impl crate::RegisterSpec for Scu0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0d4::R`](R) reader structure"]
impl crate::Readable for Scu0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0d4::W`](W) writer structure"]
impl crate::Writable for Scu0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0D4 to value 0x0300_2103"]
impl crate::Resettable for Scu0d4Spec {
    const RESET_VALUE: u32 = 0x0300_2103;
}
