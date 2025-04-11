#[doc = "Register `I2CFILTERTHR10` reader"]
pub type R = crate::R<I2cfilterthr10Spec>;
#[doc = "Register `I2CFILTERTHR10` writer"]
pub type W = crate::W<I2cfilterthr10Spec>;
#[doc = "Field `TMRCFG` reader - TMR_CFG"]
pub type TmrcfgR = crate::FieldReader<u32>;
#[doc = "Field `TMRCFG` writer - TMR_CFG"]
pub type TmrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TMR_CFG"]
    #[inline(always)]
    pub fn tmrcfg(&self) -> TmrcfgR {
        TmrcfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TMR_CFG"]
    #[inline(always)]
    pub fn tmrcfg(&mut self) -> TmrcfgW<I2cfilterthr10Spec> {
        TmrcfgW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THRN\\_TMR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr10Spec;
impl crate::RegisterSpec for I2cfilterthr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr10::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr10Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr10::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR10 to value 0"]
impl crate::Resettable for I2cfilterthr10Spec {}
