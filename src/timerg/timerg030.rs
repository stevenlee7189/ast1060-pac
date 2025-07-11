#[doc = "Register `TIMERG030` reader"]
pub type R = crate::R<Timerg030Spec>;
#[doc = "Register `TIMERG030` writer"]
pub type W = crate::W<Timerg030Spec>;
#[doc = "Timer enable for Timer/Counter #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter1 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter1> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter1` reader - Timer enable for Timer/Counter #1"]
pub type TimerEnblForTimerCounter1R = crate::BitReader<TimerEnblForTimerCounter1>;
impl TimerEnblForTimerCounter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter1 {
        match self.bits {
            false => TimerEnblForTimerCounter1::Disable,
            true => TimerEnblForTimerCounter1::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter1::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter1::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter1` writer - Timer enable for Timer/Counter #1"]
pub type TimerEnblForTimerCounter1W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter1>;
impl<'a, REG> TimerEnblForTimerCounter1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter1::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter1::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter1 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter1> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter1` reader - Clock selection for Timer/Counter #1"]
pub type ClkSelForTimerCounter1R = crate::BitReader<ClkSelForTimerCounter1>;
impl ClkSelForTimerCounter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter1 {
        match self.bits {
            false => ClkSelForTimerCounter1::ApbClockPclk,
            true => ClkSelForTimerCounter1::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter1::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter1::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter1` writer - Clock selection for Timer/Counter #1"]
pub type ClkSelForTimerCounter1W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter1>;
impl<'a, REG> ClkSelForTimerCounter1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter1::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter1::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter1 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter1> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter1` reader - Enable Overflow Interrupt for Timer/Counter #1"]
pub type EnblOverflowIntforTimerCounter1R = crate::BitReader<EnblOverflowIntforTimerCounter1>;
impl EnblOverflowIntforTimerCounter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter1 {
        match self . bits { false => EnblOverflowIntforTimerCounter1 :: Disable , true => EnblOverflowIntforTimerCounter1 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter1::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter1 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter1` writer - Enable Overflow Interrupt for Timer/Counter #1"]
pub type EnblOverflowIntforTimerCounter1W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter1>;
impl<'a, REG> EnblOverflowIntforTimerCounter1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter1::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter1 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer1CanBeRstByWatchdog` reader - Enable Timer #1 can be reset by watchdog"]
pub type EnblTimer1canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer1CanBeRstByWatchdog` writer - Enable Timer #1 can be reset by watchdog"]
pub type EnblTimer1canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter2 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter2> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter2` reader - Timer enable for Timer/Counter #2"]
pub type TimerEnblForTimerCounter2R = crate::BitReader<TimerEnblForTimerCounter2>;
impl TimerEnblForTimerCounter2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter2 {
        match self.bits {
            false => TimerEnblForTimerCounter2::Disable,
            true => TimerEnblForTimerCounter2::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter2::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter2::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter2` writer - Timer enable for Timer/Counter #2"]
pub type TimerEnblForTimerCounter2W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter2>;
impl<'a, REG> TimerEnblForTimerCounter2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter2::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter2::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter2 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter2> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter2` reader - Clock selection for Timer/Counter #2"]
pub type ClkSelForTimerCounter2R = crate::BitReader<ClkSelForTimerCounter2>;
impl ClkSelForTimerCounter2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter2 {
        match self.bits {
            false => ClkSelForTimerCounter2::ApbClockPclk,
            true => ClkSelForTimerCounter2::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter2::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter2::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter2` writer - Clock selection for Timer/Counter #2"]
pub type ClkSelForTimerCounter2W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter2>;
impl<'a, REG> ClkSelForTimerCounter2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter2::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter2::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter2 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter2> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter2` reader - Enable Overflow Interrupt for Timer/Counter #2"]
pub type EnblOverflowIntforTimerCounter2R = crate::BitReader<EnblOverflowIntforTimerCounter2>;
impl EnblOverflowIntforTimerCounter2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter2 {
        match self . bits { false => EnblOverflowIntforTimerCounter2 :: Disable , true => EnblOverflowIntforTimerCounter2 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter2::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter2 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter2` writer - Enable Overflow Interrupt for Timer/Counter #2"]
pub type EnblOverflowIntforTimerCounter2W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter2>;
impl<'a, REG> EnblOverflowIntforTimerCounter2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter2::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter2 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer2CanBeRstByWatchdog` reader - Enable Timer #2 can be reset by watchdog"]
pub type EnblTimer2canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer2CanBeRstByWatchdog` writer - Enable Timer #2 can be reset by watchdog"]
pub type EnblTimer2canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter3 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter3> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter3` reader - Timer enable for Timer/Counter #3"]
pub type TimerEnblForTimerCounter3R = crate::BitReader<TimerEnblForTimerCounter3>;
impl TimerEnblForTimerCounter3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter3 {
        match self.bits {
            false => TimerEnblForTimerCounter3::Disable,
            true => TimerEnblForTimerCounter3::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter3::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter3::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter3` writer - Timer enable for Timer/Counter #3"]
pub type TimerEnblForTimerCounter3W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter3>;
impl<'a, REG> TimerEnblForTimerCounter3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter3::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter3::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter3 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter3> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter3` reader - Clock selection for Timer/Counter #3"]
pub type ClkSelForTimerCounter3R = crate::BitReader<ClkSelForTimerCounter3>;
impl ClkSelForTimerCounter3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter3 {
        match self.bits {
            false => ClkSelForTimerCounter3::ApbClockPclk,
            true => ClkSelForTimerCounter3::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter3::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter3::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter3` writer - Clock selection for Timer/Counter #3"]
pub type ClkSelForTimerCounter3W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter3>;
impl<'a, REG> ClkSelForTimerCounter3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter3::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter3::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter3 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter3> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter3` reader - Enable Overflow Interrupt for Timer/Counter #3"]
pub type EnblOverflowIntforTimerCounter3R = crate::BitReader<EnblOverflowIntforTimerCounter3>;
impl EnblOverflowIntforTimerCounter3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter3 {
        match self . bits { false => EnblOverflowIntforTimerCounter3 :: Disable , true => EnblOverflowIntforTimerCounter3 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter3::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter3 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter3` writer - Enable Overflow Interrupt for Timer/Counter #3"]
pub type EnblOverflowIntforTimerCounter3W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter3>;
impl<'a, REG> EnblOverflowIntforTimerCounter3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter3::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter3 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer3CanBeRstByWatchdog` reader - Enable Timer #3 can be reset by watchdog"]
pub type EnblTimer3canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer3CanBeRstByWatchdog` writer - Enable Timer #3 can be reset by watchdog"]
pub type EnblTimer3canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter4 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter4> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter4` reader - Timer enable for Timer/Counter #4"]
pub type TimerEnblForTimerCounter4R = crate::BitReader<TimerEnblForTimerCounter4>;
impl TimerEnblForTimerCounter4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter4 {
        match self.bits {
            false => TimerEnblForTimerCounter4::Disable,
            true => TimerEnblForTimerCounter4::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter4::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter4::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter4` writer - Timer enable for Timer/Counter #4"]
pub type TimerEnblForTimerCounter4W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter4>;
impl<'a, REG> TimerEnblForTimerCounter4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter4::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter4::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter4 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter4> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter4` reader - Clock selection for Timer/Counter #4"]
pub type ClkSelForTimerCounter4R = crate::BitReader<ClkSelForTimerCounter4>;
impl ClkSelForTimerCounter4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter4 {
        match self.bits {
            false => ClkSelForTimerCounter4::ApbClockPclk,
            true => ClkSelForTimerCounter4::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter4::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter4::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter4` writer - Clock selection for Timer/Counter #4"]
pub type ClkSelForTimerCounter4W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter4>;
impl<'a, REG> ClkSelForTimerCounter4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter4::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter4::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter4 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter4> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter4` reader - Enable Overflow Interrupt for Timer/Counter #4"]
pub type EnblOverflowIntforTimerCounter4R = crate::BitReader<EnblOverflowIntforTimerCounter4>;
impl EnblOverflowIntforTimerCounter4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter4 {
        match self . bits { false => EnblOverflowIntforTimerCounter4 :: Disable , true => EnblOverflowIntforTimerCounter4 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter4::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter4 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter4` writer - Enable Overflow Interrupt for Timer/Counter #4"]
pub type EnblOverflowIntforTimerCounter4W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter4>;
impl<'a, REG> EnblOverflowIntforTimerCounter4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter4::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter4 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer4CanBeRstByWatchdog` reader - Enable Timer #4 can be reset by watchdog"]
pub type EnblTimer4canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer4CanBeRstByWatchdog` writer - Enable Timer #4 can be reset by watchdog"]
pub type EnblTimer4canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter5 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter5> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter5` reader - Timer enable for Timer/Counter #5"]
pub type TimerEnblForTimerCounter5R = crate::BitReader<TimerEnblForTimerCounter5>;
impl TimerEnblForTimerCounter5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter5 {
        match self.bits {
            false => TimerEnblForTimerCounter5::Disable,
            true => TimerEnblForTimerCounter5::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter5::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter5::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter5` writer - Timer enable for Timer/Counter #5"]
pub type TimerEnblForTimerCounter5W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter5>;
impl<'a, REG> TimerEnblForTimerCounter5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter5::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter5::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter5 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter5> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter5` reader - Clock selection for Timer/Counter #5"]
pub type ClkSelForTimerCounter5R = crate::BitReader<ClkSelForTimerCounter5>;
impl ClkSelForTimerCounter5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter5 {
        match self.bits {
            false => ClkSelForTimerCounter5::ApbClockPclk,
            true => ClkSelForTimerCounter5::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter5::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter5::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter5` writer - Clock selection for Timer/Counter #5"]
pub type ClkSelForTimerCounter5W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter5>;
impl<'a, REG> ClkSelForTimerCounter5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter5::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter5::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter5 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter5> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter5` reader - Enable Overflow Interrupt for Timer/Counter #5"]
pub type EnblOverflowIntforTimerCounter5R = crate::BitReader<EnblOverflowIntforTimerCounter5>;
impl EnblOverflowIntforTimerCounter5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter5 {
        match self . bits { false => EnblOverflowIntforTimerCounter5 :: Disable , true => EnblOverflowIntforTimerCounter5 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter5::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter5 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter5` writer - Enable Overflow Interrupt for Timer/Counter #5"]
pub type EnblOverflowIntforTimerCounter5W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter5>;
impl<'a, REG> EnblOverflowIntforTimerCounter5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter5::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter5 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer5CanBeRstByWatchdog` reader - Enable Timer #5 can be reset by watchdog"]
pub type EnblTimer5canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer5CanBeRstByWatchdog` writer - Enable Timer #5 can be reset by watchdog"]
pub type EnblTimer5canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter6 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter6> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter6` reader - Timer enable for Timer/Counter #6"]
pub type TimerEnblForTimerCounter6R = crate::BitReader<TimerEnblForTimerCounter6>;
impl TimerEnblForTimerCounter6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter6 {
        match self.bits {
            false => TimerEnblForTimerCounter6::Disable,
            true => TimerEnblForTimerCounter6::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter6::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter6::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter6` writer - Timer enable for Timer/Counter #6"]
pub type TimerEnblForTimerCounter6W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter6>;
impl<'a, REG> TimerEnblForTimerCounter6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter6::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter6::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter6 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter6> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter6` reader - Clock selection for Timer/Counter #6"]
pub type ClkSelForTimerCounter6R = crate::BitReader<ClkSelForTimerCounter6>;
impl ClkSelForTimerCounter6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter6 {
        match self.bits {
            false => ClkSelForTimerCounter6::ApbClockPclk,
            true => ClkSelForTimerCounter6::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter6::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter6::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter6` writer - Clock selection for Timer/Counter #6"]
pub type ClkSelForTimerCounter6W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter6>;
impl<'a, REG> ClkSelForTimerCounter6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter6::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter6::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter6 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter6> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter6` reader - Enable Overflow Interrupt for Timer/Counter #6"]
pub type EnblOverflowIntforTimerCounter6R = crate::BitReader<EnblOverflowIntforTimerCounter6>;
impl EnblOverflowIntforTimerCounter6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter6 {
        match self . bits { false => EnblOverflowIntforTimerCounter6 :: Disable , true => EnblOverflowIntforTimerCounter6 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter6::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter6 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter6` writer - Enable Overflow Interrupt for Timer/Counter #6"]
pub type EnblOverflowIntforTimerCounter6W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter6>;
impl<'a, REG> EnblOverflowIntforTimerCounter6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter6::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter6 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer6CanBeRstByWatchdog` reader - Enable Timer #6 can be reset by watchdog"]
pub type EnblTimer6canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer6CanBeRstByWatchdog` writer - Enable Timer #6 can be reset by watchdog"]
pub type EnblTimer6canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter7 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter7> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter7` reader - Timer enable for Timer/Counter #7"]
pub type TimerEnblForTimerCounter7R = crate::BitReader<TimerEnblForTimerCounter7>;
impl TimerEnblForTimerCounter7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter7 {
        match self.bits {
            false => TimerEnblForTimerCounter7::Disable,
            true => TimerEnblForTimerCounter7::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter7::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter7::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter7` writer - Timer enable for Timer/Counter #7"]
pub type TimerEnblForTimerCounter7W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter7>;
impl<'a, REG> TimerEnblForTimerCounter7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter7::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter7::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter7 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter7> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter7` reader - Clock selection for Timer/Counter #7"]
pub type ClkSelForTimerCounter7R = crate::BitReader<ClkSelForTimerCounter7>;
impl ClkSelForTimerCounter7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter7 {
        match self.bits {
            false => ClkSelForTimerCounter7::ApbClockPclk,
            true => ClkSelForTimerCounter7::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter7::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter7::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter7` writer - Clock selection for Timer/Counter #7"]
pub type ClkSelForTimerCounter7W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter7>;
impl<'a, REG> ClkSelForTimerCounter7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter7::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter7::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter7 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter7> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter7` reader - Enable Overflow Interrupt for Timer/Counter #7"]
pub type EnblOverflowIntforTimerCounter7R = crate::BitReader<EnblOverflowIntforTimerCounter7>;
impl EnblOverflowIntforTimerCounter7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter7 {
        match self . bits { false => EnblOverflowIntforTimerCounter7 :: Disable , true => EnblOverflowIntforTimerCounter7 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter7::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter7 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter7` writer - Enable Overflow Interrupt for Timer/Counter #7"]
pub type EnblOverflowIntforTimerCounter7W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter7>;
impl<'a, REG> EnblOverflowIntforTimerCounter7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter7::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter7 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer7CanBeRstByWatchdog` reader - Enable Timer #7 can be reset by watchdog"]
pub type EnblTimer7canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer7CanBeRstByWatchdog` writer - Enable Timer #7 can be reset by watchdog"]
pub type EnblTimer7canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer enable for Timer/Counter #8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEnblForTimerCounter8 {
    #[doc = "0: disable"]
    Disable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<TimerEnblForTimerCounter8> for bool {
    #[inline(always)]
    fn from(variant: TimerEnblForTimerCounter8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimerEnblForTimerCounter8` reader - Timer enable for Timer/Counter #8"]
pub type TimerEnblForTimerCounter8R = crate::BitReader<TimerEnblForTimerCounter8>;
impl TimerEnblForTimerCounter8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEnblForTimerCounter8 {
        match self.bits {
            false => TimerEnblForTimerCounter8::Disable,
            true => TimerEnblForTimerCounter8::Enable,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimerEnblForTimerCounter8::Disable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimerEnblForTimerCounter8::Enable
    }
}
#[doc = "Field `TimerEnblForTimerCounter8` writer - Timer enable for Timer/Counter #8"]
pub type TimerEnblForTimerCounter8W<'a, REG> = crate::BitWriter<'a, REG, TimerEnblForTimerCounter8>;
impl<'a, REG> TimerEnblForTimerCounter8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter8::Disable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEnblForTimerCounter8::Enable)
    }
}
#[doc = "Clock selection for Timer/Counter #8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSelForTimerCounter8 {
    #[doc = "0: APB clock (PCLK)"]
    ApbClockPclk = 0,
    #[doc = "1: 1 MHz clock"]
    _1MhzClock = 1,
}
impl From<ClkSelForTimerCounter8> for bool {
    #[inline(always)]
    fn from(variant: ClkSelForTimerCounter8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSelForTimerCounter8` reader - Clock selection for Timer/Counter #8"]
pub type ClkSelForTimerCounter8R = crate::BitReader<ClkSelForTimerCounter8>;
impl ClkSelForTimerCounter8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSelForTimerCounter8 {
        match self.bits {
            false => ClkSelForTimerCounter8::ApbClockPclk,
            true => ClkSelForTimerCounter8::_1MhzClock,
        }
    }
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn is_apb_clock_pclk(&self) -> bool {
        *self == ClkSelForTimerCounter8::ApbClockPclk
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn is_1_mhz_clock(&self) -> bool {
        *self == ClkSelForTimerCounter8::_1MhzClock
    }
}
#[doc = "Field `ClkSelForTimerCounter8` writer - Clock selection for Timer/Counter #8"]
pub type ClkSelForTimerCounter8W<'a, REG> = crate::BitWriter<'a, REG, ClkSelForTimerCounter8>;
impl<'a, REG> ClkSelForTimerCounter8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "APB clock (PCLK)"]
    #[inline(always)]
    pub fn apb_clock_pclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter8::ApbClockPclk)
    }
    #[doc = "1 MHz clock"]
    #[inline(always)]
    pub fn _1_mhz_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSelForTimerCounter8::_1MhzClock)
    }
}
#[doc = "Enable Overflow Interrupt for Timer/Counter #8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOverflowIntforTimerCounter8 {
    #[doc = "0: disable."]
    Disable = 0,
    #[doc = "1: enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated = 1,
}
impl From<EnblOverflowIntforTimerCounter8> for bool {
    #[inline(always)]
    fn from(variant: EnblOverflowIntforTimerCounter8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter8` reader - Enable Overflow Interrupt for Timer/Counter #8"]
pub type EnblOverflowIntforTimerCounter8R = crate::BitReader<EnblOverflowIntforTimerCounter8>;
impl EnblOverflowIntforTimerCounter8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOverflowIntforTimerCounter8 {
        match self . bits { false => EnblOverflowIntforTimerCounter8 :: Disable , true => EnblOverflowIntforTimerCounter8 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated , }
    }
    #[doc = "disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOverflowIntforTimerCounter8::Disable
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn is_enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        &self,
    ) -> bool {
        * self == EnblOverflowIntforTimerCounter8 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated
    }
}
#[doc = "Field `EnblOverflowINTForTimerCounter8` writer - Enable Overflow Interrupt for Timer/Counter #8"]
pub type EnblOverflowIntforTimerCounter8W<'a, REG> =
    crate::BitWriter<'a, REG, EnblOverflowIntforTimerCounter8>;
impl<'a, REG> EnblOverflowIntforTimerCounter8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOverflowIntforTimerCounter8::Disable)
    }
    #[doc = "enable. when timer overflow (count to 0) occurred, interrupt will be generated"]
    #[inline(always)]
    pub fn enable_when_timer_overflow_count_to_0_occurred_interrupt_will_be_generated(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblOverflowIntforTimerCounter8 :: EnableWhenTimerOverflowCountTo0OccurredInterruptWillBeGenerated)
    }
}
#[doc = "Field `EnblTimer8CanBeRstByWatchdog` reader - Enable Timer #8 can be reset by watchdog"]
pub type EnblTimer8canBeRstByWatchdogR = crate::BitReader;
#[doc = "Field `EnblTimer8CanBeRstByWatchdog` writer - Enable Timer #8 can be reset by watchdog"]
pub type EnblTimer8canBeRstByWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer enable for Timer/Counter #1"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter1(&self) -> TimerEnblForTimerCounter1R {
        TimerEnblForTimerCounter1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock selection for Timer/Counter #1"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter1(&self) -> ClkSelForTimerCounter1R {
        ClkSelForTimerCounter1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Overflow Interrupt for Timer/Counter #1"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter1(&self) -> EnblOverflowIntforTimerCounter1R {
        EnblOverflowIntforTimerCounter1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Timer #1 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer1can_be_rst_by_watchdog(&self) -> EnblTimer1canBeRstByWatchdogR {
        EnblTimer1canBeRstByWatchdogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer enable for Timer/Counter #2"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter2(&self) -> TimerEnblForTimerCounter2R {
        TimerEnblForTimerCounter2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock selection for Timer/Counter #2"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter2(&self) -> ClkSelForTimerCounter2R {
        ClkSelForTimerCounter2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Overflow Interrupt for Timer/Counter #2"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter2(&self) -> EnblOverflowIntforTimerCounter2R {
        EnblOverflowIntforTimerCounter2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Timer #2 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer2can_be_rst_by_watchdog(&self) -> EnblTimer2canBeRstByWatchdogR {
        EnblTimer2canBeRstByWatchdogR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer enable for Timer/Counter #3"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter3(&self) -> TimerEnblForTimerCounter3R {
        TimerEnblForTimerCounter3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock selection for Timer/Counter #3"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter3(&self) -> ClkSelForTimerCounter3R {
        ClkSelForTimerCounter3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Overflow Interrupt for Timer/Counter #3"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter3(&self) -> EnblOverflowIntforTimerCounter3R {
        EnblOverflowIntforTimerCounter3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Timer #3 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer3can_be_rst_by_watchdog(&self) -> EnblTimer3canBeRstByWatchdogR {
        EnblTimer3canBeRstByWatchdogR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer enable for Timer/Counter #4"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter4(&self) -> TimerEnblForTimerCounter4R {
        TimerEnblForTimerCounter4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Clock selection for Timer/Counter #4"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter4(&self) -> ClkSelForTimerCounter4R {
        ClkSelForTimerCounter4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Overflow Interrupt for Timer/Counter #4"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter4(&self) -> EnblOverflowIntforTimerCounter4R {
        EnblOverflowIntforTimerCounter4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Timer #4 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer4can_be_rst_by_watchdog(&self) -> EnblTimer4canBeRstByWatchdogR {
        EnblTimer4canBeRstByWatchdogR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer enable for Timer/Counter #5"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter5(&self) -> TimerEnblForTimerCounter5R {
        TimerEnblForTimerCounter5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock selection for Timer/Counter #5"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter5(&self) -> ClkSelForTimerCounter5R {
        ClkSelForTimerCounter5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Overflow Interrupt for Timer/Counter #5"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter5(&self) -> EnblOverflowIntforTimerCounter5R {
        EnblOverflowIntforTimerCounter5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Timer #5 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer5can_be_rst_by_watchdog(&self) -> EnblTimer5canBeRstByWatchdogR {
        EnblTimer5canBeRstByWatchdogR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer enable for Timer/Counter #6"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter6(&self) -> TimerEnblForTimerCounter6R {
        TimerEnblForTimerCounter6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clock selection for Timer/Counter #6"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter6(&self) -> ClkSelForTimerCounter6R {
        ClkSelForTimerCounter6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Overflow Interrupt for Timer/Counter #6"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter6(&self) -> EnblOverflowIntforTimerCounter6R {
        EnblOverflowIntforTimerCounter6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Timer #6 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer6can_be_rst_by_watchdog(&self) -> EnblTimer6canBeRstByWatchdogR {
        EnblTimer6canBeRstByWatchdogR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer enable for Timer/Counter #7"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter7(&self) -> TimerEnblForTimerCounter7R {
        TimerEnblForTimerCounter7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock selection for Timer/Counter #7"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter7(&self) -> ClkSelForTimerCounter7R {
        ClkSelForTimerCounter7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Overflow Interrupt for Timer/Counter #7"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter7(&self) -> EnblOverflowIntforTimerCounter7R {
        EnblOverflowIntforTimerCounter7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Timer #7 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer7can_be_rst_by_watchdog(&self) -> EnblTimer7canBeRstByWatchdogR {
        EnblTimer7canBeRstByWatchdogR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer enable for Timer/Counter #8"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter8(&self) -> TimerEnblForTimerCounter8R {
        TimerEnblForTimerCounter8R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock selection for Timer/Counter #8"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter8(&self) -> ClkSelForTimerCounter8R {
        ClkSelForTimerCounter8R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Overflow Interrupt for Timer/Counter #8"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter8(&self) -> EnblOverflowIntforTimerCounter8R {
        EnblOverflowIntforTimerCounter8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Timer #8 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer8can_be_rst_by_watchdog(&self) -> EnblTimer8canBeRstByWatchdogR {
        EnblTimer8canBeRstByWatchdogR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable for Timer/Counter #1"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter1(&mut self) -> TimerEnblForTimerCounter1W<Timerg030Spec> {
        TimerEnblForTimerCounter1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock selection for Timer/Counter #1"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter1(&mut self) -> ClkSelForTimerCounter1W<Timerg030Spec> {
        ClkSelForTimerCounter1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Overflow Interrupt for Timer/Counter #1"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter1(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter1W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Timer #1 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer1can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer1canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer1canBeRstByWatchdogW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer enable for Timer/Counter #2"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter2(&mut self) -> TimerEnblForTimerCounter2W<Timerg030Spec> {
        TimerEnblForTimerCounter2W::new(self, 4)
    }
    #[doc = "Bit 5 - Clock selection for Timer/Counter #2"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter2(&mut self) -> ClkSelForTimerCounter2W<Timerg030Spec> {
        ClkSelForTimerCounter2W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Overflow Interrupt for Timer/Counter #2"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter2(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter2W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter2W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Timer #2 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer2can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer2canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer2canBeRstByWatchdogW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer enable for Timer/Counter #3"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter3(&mut self) -> TimerEnblForTimerCounter3W<Timerg030Spec> {
        TimerEnblForTimerCounter3W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock selection for Timer/Counter #3"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter3(&mut self) -> ClkSelForTimerCounter3W<Timerg030Spec> {
        ClkSelForTimerCounter3W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Overflow Interrupt for Timer/Counter #3"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter3(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter3W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter3W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Timer #3 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer3can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer3canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer3canBeRstByWatchdogW::new(self, 11)
    }
    #[doc = "Bit 12 - Timer enable for Timer/Counter #4"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter4(&mut self) -> TimerEnblForTimerCounter4W<Timerg030Spec> {
        TimerEnblForTimerCounter4W::new(self, 12)
    }
    #[doc = "Bit 13 - Clock selection for Timer/Counter #4"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter4(&mut self) -> ClkSelForTimerCounter4W<Timerg030Spec> {
        ClkSelForTimerCounter4W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Overflow Interrupt for Timer/Counter #4"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter4(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter4W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter4W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Timer #4 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer4can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer4canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer4canBeRstByWatchdogW::new(self, 15)
    }
    #[doc = "Bit 16 - Timer enable for Timer/Counter #5"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter5(&mut self) -> TimerEnblForTimerCounter5W<Timerg030Spec> {
        TimerEnblForTimerCounter5W::new(self, 16)
    }
    #[doc = "Bit 17 - Clock selection for Timer/Counter #5"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter5(&mut self) -> ClkSelForTimerCounter5W<Timerg030Spec> {
        ClkSelForTimerCounter5W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Overflow Interrupt for Timer/Counter #5"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter5(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter5W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter5W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Timer #5 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer5can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer5canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer5canBeRstByWatchdogW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer enable for Timer/Counter #6"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter6(&mut self) -> TimerEnblForTimerCounter6W<Timerg030Spec> {
        TimerEnblForTimerCounter6W::new(self, 20)
    }
    #[doc = "Bit 21 - Clock selection for Timer/Counter #6"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter6(&mut self) -> ClkSelForTimerCounter6W<Timerg030Spec> {
        ClkSelForTimerCounter6W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Overflow Interrupt for Timer/Counter #6"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter6(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter6W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter6W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Timer #6 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer6can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer6canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer6canBeRstByWatchdogW::new(self, 23)
    }
    #[doc = "Bit 24 - Timer enable for Timer/Counter #7"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter7(&mut self) -> TimerEnblForTimerCounter7W<Timerg030Spec> {
        TimerEnblForTimerCounter7W::new(self, 24)
    }
    #[doc = "Bit 25 - Clock selection for Timer/Counter #7"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter7(&mut self) -> ClkSelForTimerCounter7W<Timerg030Spec> {
        ClkSelForTimerCounter7W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Overflow Interrupt for Timer/Counter #7"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter7(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter7W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter7W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Timer #7 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer7can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer7canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer7canBeRstByWatchdogW::new(self, 27)
    }
    #[doc = "Bit 28 - Timer enable for Timer/Counter #8"]
    #[inline(always)]
    pub fn timer_enbl_for_timer_counter8(&mut self) -> TimerEnblForTimerCounter8W<Timerg030Spec> {
        TimerEnblForTimerCounter8W::new(self, 28)
    }
    #[doc = "Bit 29 - Clock selection for Timer/Counter #8"]
    #[inline(always)]
    pub fn clk_sel_for_timer_counter8(&mut self) -> ClkSelForTimerCounter8W<Timerg030Spec> {
        ClkSelForTimerCounter8W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Overflow Interrupt for Timer/Counter #8"]
    #[inline(always)]
    pub fn enbl_overflow_intfor_timer_counter8(
        &mut self,
    ) -> EnblOverflowIntforTimerCounter8W<Timerg030Spec> {
        EnblOverflowIntforTimerCounter8W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Timer #8 can be reset by watchdog"]
    #[inline(always)]
    pub fn enbl_timer8can_be_rst_by_watchdog(
        &mut self,
    ) -> EnblTimer8canBeRstByWatchdogW<Timerg030Spec> {
        EnblTimer8canBeRstByWatchdogW::new(self, 31)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerg030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerg030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timerg030Spec;
impl crate::RegisterSpec for Timerg030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerg030::R`](R) reader structure"]
impl crate::Readable for Timerg030Spec {}
#[doc = "`write(|w| ..)` method takes [`timerg030::W`](W) writer structure"]
impl crate::Writable for Timerg030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERG030 to value 0"]
impl crate::Resettable for Timerg030Spec {}
