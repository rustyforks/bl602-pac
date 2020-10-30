#[doc = "Register `se_aes_0_ctrl_prot` reader"]
pub struct R(crate::R<SE_AES_0_CTRL_PROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_CTRL_PROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_AES_0_CTRL_PROT_SPEC>> for R {
    fn from(reader: crate::R<SE_AES_0_CTRL_PROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_ctrl_prot` writer"]
pub struct W(crate::W<SE_AES_0_CTRL_PROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_CTRL_PROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<SE_AES_0_CTRL_PROT_SPEC>> for W {
    fn from(writer: crate::W<SE_AES_0_CTRL_PROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_id1_en` reader - "]
pub struct SE_AES_ID1_EN_R(crate::FieldReader<bool, bool>);
impl SE_AES_ID1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_ID1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_ID1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_id1_en` writer - "]
pub struct SE_AES_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_ID1_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `se_aes_id0_en` reader - "]
pub struct SE_AES_ID0_EN_R(crate::FieldReader<bool, bool>);
impl SE_AES_ID0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_id0_en` writer - "]
pub struct SE_AES_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_ID0_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `se_aes_prot_en` reader - "]
pub struct SE_AES_PROT_EN_R(crate::FieldReader<bool, bool>);
impl SE_AES_PROT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_PROT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_PROT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_prot_en` writer - "]
pub struct SE_AES_PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_PROT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_id1_en(&self) -> SE_AES_ID1_EN_R {
        SE_AES_ID1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_id0_en(&self) -> SE_AES_ID0_EN_R {
        SE_AES_ID0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_prot_en(&self) -> SE_AES_PROT_EN_R {
        SE_AES_PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_id1_en(&mut self) -> SE_AES_ID1_EN_W {
        SE_AES_ID1_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_id0_en(&mut self) -> SE_AES_ID0_EN_W {
        SE_AES_ID0_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_prot_en(&mut self) -> SE_AES_PROT_EN_W {
        SE_AES_PROT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_ctrl_prot](index.html) module"]
pub struct SE_AES_0_CTRL_PROT_SPEC;
impl crate::RegisterSpec for SE_AES_0_CTRL_PROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_ctrl_prot::R](R) reader structure"]
impl crate::Readable for SE_AES_0_CTRL_PROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_ctrl_prot::W](W) writer structure"]
impl crate::Writable for SE_AES_0_CTRL_PROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_aes_0_ctrl_prot to value 0"]
impl crate::Resettable for SE_AES_0_CTRL_PROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}