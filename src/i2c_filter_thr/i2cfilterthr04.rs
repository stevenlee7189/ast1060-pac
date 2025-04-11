#[doc = "Register `I2CFILTERTHR04` reader"]
pub type R = crate::R<I2cfilterthr04Spec>;
#[doc = "Register `I2CFILTERTHR04` writer"]
pub type W = crate::W<I2cfilterthr04Spec>;
#[doc = "Field `EN` reader - EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<I2cfilterthr04Spec> {
        EnW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THRN\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr04Spec;
impl crate::RegisterSpec for I2cfilterthr04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr04::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr04Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr04::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR04 to value 0"]
impl crate::Resettable for I2cfilterthr04Spec {}
