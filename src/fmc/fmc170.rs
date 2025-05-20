#[doc = "Register `FMC170` reader"]
pub type R = crate::R<Fmc170Spec>;
#[doc = "Register `FMC170` writer"]
pub type W = crate::W<Fmc170Spec>;
#[doc = "Field `Cmd18` reader - Command #1"]
pub type Cmd18R = crate::FieldReader;
#[doc = "Field `Cmd18` writer - Command #1"]
pub type Cmd18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd28` reader - Command #2"]
pub type Cmd28R = crate::FieldReader;
#[doc = "Field `Cmd28` writer - Command #2"]
pub type Cmd28W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd38` reader - Command #3"]
pub type Cmd38R = crate::FieldReader;
#[doc = "Field `Cmd38` writer - Command #3"]
pub type Cmd38W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd8 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
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
            false => _3b4bcmd8::Command321AreFor3bMode,
            true => _3b4bcmd8::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd8::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd8::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd8` writer - 3B/4B Command"]
pub type _3b4bcmd8W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd8>;
impl<'a, REG> _3b4bcmd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd8::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd8::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting8 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting8> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting8 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting8 {}
#[doc = "Field `Cmd1Setting8` reader - Command #1 setting"]
pub type Cmd1setting8R = crate::FieldReader<Cmd1setting8>;
impl Cmd1setting8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting8 {
        match self.bits {
            0 => Cmd1setting8::Disable,
            1 => Cmd1setting8::ForRead,
            2 => Cmd1setting8::ForWrite,
            3 => Cmd1setting8::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting8::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting8::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting8::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting8::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting8` writer - Command #1 setting"]
pub type Cmd1setting8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting8, crate::Safe>;
impl<'a, REG> Cmd1setting8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting8::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting8::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting8::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting8::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting8 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting8> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting8 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting8 {}
#[doc = "Field `Cmd2Setting8` reader - Command #2 setting"]
pub type Cmd2setting8R = crate::FieldReader<Cmd2setting8>;
impl Cmd2setting8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting8 {
        match self.bits {
            0 => Cmd2setting8::Disable,
            1 => Cmd2setting8::ForRead,
            2 => Cmd2setting8::ForWrite,
            3 => Cmd2setting8::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting8::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting8::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting8::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting8::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting8` writer - Command #2 setting"]
pub type Cmd2setting8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting8, crate::Safe>;
impl<'a, REG> Cmd2setting8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting8::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting8::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting8::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting8::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting8 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting8> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting8 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting8 {}
#[doc = "Field `Cmd3Setting8` reader - Command #3 setting"]
pub type Cmd3setting8R = crate::FieldReader<Cmd3setting8>;
impl Cmd3setting8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting8 {
        match self.bits {
            0 => Cmd3setting8::Disable,
            1 => Cmd3setting8::ForRead,
            2 => Cmd3setting8::ForWrite,
            3 => Cmd3setting8::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting8::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting8::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting8::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting8::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting8` writer - Command #3 setting"]
pub type Cmd3setting8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting8, crate::Safe>;
impl<'a, REG> Cmd3setting8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting8::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting8::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting8::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting8::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd18(&self) -> Cmd18R {
        Cmd18R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd28(&self) -> Cmd28R {
        Cmd28R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd38(&self) -> Cmd38R {
        Cmd38R::new(((self.bits >> 16) & 0xff) as u8)
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
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting8(&self) -> Cmd1setting8R {
        Cmd1setting8R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting8(&self) -> Cmd2setting8R {
        Cmd2setting8R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting8(&self) -> Cmd3setting8R {
        Cmd3setting8R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd18(&mut self) -> Cmd18W<Fmc170Spec> {
        Cmd18W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd28(&mut self) -> Cmd28W<Fmc170Spec> {
        Cmd28W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd38(&mut self) -> Cmd38W<Fmc170Spec> {
        Cmd38W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd8(&mut self) -> _3b4bcmd8W<Fmc170Spec> {
        _3b4bcmd8W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting8(&mut self) -> Cmd1setting8W<Fmc170Spec> {
        Cmd1setting8W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting8(&mut self) -> Cmd2setting8W<Fmc170Spec> {
        Cmd2setting8W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting8(&mut self) -> Cmd3setting8W<Fmc170Spec> {
        Cmd3setting8W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc170::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc170::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc170Spec;
impl crate::RegisterSpec for Fmc170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc170::R`](R) reader structure"]
impl crate::Readable for Fmc170Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc170::W`](W) writer structure"]
impl crate::Writable for Fmc170Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC170 to value 0"]
impl crate::Resettable for Fmc170Spec {}
