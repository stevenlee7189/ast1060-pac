#[doc = "Register `TIMERG034` reader"]
pub type R = crate::R<Timerg034Spec>;
#[doc = "Register `TIMERG034` writer"]
pub type W = crate::W<Timerg034Spec>;
#[doc = "Field `Timer1INTEvent` reader - Timer1 Interrupt Event"]
pub type Timer1inteventR = crate::BitReader;
#[doc = "Field `Timer1INTEvent` writer - Timer1 Interrupt Event"]
pub type Timer1inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer2INTEvent` reader - Timer2 Interrupt Event"]
pub type Timer2inteventR = crate::BitReader;
#[doc = "Field `Timer2INTEvent` writer - Timer2 Interrupt Event"]
pub type Timer2inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer3INTEvent` reader - Timer3 Interrupt Event"]
pub type Timer3inteventR = crate::BitReader;
#[doc = "Field `Timer3INTEvent` writer - Timer3 Interrupt Event"]
pub type Timer3inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer4INTEvent` reader - Timer4 Interrupt Event"]
pub type Timer4inteventR = crate::BitReader;
#[doc = "Field `Timer4INTEvent` writer - Timer4 Interrupt Event"]
pub type Timer4inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer5INTEvent` reader - Timer5 Interrupt Event"]
pub type Timer5inteventR = crate::BitReader;
#[doc = "Field `Timer5INTEvent` writer - Timer5 Interrupt Event"]
pub type Timer5inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer6INTEvent` reader - Timer6 Interrupt Event"]
pub type Timer6inteventR = crate::BitReader;
#[doc = "Field `Timer6INTEvent` writer - Timer6 Interrupt Event"]
pub type Timer6inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer7INTEvent` reader - Timer7 Interrupt Event"]
pub type Timer7inteventR = crate::BitReader;
#[doc = "Field `Timer7INTEvent` writer - Timer7 Interrupt Event"]
pub type Timer7inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Timer8INTEvent` reader - Timer8 Interrupt Event"]
pub type Timer8inteventR = crate::BitReader;
#[doc = "Field `Timer8INTEvent` writer - Timer8 Interrupt Event"]
pub type Timer8inteventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Timer1 Interrupt Event"]
    #[inline(always)]
    pub fn timer1intevent(&self) -> Timer1inteventR {
        Timer1inteventR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer2 Interrupt Event"]
    #[inline(always)]
    pub fn timer2intevent(&self) -> Timer2inteventR {
        Timer2inteventR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer3 Interrupt Event"]
    #[inline(always)]
    pub fn timer3intevent(&self) -> Timer3inteventR {
        Timer3inteventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer4 Interrupt Event"]
    #[inline(always)]
    pub fn timer4intevent(&self) -> Timer4inteventR {
        Timer4inteventR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer5 Interrupt Event"]
    #[inline(always)]
    pub fn timer5intevent(&self) -> Timer5inteventR {
        Timer5inteventR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer6 Interrupt Event"]
    #[inline(always)]
    pub fn timer6intevent(&self) -> Timer6inteventR {
        Timer6inteventR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer7 Interrupt Event"]
    #[inline(always)]
    pub fn timer7intevent(&self) -> Timer7inteventR {
        Timer7inteventR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer8 Interrupt Event"]
    #[inline(always)]
    pub fn timer8intevent(&self) -> Timer8inteventR {
        Timer8inteventR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Timer1 Interrupt Event"]
    #[inline(always)]
    pub fn timer1intevent(&mut self) -> Timer1inteventW<Timerg034Spec> {
        Timer1inteventW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer2 Interrupt Event"]
    #[inline(always)]
    pub fn timer2intevent(&mut self) -> Timer2inteventW<Timerg034Spec> {
        Timer2inteventW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer3 Interrupt Event"]
    #[inline(always)]
    pub fn timer3intevent(&mut self) -> Timer3inteventW<Timerg034Spec> {
        Timer3inteventW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer4 Interrupt Event"]
    #[inline(always)]
    pub fn timer4intevent(&mut self) -> Timer4inteventW<Timerg034Spec> {
        Timer4inteventW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer5 Interrupt Event"]
    #[inline(always)]
    pub fn timer5intevent(&mut self) -> Timer5inteventW<Timerg034Spec> {
        Timer5inteventW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer6 Interrupt Event"]
    #[inline(always)]
    pub fn timer6intevent(&mut self) -> Timer6inteventW<Timerg034Spec> {
        Timer6inteventW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer7 Interrupt Event"]
    #[inline(always)]
    pub fn timer7intevent(&mut self) -> Timer7inteventW<Timerg034Spec> {
        Timer7inteventW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer8 Interrupt Event"]
    #[inline(always)]
    pub fn timer8intevent(&mut self) -> Timer8inteventW<Timerg034Spec> {
        Timer8inteventW::new(self, 7)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerg034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerg034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timerg034Spec;
impl crate::RegisterSpec for Timerg034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerg034::R`](R) reader structure"]
impl crate::Readable for Timerg034Spec {}
#[doc = "`write(|w| ..)` method takes [`timerg034::W`](W) writer structure"]
impl crate::Writable for Timerg034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERG034 to value 0"]
impl crate::Resettable for Timerg034Spec {}
