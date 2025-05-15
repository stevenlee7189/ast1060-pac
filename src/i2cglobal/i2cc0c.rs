#[doc = "Register `I2CC0C` reader"]
pub type R = crate::R<I2cc0cSpec>;
#[doc = "Register `I2CC0C` writer"]
pub type W = crate::W<I2cc0cSpec>;
#[doc = "Field `SameAsI2CD1C` reader - Same as hlinkI2CD1C"]
pub type SameAsI2cd1cR = crate::FieldReader<u32>;
#[doc = "Field `SameAsI2CD1C` writer - Same as hlinkI2CD1C"]
pub type SameAsI2cd1cW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Same as hlinkI2CD1C"]
    #[inline(always)]
    pub fn same_as_i2cd1c(&self) -> SameAsI2cd1cR {
        SameAsI2cd1cR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Same as hlinkI2CD1C"]
    #[inline(always)]
    pub fn same_as_i2cd1c(&mut self) -> SameAsI2cd1cW<I2cc0cSpec> {
        SameAsI2cd1cW::new(self, 0)
    }
}
#[doc = "Master/Slave Pool Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc0cSpec;
impl crate::RegisterSpec for I2cc0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc0c::R`](R) reader structure"]
impl crate::Readable for I2cc0cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cc0c::W`](W) writer structure"]
impl crate::Writable for I2cc0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC0C to value 0"]
impl crate::Resettable for I2cc0cSpec {}
