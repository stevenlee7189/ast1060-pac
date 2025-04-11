#[doc = "Register `I2CFILTERTHR0C` reader"]
pub type R = crate::R<I2cfilterthr0cSpec>;
#[doc = "Register `I2CFILTERTHR0C` writer"]
pub type W = crate::W<I2cfilterthr0cSpec>;
#[doc = "Field `CFG` reader - CFG"]
pub type CfgR = crate::FieldReader<u32>;
#[doc = "Field `CFG` writer - CFG"]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CFG"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFG"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CfgW<I2cfilterthr0cSpec> {
        CfgW::new(self, 0)
    }
}
#[doc = "I2CFLT\\_THRN\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr0cSpec;
impl crate::RegisterSpec for I2cfilterthr0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr0c::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr0cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr0c::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR0C to value 0"]
impl crate::Resettable for I2cfilterthr0cSpec {}
