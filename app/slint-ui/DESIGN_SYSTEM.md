# نظام التصميم الموحد - Design System Guide

## 🎨 نظام الألوان (Color System)

### الألوان الأساسية

```slint
ColorTheme.primary_color      // #1D1B42 - الأرجواني الداكن (اللون الأساسي)
ColorTheme.secondary_color    // #2969A4 - الأزرق (لون ثانوي)
ColorTheme.accent_color       // #00D4FF - السماوي (لون التركيز)
ColorTheme.neutral_color      // #EFF4FF - الأبيض الفاتح (النص/الخلفيات الفاتحة)
```

### الألوان الحالات (State Colors)

```slint
ColorTheme.success_color      // #10B981 - أخضر (للعمليات الناجحة)
ColorTheme.warning_color      // #F59E0B - برتقالي (للتحذيرات)
ColorTheme.error_color        // #EF4444 - أحمر (للأخطاء)
ColorTheme.info_color         // #0EA5E9 - أزرق فاتح (للمعلومات)
```

### درجات الألوان

```slint
ColorTheme.light_gray         // #F5F7FA - رمادي فاتح
ColorTheme.medium_gray        // #B8BEC6 - رمادي وسط
ColorTheme.dark_gray          // #5A6370 - رمادي داكن
```

## 📐 المسافات (Spacing System)

```slint
Spacing.xs      // 4px   - المسافات الصغيرة جداً
Spacing.sm      // 8px   - مسافات صغيرة
Spacing.md      // 16px  - مسافات متوسطة
Spacing.lg      // 24px  - مسافات كبيرة
Spacing.xl      // 32px  - مسافات كبيرة جداً
Spacing.xxl     // 48px  - مسافات ضخمة
```

## 🔲 حدود الزوايا (Border Radius)

```slint
BorderRadius.none      // 0px    - بدون تقوس
BorderRadius.sm        // 4px    - تقوس صغير
BorderRadius.md        // 8px    - تقوس متوسط
BorderRadius.lg        // 12px   - تقوس كبير
BorderRadius.full      // 9999px - دائري كامل
```

## 🔤 خط التنسيق (Typography)

```slint
Typography.heading1_size    // 32px - عنوان أساسي كبير
Typography.heading2_size    // 24px - عنوان ثانوي
Typography.heading3_size    // 20px - عنوان ثالث
Typography.body_size        // 14px - نص عام
Typography.small_size       // 12px - نص صغير

Typography.font_family      // "Roboto, Arial, sans-serif"
Typography.mono_family      // "Monaco, 'Courier New', monospace"
```

## 🎯 المكونات المخصصة (Styled Components)

### 1. StyledButton

```slint
import { StyledButton } from "theme.slint";

StyledButton {
    text: "انقر هنا";
    variant: "primary";        // primary, secondary, success, warning, error, ghost
    disabled: false;
    clicked => {
        // معالج الضغط
    }
}
```

**Variants:**

- `primary` - الزر الأساسي (أرجواني)
- `secondary` - الزر الثانوي (أزرق)
- `success` - زر النجاح (أخضر)
- `warning` - زر التحذير (برتقالي)
- `error` - زر الخطأ (أحمر)
- `ghost` - زر شفاف مع حد فقط

### 2. StyledCard

```slint
StyledCard {
    elevated: false;
    // يمكن وضع محتوى داخل البطاقة
}
```

### 3. StyledLabel

```slint
StyledLabel {
    label_text: "عنوان النموذج";
    size: "heading2";      // heading1, heading2, heading3, body, small
    weight: "bold";        // normal, bold
}
```

### 4. StyledBadge

```slint
StyledBadge {
    text: "جديد";
    variant: "success";    // primary, secondary, success, warning, error, info
}
```

### 5. StyledAlert

```slint
StyledAlert {
    message: "تم حفظ البيانات بنجاح!";
    alert_type: "success";  // info, success, warning, error
}
```

## 📋 أمثلة الاستخدام

### مثال 1: شاشة تسجيل الدخول

```slint
import { StyledButton, StyledLabel, ColorTheme, Spacing } from "theme.slint";

export component LoginScreen inherits Rectangle {
    background: ColorTheme.background;

    VerticalBox {
        spacing: Spacing.md;
        padding: Spacing.lg;

        StyledLabel {
            label_text: "تسجيل الدخول";
            size: "heading1";
            weight: "bold";
        }

        Rectangle {
            height: 40px;
            background: ColorTheme.surface;
            border-radius: BorderRadius.md;
            border-width: 1px;
            border-color: ColorTheme.border_color;
        }

        StyledButton {
            text: "دخول";
            variant: "primary";
            clicked => {
                // معالجة تسجيل الدخول
            }
        }
    }
}
```

### مثال 2: لوحة التحكم

```slint
export component Dashboard inherits Rectangle {
    background: ColorTheme.light_gray;

    VerticalBox {
        spacing: Spacing.md;
        padding: Spacing.lg;

        // الرأس
        StyledLabel {
            label_text: "لوحة التحكم";
            size: "heading2";
            weight: "bold";
        }

        // الحالات
        HorizontalBox {
            spacing: Spacing.md;

            StyledBadge { text: "نشط"; variant: "success"; }
            StyledBadge { text: "تحذير"; variant: "warning"; }
            StyledBadge { text: "خطأ"; variant: "error"; }
        }

        // الأزرار
        HorizontalBox {
            spacing: Spacing.md;

            StyledButton { text: "حفظ"; variant: "primary"; }
            StyledButton { text: "إلغاء"; variant: "ghost"; }
        }
    }
}
```

## 🔄 تخصيص الألوان

يمكنك تغيير الألوان الأساسية من خلال تحديث `ColorTheme` في `theme.slint`:

```slint
export global ColorTheme {
    in property <color> primary_color: #YOUR_COLOR;      // غيّر هنا
    in property <color> secondary_color: #YOUR_COLOR;    // غيّر هنا
    // ... الخ
}
```

## 💡 أفضل الممارسات

1. **استخدم الألوان بدقة**: استخدم `success_color` للعمليات الناجحة فقط
2. **اتبع المسافات**: استخدم الثوابت `Spacing` بدلاً من أرقام عشوائية
3. **استمر في التناسق**: استخدم نفس `BorderRadius` للمكونات المشابهة
4. **اختبر التباين**: تأكد من أن النصوص واضحة على الخلفيات
5. **كن عاقلاً**: لا تستخدم أكثر من 3-4 ألوان في نفس الشاشة

## 📱 الاستجابة

استخدم `min-width` و `min-height` مع `horizontal-stretch` و `vertical-stretch` لإنشاء واجهات تفاعلية:

```slint
StyledButton {
    horizontal-stretch: 1;  // يمتد عرضياً
    vertical-stretch: 0;    // لا يمتد عمودياً
}
```

---

تم إنشاء هذا النظام لتوحيد التصميم والألوان والمسافات عبر جميع مكونات التطبيق! 🎨✨
