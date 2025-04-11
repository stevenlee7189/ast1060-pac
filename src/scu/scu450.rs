#[doc = "Register `SCU450` reader"]
pub type R = crate::R<Scu450Spec>;
#[doc = "Register `SCU450` writer"]
pub type W = crate::W<Scu450Spec>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EnblI2C1` reader - Enable I2C1"]
pub type EnblI2c1R = crate::BitReader;
#[doc = "Field `EnblI2C1` writer - Enable I2C1"]
pub type EnblI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C2` reader - Enable I2C2"]
pub type EnblI2c2R = crate::BitReader;
#[doc = "Field `EnblI2C2` writer - Enable I2C2"]
pub type EnblI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C3` reader - Enable I2C3"]
pub type EnblI2c3R = crate::BitReader;
#[doc = "Field `EnblI2C3` writer - Enable I2C3"]
pub type EnblI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C4` reader - Enable I2C4"]
pub type EnblI2c4R = crate::BitReader;
#[doc = "Field `EnblI2C4` writer - Enable I2C4"]
pub type EnblI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C5` reader - Enable I2C5"]
pub type EnblI2c5R = crate::BitReader;
#[doc = "Field `EnblI2C5` writer - Enable I2C5"]
pub type EnblI2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C6` reader - Enable I2C6"]
pub type EnblI2c6R = crate::BitReader;
#[doc = "Field `EnblI2C6` writer - Enable I2C6"]
pub type EnblI2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C7` reader - Enable I2C7"]
pub type EnblI2c7R = crate::BitReader;
#[doc = "Field `EnblI2C7` writer - Enable I2C7"]
pub type EnblI2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C8` reader - Enable I2C8"]
pub type EnblI2c8R = crate::BitReader;
#[doc = "Field `EnblI2C8` writer - Enable I2C8"]
pub type EnblI2c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C9` reader - Enable I2C9"]
pub type EnblI2c9R = crate::BitReader;
#[doc = "Field `EnblI2C9` writer - Enable I2C9"]
pub type EnblI2c9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C10` reader - Enable I2C10"]
pub type EnblI2c10R = crate::BitReader;
#[doc = "Field `EnblI2C10` writer - Enable I2C10"]
pub type EnblI2c10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C11` reader - Enable I2C11"]
pub type EnblI2c11R = crate::BitReader;
#[doc = "Field `EnblI2C11` writer - Enable I2C11"]
pub type EnblI2c11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C12` reader - Enable I2C12"]
pub type EnblI2c12R = crate::BitReader;
#[doc = "Field `EnblI2C12` writer - Enable I2C12"]
pub type EnblI2c12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C13` reader - Enable I2C13"]
pub type EnblI2c13R = crate::BitReader;
#[doc = "Field `EnblI2C13` writer - Enable I2C13"]
pub type EnblI2c13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI2C14` reader - Enable I2C14"]
pub type EnblI2c14R = crate::BitReader;
#[doc = "Field `EnblI2C14` writer - Enable I2C14"]
pub type EnblI2c14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Enable I2C1"]
    #[inline(always)]
    pub fn enbl_i2c1(&self) -> EnblI2c1R {
        EnblI2c1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable I2C2"]
    #[inline(always)]
    pub fn enbl_i2c2(&self) -> EnblI2c2R {
        EnblI2c2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable I2C3"]
    #[inline(always)]
    pub fn enbl_i2c3(&self) -> EnblI2c3R {
        EnblI2c3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable I2C4"]
    #[inline(always)]
    pub fn enbl_i2c4(&self) -> EnblI2c4R {
        EnblI2c4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable I2C5"]
    #[inline(always)]
    pub fn enbl_i2c5(&self) -> EnblI2c5R {
        EnblI2c5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable I2C6"]
    #[inline(always)]
    pub fn enbl_i2c6(&self) -> EnblI2c6R {
        EnblI2c6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable I2C7"]
    #[inline(always)]
    pub fn enbl_i2c7(&self) -> EnblI2c7R {
        EnblI2c7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable I2C8"]
    #[inline(always)]
    pub fn enbl_i2c8(&self) -> EnblI2c8R {
        EnblI2c8R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable I2C9"]
    #[inline(always)]
    pub fn enbl_i2c9(&self) -> EnblI2c9R {
        EnblI2c9R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable I2C10"]
    #[inline(always)]
    pub fn enbl_i2c10(&self) -> EnblI2c10R {
        EnblI2c10R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable I2C11"]
    #[inline(always)]
    pub fn enbl_i2c11(&self) -> EnblI2c11R {
        EnblI2c11R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable I2C12"]
    #[inline(always)]
    pub fn enbl_i2c12(&self) -> EnblI2c12R {
        EnblI2c12R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable I2C13"]
    #[inline(always)]
    pub fn enbl_i2c13(&self) -> EnblI2c13R {
        EnblI2c13R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable I2C14"]
    #[inline(always)]
    pub fn enbl_i2c14(&self) -> EnblI2c14R {
        EnblI2c14R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Scu450Spec> {
        Reserved6W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scu450Spec> {
        Reserved5W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu450Spec> {
        Reserved4W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu450Spec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu450Spec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bit 16 - Enable I2C1"]
    #[inline(always)]
    pub fn enbl_i2c1(&mut self) -> EnblI2c1W<Scu450Spec> {
        EnblI2c1W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable I2C2"]
    #[inline(always)]
    pub fn enbl_i2c2(&mut self) -> EnblI2c2W<Scu450Spec> {
        EnblI2c2W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable I2C3"]
    #[inline(always)]
    pub fn enbl_i2c3(&mut self) -> EnblI2c3W<Scu450Spec> {
        EnblI2c3W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable I2C4"]
    #[inline(always)]
    pub fn enbl_i2c4(&mut self) -> EnblI2c4W<Scu450Spec> {
        EnblI2c4W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable I2C5"]
    #[inline(always)]
    pub fn enbl_i2c5(&mut self) -> EnblI2c5W<Scu450Spec> {
        EnblI2c5W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable I2C6"]
    #[inline(always)]
    pub fn enbl_i2c6(&mut self) -> EnblI2c6W<Scu450Spec> {
        EnblI2c6W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable I2C7"]
    #[inline(always)]
    pub fn enbl_i2c7(&mut self) -> EnblI2c7W<Scu450Spec> {
        EnblI2c7W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable I2C8"]
    #[inline(always)]
    pub fn enbl_i2c8(&mut self) -> EnblI2c8W<Scu450Spec> {
        EnblI2c8W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable I2C9"]
    #[inline(always)]
    pub fn enbl_i2c9(&mut self) -> EnblI2c9W<Scu450Spec> {
        EnblI2c9W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable I2C10"]
    #[inline(always)]
    pub fn enbl_i2c10(&mut self) -> EnblI2c10W<Scu450Spec> {
        EnblI2c10W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable I2C11"]
    #[inline(always)]
    pub fn enbl_i2c11(&mut self) -> EnblI2c11W<Scu450Spec> {
        EnblI2c11W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable I2C12"]
    #[inline(always)]
    pub fn enbl_i2c12(&mut self) -> EnblI2c12W<Scu450Spec> {
        EnblI2c12W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable I2C13"]
    #[inline(always)]
    pub fn enbl_i2c13(&mut self) -> EnblI2c13W<Scu450Spec> {
        EnblI2c13W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable I2C14"]
    #[inline(always)]
    pub fn enbl_i2c14(&mut self) -> EnblI2c14W<Scu450Spec> {
        EnblI2c14W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu450Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Multi-function Pin Control \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu450::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu450::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu450Spec;
impl crate::RegisterSpec for Scu450Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu450::R`](R) reader structure"]
impl crate::Readable for Scu450Spec {}
#[doc = "`write(|w| ..)` method takes [`scu450::W`](W) writer structure"]
impl crate::Writable for Scu450Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU450 to value 0xa000"]
impl crate::Resettable for Scu450Spec {
    const RESET_VALUE: u32 = 0xa000;
}
