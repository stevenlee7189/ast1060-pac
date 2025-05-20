#[doc = "Register `FMC178` reader"]
pub type R = crate::R<Fmc178Spec>;
#[doc = "Register `FMC178` writer"]
pub type W = crate::W<Fmc178Spec>;
#[doc = "Field `Cmd110` reader - Command #1"]
pub type Cmd110R = crate::FieldReader;
#[doc = "Field `Cmd110` writer - Command #1"]
pub type Cmd110W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd210` reader - Command #2"]
pub type Cmd210R = crate::FieldReader;
#[doc = "Field `Cmd210` writer - Command #2"]
pub type Cmd210W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd310` reader - Command #3"]
pub type Cmd310R = crate::FieldReader;
#[doc = "Field `Cmd310` writer - Command #3"]
pub type Cmd310W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd10 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd10> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd10` reader - 3B/4B Command"]
pub type _3b4bcmd10R = crate::BitReader<_3b4bcmd10>;
impl _3b4bcmd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd10 {
        match self.bits {
            false => _3b4bcmd10::Command321AreFor3bMode,
            true => _3b4bcmd10::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd10::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd10::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd10` writer - 3B/4B Command"]
pub type _3b4bcmd10W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd10>;
impl<'a, REG> _3b4bcmd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd10::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd10::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting10 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting10> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting10 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting10 {}
#[doc = "Field `Cmd1Setting10` reader - Command #1 setting"]
pub type Cmd1setting10R = crate::FieldReader<Cmd1setting10>;
impl Cmd1setting10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting10 {
        match self.bits {
            0 => Cmd1setting10::Disable,
            1 => Cmd1setting10::ForRead,
            2 => Cmd1setting10::ForWrite,
            3 => Cmd1setting10::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting10::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting10::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting10::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting10::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting10` writer - Command #1 setting"]
pub type Cmd1setting10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting10, crate::Safe>;
impl<'a, REG> Cmd1setting10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting10::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting10::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting10::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting10::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting10 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting10> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting10 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting10 {}
#[doc = "Field `Cmd2Setting10` reader - Command #2 setting"]
pub type Cmd2setting10R = crate::FieldReader<Cmd2setting10>;
impl Cmd2setting10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting10 {
        match self.bits {
            0 => Cmd2setting10::Disable,
            1 => Cmd2setting10::ForRead,
            2 => Cmd2setting10::ForWrite,
            3 => Cmd2setting10::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting10::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting10::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting10::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting10::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting10` writer - Command #2 setting"]
pub type Cmd2setting10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting10, crate::Safe>;
impl<'a, REG> Cmd2setting10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting10::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting10::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting10::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting10::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting10 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting10> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting10 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting10 {}
#[doc = "Field `Cmd3Setting10` reader - Command #3 setting"]
pub type Cmd3setting10R = crate::FieldReader<Cmd3setting10>;
impl Cmd3setting10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting10 {
        match self.bits {
            0 => Cmd3setting10::Disable,
            1 => Cmd3setting10::ForRead,
            2 => Cmd3setting10::ForWrite,
            3 => Cmd3setting10::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting10::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting10::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting10::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting10::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting10` writer - Command #3 setting"]
pub type Cmd3setting10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting10, crate::Safe>;
impl<'a, REG> Cmd3setting10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting10::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting10::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting10::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting10::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd110(&self) -> Cmd110R {
        Cmd110R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd210(&self) -> Cmd210R {
        Cmd210R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd310(&self) -> Cmd310R {
        Cmd310R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd10(&self) -> _3b4bcmd10R {
        _3b4bcmd10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting10(&self) -> Cmd1setting10R {
        Cmd1setting10R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting10(&self) -> Cmd2setting10R {
        Cmd2setting10R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting10(&self) -> Cmd3setting10R {
        Cmd3setting10R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd110(&mut self) -> Cmd110W<Fmc178Spec> {
        Cmd110W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd210(&mut self) -> Cmd210W<Fmc178Spec> {
        Cmd210W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd310(&mut self) -> Cmd310W<Fmc178Spec> {
        Cmd310W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd10(&mut self) -> _3b4bcmd10W<Fmc178Spec> {
        _3b4bcmd10W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting10(&mut self) -> Cmd1setting10W<Fmc178Spec> {
        Cmd1setting10W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting10(&mut self) -> Cmd2setting10W<Fmc178Spec> {
        Cmd2setting10W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting10(&mut self) -> Cmd3setting10W<Fmc178Spec> {
        Cmd3setting10W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc178Spec;
impl crate::RegisterSpec for Fmc178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc178::R`](R) reader structure"]
impl crate::Readable for Fmc178Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc178::W`](W) writer structure"]
impl crate::Writable for Fmc178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC178 to value 0"]
impl crate::Resettable for Fmc178Spec {}
