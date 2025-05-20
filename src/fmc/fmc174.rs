#[doc = "Register `FMC174` reader"]
pub type R = crate::R<Fmc174Spec>;
#[doc = "Register `FMC174` writer"]
pub type W = crate::W<Fmc174Spec>;
#[doc = "Field `Cmd19` reader - Command #1"]
pub type Cmd19R = crate::FieldReader;
#[doc = "Field `Cmd19` writer - Command #1"]
pub type Cmd19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd29` reader - Command #2"]
pub type Cmd29R = crate::FieldReader;
#[doc = "Field `Cmd29` writer - Command #2"]
pub type Cmd29W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd39` reader - Command #3"]
pub type Cmd39R = crate::FieldReader;
#[doc = "Field `Cmd39` writer - Command #3"]
pub type Cmd39W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd9 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd9> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd9` reader - 3B/4B Command"]
pub type _3b4bcmd9R = crate::BitReader<_3b4bcmd9>;
impl _3b4bcmd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd9 {
        match self.bits {
            false => _3b4bcmd9::Command321AreFor3bMode,
            true => _3b4bcmd9::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd9::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd9::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd9` writer - 3B/4B Command"]
pub type _3b4bcmd9W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd9>;
impl<'a, REG> _3b4bcmd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd9::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd9::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting9 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting9> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting9 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting9 {}
#[doc = "Field `Cmd1Setting9` reader - Command #1 setting"]
pub type Cmd1setting9R = crate::FieldReader<Cmd1setting9>;
impl Cmd1setting9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting9 {
        match self.bits {
            0 => Cmd1setting9::Disable,
            1 => Cmd1setting9::ForRead,
            2 => Cmd1setting9::ForWrite,
            3 => Cmd1setting9::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting9::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting9::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting9::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting9::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting9` writer - Command #1 setting"]
pub type Cmd1setting9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting9, crate::Safe>;
impl<'a, REG> Cmd1setting9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting9::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting9::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting9::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting9::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting9 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting9> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting9 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting9 {}
#[doc = "Field `Cmd2Setting9` reader - Command #2 setting"]
pub type Cmd2setting9R = crate::FieldReader<Cmd2setting9>;
impl Cmd2setting9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting9 {
        match self.bits {
            0 => Cmd2setting9::Disable,
            1 => Cmd2setting9::ForRead,
            2 => Cmd2setting9::ForWrite,
            3 => Cmd2setting9::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting9::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting9::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting9::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting9::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting9` writer - Command #2 setting"]
pub type Cmd2setting9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting9, crate::Safe>;
impl<'a, REG> Cmd2setting9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting9::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting9::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting9::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting9::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting9 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting9> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting9 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting9 {}
#[doc = "Field `Cmd3Setting9` reader - Command #3 setting"]
pub type Cmd3setting9R = crate::FieldReader<Cmd3setting9>;
impl Cmd3setting9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting9 {
        match self.bits {
            0 => Cmd3setting9::Disable,
            1 => Cmd3setting9::ForRead,
            2 => Cmd3setting9::ForWrite,
            3 => Cmd3setting9::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting9::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting9::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting9::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting9::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting9` writer - Command #3 setting"]
pub type Cmd3setting9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting9, crate::Safe>;
impl<'a, REG> Cmd3setting9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting9::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting9::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting9::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting9::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd19(&self) -> Cmd19R {
        Cmd19R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd29(&self) -> Cmd29R {
        Cmd29R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd39(&self) -> Cmd39R {
        Cmd39R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd9(&self) -> _3b4bcmd9R {
        _3b4bcmd9R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting9(&self) -> Cmd1setting9R {
        Cmd1setting9R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting9(&self) -> Cmd2setting9R {
        Cmd2setting9R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting9(&self) -> Cmd3setting9R {
        Cmd3setting9R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd19(&mut self) -> Cmd19W<Fmc174Spec> {
        Cmd19W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd29(&mut self) -> Cmd29W<Fmc174Spec> {
        Cmd29W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd39(&mut self) -> Cmd39W<Fmc174Spec> {
        Cmd39W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd9(&mut self) -> _3b4bcmd9W<Fmc174Spec> {
        _3b4bcmd9W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting9(&mut self) -> Cmd1setting9W<Fmc174Spec> {
        Cmd1setting9W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting9(&mut self) -> Cmd2setting9W<Fmc174Spec> {
        Cmd2setting9W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting9(&mut self) -> Cmd3setting9W<Fmc174Spec> {
        Cmd3setting9W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc174::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc174::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc174Spec;
impl crate::RegisterSpec for Fmc174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc174::R`](R) reader structure"]
impl crate::Readable for Fmc174Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc174::W`](W) writer structure"]
impl crate::Writable for Fmc174Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC174 to value 0"]
impl crate::Resettable for Fmc174Spec {}
