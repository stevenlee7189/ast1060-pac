#[doc = "Register `SPI160` reader"]
pub type R = crate::R<Spi160Spec>;
#[doc = "Register `SPI160` writer"]
pub type W = crate::W<Spi160Spec>;
#[doc = "Field `Cmd04` reader - Command 0"]
pub type Cmd04R = crate::FieldReader;
#[doc = "Field `Cmd04` writer - Command 0"]
pub type Cmd04W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd14` reader - Command 1"]
pub type Cmd14R = crate::FieldReader;
#[doc = "Field `Cmd14` writer - Command 1"]
pub type Cmd14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd24` reader - Command 2"]
pub type Cmd24R = crate::FieldReader;
#[doc = "Field `Cmd24` writer - Command 2"]
pub type Cmd24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd4 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd4> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd4` reader - 3B/4B Command"]
pub type _3b4bcmd4R = crate::BitReader<_3b4bcmd4>;
impl _3b4bcmd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd4 {
        match self.bits {
            false => _3b4bcmd4::Command210AreFor3bMode,
            true => _3b4bcmd4::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd4::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd4::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd4` writer - 3B/4B Command"]
pub type _3b4bcmd4W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd4>;
impl<'a, REG> _3b4bcmd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd4::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd4::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead4` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead4R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead4` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr4` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr4R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr4` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead4` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead4R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead4` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr4` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr4R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr4` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead4` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead4R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead4` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr4` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr4R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr4` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd04(&self) -> Cmd04R {
        Cmd04R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd14(&self) -> Cmd14R {
        Cmd14R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd24(&self) -> Cmd24R {
        Cmd24R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd4(&self) -> _3b4bcmd4R {
        _3b4bcmd4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read4(&self) -> EnblCmd0forRead4R {
        EnblCmd0forRead4R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr4(&self) -> EnblCmd0forWr4R {
        EnblCmd0forWr4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read4(&self) -> EnblCmd1forRead4R {
        EnblCmd1forRead4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr4(&self) -> EnblCmd1forWr4R {
        EnblCmd1forWr4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read4(&self) -> EnblCmd2forRead4R {
        EnblCmd2forRead4R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr4(&self) -> EnblCmd2forWr4R {
        EnblCmd2forWr4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd04(&mut self) -> Cmd04W<Spi160Spec> {
        Cmd04W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd14(&mut self) -> Cmd14W<Spi160Spec> {
        Cmd14W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd24(&mut self) -> Cmd24W<Spi160Spec> {
        Cmd24W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd4(&mut self) -> _3b4bcmd4W<Spi160Spec> {
        _3b4bcmd4W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read4(&mut self) -> EnblCmd0forRead4W<Spi160Spec> {
        EnblCmd0forRead4W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr4(&mut self) -> EnblCmd0forWr4W<Spi160Spec> {
        EnblCmd0forWr4W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read4(&mut self) -> EnblCmd1forRead4W<Spi160Spec> {
        EnblCmd1forRead4W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr4(&mut self) -> EnblCmd1forWr4W<Spi160Spec> {
        EnblCmd1forWr4W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read4(&mut self) -> EnblCmd2forRead4W<Spi160Spec> {
        EnblCmd2forRead4W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr4(&mut self) -> EnblCmd2forWr4W<Spi160Spec> {
        EnblCmd2forWr4W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi160Spec;
impl crate::RegisterSpec for Spi160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi160::R`](R) reader structure"]
impl crate::Readable for Spi160Spec {}
#[doc = "`write(|w| ..)` method takes [`spi160::W`](W) writer structure"]
impl crate::Writable for Spi160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI160 to value 0"]
impl crate::Resettable for Spi160Spec {}
