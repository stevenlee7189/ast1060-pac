#[doc = "Register `SPI164` reader"]
pub type R = crate::R<Spi164Spec>;
#[doc = "Register `SPI164` writer"]
pub type W = crate::W<Spi164Spec>;
#[doc = "Field `Cmd05` reader - Command 0"]
pub type Cmd05R = crate::FieldReader;
#[doc = "Field `Cmd05` writer - Command 0"]
pub type Cmd05W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd15` reader - Command 1"]
pub type Cmd15R = crate::FieldReader;
#[doc = "Field `Cmd15` writer - Command 1"]
pub type Cmd15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd25` reader - Command 2"]
pub type Cmd25R = crate::FieldReader;
#[doc = "Field `Cmd25` writer - Command 2"]
pub type Cmd25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd5 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd5> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd5` reader - 3B/4B Command"]
pub type _3b4bcmd5R = crate::BitReader<_3b4bcmd5>;
impl _3b4bcmd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd5 {
        match self.bits {
            false => _3b4bcmd5::Command210AreFor3bMode,
            true => _3b4bcmd5::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd5::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd5::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd5` writer - 3B/4B Command"]
pub type _3b4bcmd5W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd5>;
impl<'a, REG> _3b4bcmd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd5::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd5::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead5` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead5R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead5` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr5` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr5R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr5` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead5` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead5R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead5` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr5` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr5R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr5` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead5` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead5R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead5` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr5` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr5R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr5` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd05(&self) -> Cmd05R {
        Cmd05R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd15(&self) -> Cmd15R {
        Cmd15R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd25(&self) -> Cmd25R {
        Cmd25R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd5(&self) -> _3b4bcmd5R {
        _3b4bcmd5R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read5(&self) -> EnblCmd0forRead5R {
        EnblCmd0forRead5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr5(&self) -> EnblCmd0forWr5R {
        EnblCmd0forWr5R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read5(&self) -> EnblCmd1forRead5R {
        EnblCmd1forRead5R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr5(&self) -> EnblCmd1forWr5R {
        EnblCmd1forWr5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read5(&self) -> EnblCmd2forRead5R {
        EnblCmd2forRead5R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr5(&self) -> EnblCmd2forWr5R {
        EnblCmd2forWr5R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd05(&mut self) -> Cmd05W<Spi164Spec> {
        Cmd05W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd15(&mut self) -> Cmd15W<Spi164Spec> {
        Cmd15W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd25(&mut self) -> Cmd25W<Spi164Spec> {
        Cmd25W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd5(&mut self) -> _3b4bcmd5W<Spi164Spec> {
        _3b4bcmd5W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read5(&mut self) -> EnblCmd0forRead5W<Spi164Spec> {
        EnblCmd0forRead5W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr5(&mut self) -> EnblCmd0forWr5W<Spi164Spec> {
        EnblCmd0forWr5W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read5(&mut self) -> EnblCmd1forRead5W<Spi164Spec> {
        EnblCmd1forRead5W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr5(&mut self) -> EnblCmd1forWr5W<Spi164Spec> {
        EnblCmd1forWr5W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read5(&mut self) -> EnblCmd2forRead5W<Spi164Spec> {
        EnblCmd2forRead5W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr5(&mut self) -> EnblCmd2forWr5W<Spi164Spec> {
        EnblCmd2forWr5W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi164Spec;
impl crate::RegisterSpec for Spi164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi164::R`](R) reader structure"]
impl crate::Readable for Spi164Spec {}
#[doc = "`write(|w| ..)` method takes [`spi164::W`](W) writer structure"]
impl crate::Writable for Spi164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI164 to value 0"]
impl crate::Resettable for Spi164Spec {}
