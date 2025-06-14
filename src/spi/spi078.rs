#[doc = "Register `SPI078` reader"]
pub type R = crate::R<Spi078Spec>;
#[doc = "Register `SPI078` writer"]
pub type W = crate::W<Spi078Spec>;
#[doc = "Field `CmdForBlock32KErase3BMode` reader - Command for Block (32K) Erase 3B mode"]
pub type CmdForBlock32kerase3bmodeR = crate::FieldReader;
#[doc = "Field `CmdForBlock32KErase3BMode` writer - Command for Block (32K) Erase 3B mode"]
pub type CmdForBlock32kerase3bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForBlock32KErase4BMode` reader - Command for Block (32K) Erase 4B mode"]
pub type CmdForBlock32kerase4bmodeR = crate::FieldReader;
#[doc = "Field `CmdForBlock32KErase4BMode` writer - Command for Block (32K) Erase 4B mode"]
pub type CmdForBlock32kerase4bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForBlock64KErase3BMode` reader - Command for Block (64K) Erase 3B mode"]
pub type CmdForBlock64kerase3bmodeR = crate::FieldReader;
#[doc = "Field `CmdForBlock64KErase3BMode` writer - Command for Block (64K) Erase 3B mode"]
pub type CmdForBlock64kerase3bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForBlock64KErase4BMode` reader - Command for Block (64K) Erase 4B mode"]
pub type CmdForBlock64kerase4bmodeR = crate::FieldReader;
#[doc = "Field `CmdForBlock64KErase4BMode` writer - Command for Block (64K) Erase 4B mode"]
pub type CmdForBlock64kerase4bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command for Block (32K) Erase 3B mode"]
    #[inline(always)]
    pub fn cmd_for_block32kerase3bmode(&self) -> CmdForBlock32kerase3bmodeR {
        CmdForBlock32kerase3bmodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command for Block (32K) Erase 4B mode"]
    #[inline(always)]
    pub fn cmd_for_block32kerase4bmode(&self) -> CmdForBlock32kerase4bmodeR {
        CmdForBlock32kerase4bmodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command for Block (64K) Erase 3B mode"]
    #[inline(always)]
    pub fn cmd_for_block64kerase3bmode(&self) -> CmdForBlock64kerase3bmodeR {
        CmdForBlock64kerase3bmodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Command for Block (64K) Erase 4B mode"]
    #[inline(always)]
    pub fn cmd_for_block64kerase4bmode(&self) -> CmdForBlock64kerase4bmodeR {
        CmdForBlock64kerase4bmodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command for Block (32K) Erase 3B mode"]
    #[inline(always)]
    pub fn cmd_for_block32kerase3bmode(&mut self) -> CmdForBlock32kerase3bmodeW<Spi078Spec> {
        CmdForBlock32kerase3bmodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command for Block (32K) Erase 4B mode"]
    #[inline(always)]
    pub fn cmd_for_block32kerase4bmode(&mut self) -> CmdForBlock32kerase4bmodeW<Spi078Spec> {
        CmdForBlock32kerase4bmodeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command for Block (64K) Erase 3B mode"]
    #[inline(always)]
    pub fn cmd_for_block64kerase3bmode(&mut self) -> CmdForBlock64kerase3bmodeW<Spi078Spec> {
        CmdForBlock64kerase3bmodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Command for Block (64K) Erase 4B mode"]
    #[inline(always)]
    pub fn cmd_for_block64kerase4bmode(&mut self) -> CmdForBlock64kerase4bmodeW<Spi078Spec> {
        CmdForBlock64kerase4bmodeW::new(self, 24)
    }
}
#[doc = "Host Direct Access Commands \\#3 (SPI1 only)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi078Spec;
impl crate::RegisterSpec for Spi078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi078::R`](R) reader structure"]
impl crate::Readable for Spi078Spec {}
#[doc = "`write(|w| ..)` method takes [`spi078::W`](W) writer structure"]
impl crate::Writable for Spi078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI078 to value 0xdcd8_5c52"]
impl crate::Resettable for Spi078Spec {
    const RESET_VALUE: u32 = 0xdcd8_5c52;
}
