#[doc = "Register `SCU054` reader"]
pub type R = crate::R<Scu054Spec>;
#[doc = "Register `SCU054` writer"]
pub type W = crate::W<Scu054Spec>;
#[doc = "Field `SCU050SysRstCtrlClearReg2` reader - SCU050 System Reset Control Clear Register 2"]
pub type Scu050sysRstCtrlClearReg2R = crate::FieldReader<u32>;
#[doc = "Field `SCU050SysRstCtrlClearReg2` writer - SCU050 System Reset Control Clear Register 2"]
pub type Scu050sysRstCtrlClearReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU050 System Reset Control Clear Register 2"]
    #[inline(always)]
    pub fn scu050sys_rst_ctrl_clear_reg2(&self) -> Scu050sysRstCtrlClearReg2R {
        Scu050sysRstCtrlClearReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU050 System Reset Control Clear Register 2"]
    #[inline(always)]
    pub fn scu050sys_rst_ctrl_clear_reg2(&mut self) -> Scu050sysRstCtrlClearReg2W<Scu054Spec> {
        Scu050sysRstCtrlClearReg2W::new(self, 0)
    }
}
#[doc = "System Reset Control Clear Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu054Spec;
impl crate::RegisterSpec for Scu054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu054::R`](R) reader structure"]
impl crate::Readable for Scu054Spec {}
#[doc = "`write(|w| ..)` method takes [`scu054::W`](W) writer structure"]
impl crate::Writable for Scu054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU054 to value 0"]
impl crate::Resettable for Scu054Spec {}
