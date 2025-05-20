#[doc = "Register `FMC154` reader"]
pub type R = crate::R<Fmc154Spec>;
#[doc = "Register `FMC154` writer"]
pub type W = crate::W<Fmc154Spec>;
#[doc = "Field `Cmd11` reader - Command #1"]
pub type Cmd11R = crate::FieldReader;
#[doc = "Field `Cmd11` writer - Command #1"]
pub type Cmd11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd21` reader - Command #2"]
pub type Cmd21R = crate::FieldReader;
#[doc = "Field `Cmd21` writer - Command #2"]
pub type Cmd21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd31` reader - Command #3"]
pub type Cmd31R = crate::FieldReader;
#[doc = "Field `Cmd31` writer - Command #3"]
pub type Cmd31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd1 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd1> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd1` reader - 3B/4B Command"]
pub type _3b4bcmd1R = crate::BitReader<_3b4bcmd1>;
impl _3b4bcmd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd1 {
        match self.bits {
            false => _3b4bcmd1::Command321AreFor3bMode,
            true => _3b4bcmd1::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd1::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd1::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd1` writer - 3B/4B Command"]
pub type _3b4bcmd1W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd1>;
impl<'a, REG> _3b4bcmd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd1::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd1::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting1 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting1> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting1 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting1 {}
#[doc = "Field `Cmd1Setting1` reader - Command #1 setting"]
pub type Cmd1setting1R = crate::FieldReader<Cmd1setting1>;
impl Cmd1setting1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting1 {
        match self.bits {
            0 => Cmd1setting1::Disable,
            1 => Cmd1setting1::ForRead,
            2 => Cmd1setting1::ForWrite,
            3 => Cmd1setting1::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting1::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting1::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting1::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting1::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting1` writer - Command #1 setting"]
pub type Cmd1setting1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting1, crate::Safe>;
impl<'a, REG> Cmd1setting1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting1::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting1::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting1::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting1::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting1 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting1> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting1 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting1 {}
#[doc = "Field `Cmd2Setting1` reader - Command #2 setting"]
pub type Cmd2setting1R = crate::FieldReader<Cmd2setting1>;
impl Cmd2setting1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting1 {
        match self.bits {
            0 => Cmd2setting1::Disable,
            1 => Cmd2setting1::ForRead,
            2 => Cmd2setting1::ForWrite,
            3 => Cmd2setting1::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting1::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting1::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting1::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting1::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting1` writer - Command #2 setting"]
pub type Cmd2setting1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting1, crate::Safe>;
impl<'a, REG> Cmd2setting1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting1::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting1::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting1::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting1::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting1 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting1> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting1 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting1 {}
#[doc = "Field `Cmd3Setting1` reader - Command #3 setting"]
pub type Cmd3setting1R = crate::FieldReader<Cmd3setting1>;
impl Cmd3setting1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting1 {
        match self.bits {
            0 => Cmd3setting1::Disable,
            1 => Cmd3setting1::ForRead,
            2 => Cmd3setting1::ForWrite,
            3 => Cmd3setting1::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting1::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting1::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting1::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting1::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting1` writer - Command #3 setting"]
pub type Cmd3setting1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting1, crate::Safe>;
impl<'a, REG> Cmd3setting1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting1::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting1::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting1::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting1::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd11(&self) -> Cmd11R {
        Cmd11R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd21(&self) -> Cmd21R {
        Cmd21R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd31(&self) -> Cmd31R {
        Cmd31R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd1(&self) -> _3b4bcmd1R {
        _3b4bcmd1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting1(&self) -> Cmd1setting1R {
        Cmd1setting1R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting1(&self) -> Cmd2setting1R {
        Cmd2setting1R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting1(&self) -> Cmd3setting1R {
        Cmd3setting1R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd11(&mut self) -> Cmd11W<Fmc154Spec> {
        Cmd11W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd21(&mut self) -> Cmd21W<Fmc154Spec> {
        Cmd21W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd31(&mut self) -> Cmd31W<Fmc154Spec> {
        Cmd31W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd1(&mut self) -> _3b4bcmd1W<Fmc154Spec> {
        _3b4bcmd1W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting1(&mut self) -> Cmd1setting1W<Fmc154Spec> {
        Cmd1setting1W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting1(&mut self) -> Cmd2setting1W<Fmc154Spec> {
        Cmd2setting1W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting1(&mut self) -> Cmd3setting1W<Fmc154Spec> {
        Cmd3setting1W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc154Spec;
impl crate::RegisterSpec for Fmc154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc154::R`](R) reader structure"]
impl crate::Readable for Fmc154Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc154::W`](W) writer structure"]
impl crate::Writable for Fmc154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC154 to value 0x0800_00d8"]
impl crate::Resettable for Fmc154Spec {
    const RESET_VALUE: u32 = 0x0800_00d8;
}
