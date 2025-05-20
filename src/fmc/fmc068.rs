#[doc = "Register `FMC068` reader"]
pub type R = crate::R<Fmc068Spec>;
#[doc = "Register `FMC068` writer"]
pub type W = crate::W<Fmc068Spec>;
#[doc = "Field `ReloadValueOfExpireTime` reader - Reload value of expire time"]
pub type ReloadValueOfExpireTimeR = crate::FieldReader<u16>;
#[doc = "Field `ReloadValueOfExpireTime` writer - Reload value of expire time"]
pub type ReloadValueOfExpireTimeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `CounterValueSts` reader - Counter value status"]
pub type CounterValueStsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Reload value of expire time"]
    #[inline(always)]
    pub fn reload_value_of_expire_time(&self) -> ReloadValueOfExpireTimeR {
        ReloadValueOfExpireTimeR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Counter value status"]
    #[inline(always)]
    pub fn counter_value_sts(&self) -> CounterValueStsR {
        CounterValueStsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Reload value of expire time"]
    #[inline(always)]
    pub fn reload_value_of_expire_time(&mut self) -> ReloadValueOfExpireTimeW<Fmc068Spec> {
        ReloadValueOfExpireTimeW::new(self, 0)
    }
}
#[doc = "FMC\\_WDT2 Timer Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc068Spec;
impl crate::RegisterSpec for Fmc068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc068::R`](R) reader structure"]
impl crate::Readable for Fmc068Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc068::W`](W) writer structure"]
impl crate::Writable for Fmc068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC068 to value 0xe0"]
impl crate::Resettable for Fmc068Spec {
    const RESET_VALUE: u32 = 0xe0;
}
