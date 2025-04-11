#[doc = "Register `I2CFILTERTHR44` reader"]
pub type R = crate::R<I2cfilterthr44Spec>;
#[doc = "Register `I2CFILTERTHR44` writer"]
pub type W = crate::W<I2cfilterthr44Spec>;
#[doc = "Field `MAP1` reader - MAP1"]
pub type Map1R = crate::FieldReader<u32>;
#[doc = "Field `MAP1` writer - MAP1"]
pub type Map1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAP1"]
    #[inline(always)]
    pub fn map1(&self) -> Map1R {
        Map1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAP1"]
    #[inline(always)]
    pub fn map1(&mut self) -> Map1W<I2cfilterthr44Spec> {
        Map1W::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THRN\\_MAP1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr44Spec;
impl crate::RegisterSpec for I2cfilterthr44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr44::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr44Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr44::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR44 to value 0"]
impl crate::Resettable for I2cfilterthr44Spec {}
