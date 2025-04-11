#[doc = "Register `SPI170` reader"]
pub type R = crate::R<Spi170Spec>;
#[doc = "Register `SPI170` writer"]
pub type W = crate::W<Spi170Spec>;
#[doc = "Field `Cmd08` reader - Command 0"]
pub type Cmd08R = crate::FieldReader;
#[doc = "Field `Cmd08` writer - Command 0"]
pub type Cmd08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd18` reader - Command 1"]
pub type Cmd18R = crate::FieldReader;
#[doc = "Field `Cmd18` writer - Command 1"]
pub type Cmd18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd28` reader - Command 2"]
pub type Cmd28R = crate::FieldReader;
#[doc = "Field `Cmd28` writer - Command 2"]
pub type Cmd28W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd8 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd8> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd8` reader - 3B/4B Command"]
pub type _3b4bcmd8R = crate::BitReader<_3b4bcmd8>;
impl _3b4bcmd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd8 {
        match self.bits {
            false => _3b4bcmd8::Command210AreFor3bMode,
            true => _3b4bcmd8::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd8::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd8::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd8` writer - 3B/4B Command"]
pub type _3b4bcmd8W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd8>;
impl<'a, REG> _3b4bcmd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd8::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd8::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead8` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead8R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead8` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr8` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr8R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr8` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead8` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead8R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead8` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr8` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr8R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr8` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead8` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead8R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead8` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr8` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr8R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr8` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd08(&self) -> Cmd08R {
        Cmd08R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd18(&self) -> Cmd18R {
        Cmd18R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd28(&self) -> Cmd28R {
        Cmd28R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd8(&self) -> _3b4bcmd8R {
        _3b4bcmd8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read8(&self) -> EnblCmd0forRead8R {
        EnblCmd0forRead8R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr8(&self) -> EnblCmd0forWr8R {
        EnblCmd0forWr8R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read8(&self) -> EnblCmd1forRead8R {
        EnblCmd1forRead8R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr8(&self) -> EnblCmd1forWr8R {
        EnblCmd1forWr8R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read8(&self) -> EnblCmd2forRead8R {
        EnblCmd2forRead8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr8(&self) -> EnblCmd2forWr8R {
        EnblCmd2forWr8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd08(&mut self) -> Cmd08W<Spi170Spec> {
        Cmd08W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd18(&mut self) -> Cmd18W<Spi170Spec> {
        Cmd18W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd28(&mut self) -> Cmd28W<Spi170Spec> {
        Cmd28W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd8(&mut self) -> _3b4bcmd8W<Spi170Spec> {
        _3b4bcmd8W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read8(&mut self) -> EnblCmd0forRead8W<Spi170Spec> {
        EnblCmd0forRead8W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr8(&mut self) -> EnblCmd0forWr8W<Spi170Spec> {
        EnblCmd0forWr8W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read8(&mut self) -> EnblCmd1forRead8W<Spi170Spec> {
        EnblCmd1forRead8W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr8(&mut self) -> EnblCmd1forWr8W<Spi170Spec> {
        EnblCmd1forWr8W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read8(&mut self) -> EnblCmd2forRead8W<Spi170Spec> {
        EnblCmd2forRead8W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr8(&mut self) -> EnblCmd2forWr8W<Spi170Spec> {
        EnblCmd2forWr8W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi170::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi170::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi170Spec;
impl crate::RegisterSpec for Spi170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi170::R`](R) reader structure"]
impl crate::Readable for Spi170Spec {}
#[doc = "`write(|w| ..)` method takes [`spi170::W`](W) writer structure"]
impl crate::Writable for Spi170Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI170 to value 0"]
impl crate::Resettable for Spi170Spec {}
