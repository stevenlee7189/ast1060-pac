#[doc = "Register `HACE5C` reader"]
pub type R = crate::R<Hace5cSpec>;
#[doc = "Register `HACE5C` writer"]
pub type W = crate::W<Hace5cSpec>;
#[doc = "Field `CmdQueueReadPointer110` reader - Command queue read pointer\\[11:0\\] (8-byte aligned)"]
pub type CmdQueueReadPointer110R = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:11 - Command queue read pointer\\[11:0\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn cmd_queue_read_pointer110(&self) -> CmdQueueReadPointer110R {
        CmdQueueReadPointer110R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {}
#[doc = "Command Queue Read Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`hace5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace5cSpec;
impl crate::RegisterSpec for Hace5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace5c::R`](R) reader structure"]
impl crate::Readable for Hace5cSpec {}
#[doc = "`write(|w| ..)` method takes [`hace5c::W`](W) writer structure"]
impl crate::Writable for Hace5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE5C to value 0"]
impl crate::Resettable for Hace5cSpec {}
