#[doc = "Register `FMC060` reader"]
pub type R = crate::R<Fmc060Spec>;
#[doc = "Register `FMC060` writer"]
pub type W = crate::W<Fmc060Spec>;
#[doc = "Field `EnblWatchdog` reader - Enable watchdog"]
pub type EnblWatchdogR = crate::BitReader;
#[doc = "Field `EnblWatchdog` writer - Enable watchdog"]
pub type EnblWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `WDActiveEventCounter` reader - Watchdog active event counter"]
pub type WdactiveEventCounterR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Enable watchdog"]
    #[inline(always)]
    pub fn enbl_watchdog(&self) -> EnblWatchdogR {
        EnblWatchdogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Watchdog active event counter"]
    #[inline(always)]
    pub fn wdactive_event_counter(&self) -> WdactiveEventCounterR {
        WdactiveEventCounterR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Enable watchdog"]
    #[inline(always)]
    pub fn enbl_watchdog(&mut self) -> EnblWatchdogW<Fmc060Spec> {
        EnblWatchdogW::new(self, 0)
    }
}
#[doc = "FMC\\_WDT1 Control/Status Register for Address Mode Detection\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc060Spec;
impl crate::RegisterSpec for Fmc060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc060::R`](R) reader structure"]
impl crate::Readable for Fmc060Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc060::W`](W) writer structure"]
impl crate::Writable for Fmc060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC060 to value 0"]
impl crate::Resettable for Fmc060Spec {}
