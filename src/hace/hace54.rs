#[doc = "Register `HACE54` reader"]
pub type R = crate::R<Hace54Spec>;
#[doc = "Register `HACE54` writer"]
pub type W = crate::W<Hace54Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `CmdQueueEndPointer111Bf` reader - Command queue end pointer\\[11:1\\] bf(16-byte aligned)"]
pub type CmdQueueEndPointer111bfR = crate::FieldReader<u16>;
#[doc = "Field `CmdQueueEndPointer111Bf` writer - Command queue end pointer\\[11:1\\] bf(16-byte aligned)"]
pub type CmdQueueEndPointer111bfW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:11 - Command queue end pointer\\[11:1\\] bf(16-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_end_pointer111bf(&self) -> CmdQueueEndPointer111bfR {
        CmdQueueEndPointer111bfR::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 1:11 - Command queue end pointer\\[11:1\\] bf(16-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_end_pointer111bf(&mut self) -> CmdQueueEndPointer111bfW<Hace54Spec> {
        CmdQueueEndPointer111bfW::new(self, 1)
    }
}
#[doc = "Command Queue End Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`hace54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace54Spec;
impl crate::RegisterSpec for Hace54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace54::R`](R) reader structure"]
impl crate::Readable for Hace54Spec {}
#[doc = "`write(|w| ..)` method takes [`hace54::W`](W) writer structure"]
impl crate::Writable for Hace54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE54 to value 0"]
impl crate::Resettable for Hace54Spec {}
