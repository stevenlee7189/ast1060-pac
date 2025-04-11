#[doc = "Register `SCU4B0` reader"]
pub type R = crate::R<Scu4b0Spec>;
#[doc = "Register `SCU4B0` writer"]
pub type W = crate::W<Scu4b0Spec>;
#[doc = "Field `EnblSALT1FnPin` reader - Enable SALT1 function pin"]
pub type EnblSalt1fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT1FnPin` writer - Enable SALT1 function pin"]
pub type EnblSalt1fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT2FnPin` reader - Enable SALT2 function pin"]
pub type EnblSalt2fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT2FnPin` writer - Enable SALT2 function pin"]
pub type EnblSalt2fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT3FnPin` reader - Enable SALT3 function pin"]
pub type EnblSalt3fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT3FnPin` writer - Enable SALT3 function pin"]
pub type EnblSalt3fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT4FnPin` reader - Enable SALT4 function pin"]
pub type EnblSalt4fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT4FnPin` writer - Enable SALT4 function pin"]
pub type EnblSalt4fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT5FnPin` reader - Enable SALT5 function pin"]
pub type EnblSalt5fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT5FnPin` writer - Enable SALT5 function pin"]
pub type EnblSalt5fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT6FnPin` reader - Enable SALT6 function pin"]
pub type EnblSalt6fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT6FnPin` writer - Enable SALT6 function pin"]
pub type EnblSalt6fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT7FnPin` reader - Enable SALT7 function pin"]
pub type EnblSalt7fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT7FnPin` writer - Enable SALT7 function pin"]
pub type EnblSalt7fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT8FnPin` reader - Enable SALT8 function pin"]
pub type EnblSalt8fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT8FnPin` writer - Enable SALT8 function pin"]
pub type EnblSalt8fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT9FnPin` reader - Enable SALT9 function pin"]
pub type EnblSalt9fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT9FnPin` writer - Enable SALT9 function pin"]
pub type EnblSalt9fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSALT10FnPin` reader - Enable SALT10 function pin"]
pub type EnblSalt10fnPinR = crate::BitReader;
#[doc = "Field `EnblSALT10FnPin` writer - Enable SALT10 function pin"]
pub type EnblSalt10fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved, must keep at value 0x0"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved, must keep at value 0x0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EnblGPIOPassthrough0GPIOC0FnPin` reader - Enable GPIO Passthrough 0 (GPIOC0) function pin"]
pub type EnblGpiopassthrough0gpioc0fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough0GPIOC0FnPin` writer - Enable GPIO Passthrough 0 (GPIOC0) function pin"]
pub type EnblGpiopassthrough0gpioc0fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough0GPIOC1FnPin` reader - Enable GPIO Passthrough 0 (GPIOC1) function pin"]
pub type EnblGpiopassthrough0gpioc1fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough0GPIOC1FnPin` writer - Enable GPIO Passthrough 0 (GPIOC1) function pin"]
pub type EnblGpiopassthrough0gpioc1fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough1GPIOC2FnPin` reader - Enable GPIO Passthrough 1 (GPIOC2) function pin"]
pub type EnblGpiopassthrough1gpioc2fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough1GPIOC2FnPin` writer - Enable GPIO Passthrough 1 (GPIOC2) function pin"]
pub type EnblGpiopassthrough1gpioc2fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough1GPIOC3FnPin` reader - Enable GPIO Passthrough 1 (GPIOC3) function pin"]
pub type EnblGpiopassthrough1gpioc3fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough1GPIOC3FnPin` writer - Enable GPIO Passthrough 1 (GPIOC3) function pin"]
pub type EnblGpiopassthrough1gpioc3fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough2GPIOC4FnPin` reader - Enable GPIO Passthrough 2 (GPIOC4) function pin"]
pub type EnblGpiopassthrough2gpioc4fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough2GPIOC4FnPin` writer - Enable GPIO Passthrough 2 (GPIOC4) function pin"]
pub type EnblGpiopassthrough2gpioc4fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough2GPIOC5FnPin` reader - Enable GPIO Passthrough 2 (GPIOC5) function pin"]
pub type EnblGpiopassthrough2gpioc5fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough2GPIOC5FnPin` writer - Enable GPIO Passthrough 2 (GPIOC5) function pin"]
pub type EnblGpiopassthrough2gpioc5fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough3GPIOC6FnPin` reader - Enable GPIO Passthrough 3 (GPIOC6) function pin"]
pub type EnblGpiopassthrough3gpioc6fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough3GPIOC6FnPin` writer - Enable GPIO Passthrough 3 (GPIOC6) function pin"]
pub type EnblGpiopassthrough3gpioc6fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblGPIOPassthrough3GPIOC7FnPin` reader - Enable GPIO Passthrough 3 (GPIOC7) function pin"]
pub type EnblGpiopassthrough3gpioc7fnPinR = crate::BitReader;
#[doc = "Field `EnblGPIOPassthrough3GPIOC7FnPin` writer - Enable GPIO Passthrough 3 (GPIOC7) function pin"]
pub type EnblGpiopassthrough3gpioc7fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable SALT1 function pin"]
    #[inline(always)]
    pub fn enbl_salt1fn_pin(&self) -> EnblSalt1fnPinR {
        EnblSalt1fnPinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable SALT2 function pin"]
    #[inline(always)]
    pub fn enbl_salt2fn_pin(&self) -> EnblSalt2fnPinR {
        EnblSalt2fnPinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable SALT3 function pin"]
    #[inline(always)]
    pub fn enbl_salt3fn_pin(&self) -> EnblSalt3fnPinR {
        EnblSalt3fnPinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable SALT4 function pin"]
    #[inline(always)]
    pub fn enbl_salt4fn_pin(&self) -> EnblSalt4fnPinR {
        EnblSalt4fnPinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable SALT5 function pin"]
    #[inline(always)]
    pub fn enbl_salt5fn_pin(&self) -> EnblSalt5fnPinR {
        EnblSalt5fnPinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable SALT6 function pin"]
    #[inline(always)]
    pub fn enbl_salt6fn_pin(&self) -> EnblSalt6fnPinR {
        EnblSalt6fnPinR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable SALT7 function pin"]
    #[inline(always)]
    pub fn enbl_salt7fn_pin(&self) -> EnblSalt7fnPinR {
        EnblSalt7fnPinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable SALT8 function pin"]
    #[inline(always)]
    pub fn enbl_salt8fn_pin(&self) -> EnblSalt8fnPinR {
        EnblSalt8fnPinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable SALT9 function pin"]
    #[inline(always)]
    pub fn enbl_salt9fn_pin(&self) -> EnblSalt9fnPinR {
        EnblSalt9fnPinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable SALT10 function pin"]
    #[inline(always)]
    pub fn enbl_salt10fn_pin(&self) -> EnblSalt10fnPinR {
        EnblSalt10fnPinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Reserved, must keep at value 0x0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Enable GPIO Passthrough 0 (GPIOC0) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough0gpioc0fn_pin(&self) -> EnblGpiopassthrough0gpioc0fnPinR {
        EnblGpiopassthrough0gpioc0fnPinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable GPIO Passthrough 0 (GPIOC1) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough0gpioc1fn_pin(&self) -> EnblGpiopassthrough0gpioc1fnPinR {
        EnblGpiopassthrough0gpioc1fnPinR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable GPIO Passthrough 1 (GPIOC2) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough1gpioc2fn_pin(&self) -> EnblGpiopassthrough1gpioc2fnPinR {
        EnblGpiopassthrough1gpioc2fnPinR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable GPIO Passthrough 1 (GPIOC3) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough1gpioc3fn_pin(&self) -> EnblGpiopassthrough1gpioc3fnPinR {
        EnblGpiopassthrough1gpioc3fnPinR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable GPIO Passthrough 2 (GPIOC4) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough2gpioc4fn_pin(&self) -> EnblGpiopassthrough2gpioc4fnPinR {
        EnblGpiopassthrough2gpioc4fnPinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable GPIO Passthrough 2 (GPIOC5) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough2gpioc5fn_pin(&self) -> EnblGpiopassthrough2gpioc5fnPinR {
        EnblGpiopassthrough2gpioc5fnPinR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable GPIO Passthrough 3 (GPIOC6) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough3gpioc6fn_pin(&self) -> EnblGpiopassthrough3gpioc6fnPinR {
        EnblGpiopassthrough3gpioc6fnPinR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable GPIO Passthrough 3 (GPIOC7) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough3gpioc7fn_pin(&self) -> EnblGpiopassthrough3gpioc7fnPinR {
        EnblGpiopassthrough3gpioc7fnPinR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SALT1 function pin"]
    #[inline(always)]
    pub fn enbl_salt1fn_pin(&mut self) -> EnblSalt1fnPinW<Scu4b0Spec> {
        EnblSalt1fnPinW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable SALT2 function pin"]
    #[inline(always)]
    pub fn enbl_salt2fn_pin(&mut self) -> EnblSalt2fnPinW<Scu4b0Spec> {
        EnblSalt2fnPinW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable SALT3 function pin"]
    #[inline(always)]
    pub fn enbl_salt3fn_pin(&mut self) -> EnblSalt3fnPinW<Scu4b0Spec> {
        EnblSalt3fnPinW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable SALT4 function pin"]
    #[inline(always)]
    pub fn enbl_salt4fn_pin(&mut self) -> EnblSalt4fnPinW<Scu4b0Spec> {
        EnblSalt4fnPinW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable SALT5 function pin"]
    #[inline(always)]
    pub fn enbl_salt5fn_pin(&mut self) -> EnblSalt5fnPinW<Scu4b0Spec> {
        EnblSalt5fnPinW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable SALT6 function pin"]
    #[inline(always)]
    pub fn enbl_salt6fn_pin(&mut self) -> EnblSalt6fnPinW<Scu4b0Spec> {
        EnblSalt6fnPinW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable SALT7 function pin"]
    #[inline(always)]
    pub fn enbl_salt7fn_pin(&mut self) -> EnblSalt7fnPinW<Scu4b0Spec> {
        EnblSalt7fnPinW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable SALT8 function pin"]
    #[inline(always)]
    pub fn enbl_salt8fn_pin(&mut self) -> EnblSalt8fnPinW<Scu4b0Spec> {
        EnblSalt8fnPinW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable SALT9 function pin"]
    #[inline(always)]
    pub fn enbl_salt9fn_pin(&mut self) -> EnblSalt9fnPinW<Scu4b0Spec> {
        EnblSalt9fnPinW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable SALT10 function pin"]
    #[inline(always)]
    pub fn enbl_salt10fn_pin(&mut self) -> EnblSalt10fnPinW<Scu4b0Spec> {
        EnblSalt10fnPinW::new(self, 9)
    }
    #[doc = "Bits 10:15 - Reserved, must keep at value 0x0"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu4b0Spec> {
        Reserved1W::new(self, 10)
    }
    #[doc = "Bit 16 - Enable GPIO Passthrough 0 (GPIOC0) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough0gpioc0fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough0gpioc0fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough0gpioc0fnPinW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable GPIO Passthrough 0 (GPIOC1) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough0gpioc1fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough0gpioc1fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough0gpioc1fnPinW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable GPIO Passthrough 1 (GPIOC2) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough1gpioc2fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough1gpioc2fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough1gpioc2fnPinW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable GPIO Passthrough 1 (GPIOC3) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough1gpioc3fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough1gpioc3fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough1gpioc3fnPinW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable GPIO Passthrough 2 (GPIOC4) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough2gpioc4fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough2gpioc4fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough2gpioc4fnPinW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable GPIO Passthrough 2 (GPIOC5) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough2gpioc5fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough2gpioc5fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough2gpioc5fnPinW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable GPIO Passthrough 3 (GPIOC6) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough3gpioc6fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough3gpioc6fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough3gpioc6fnPinW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable GPIO Passthrough 3 (GPIOC7) function pin"]
    #[inline(always)]
    pub fn enbl_gpiopassthrough3gpioc7fn_pin(
        &mut self,
    ) -> EnblGpiopassthrough3gpioc7fnPinW<Scu4b0Spec> {
        EnblGpiopassthrough3gpioc7fnPinW::new(self, 23)
    }
}
#[doc = "Multi-function Pin Control \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4b0Spec;
impl crate::RegisterSpec for Scu4b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4b0::R`](R) reader structure"]
impl crate::Readable for Scu4b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4b0::W`](W) writer structure"]
impl crate::Writable for Scu4b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4B0 to value 0"]
impl crate::Resettable for Scu4b0Spec {}
