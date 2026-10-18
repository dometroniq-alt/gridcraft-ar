//! Arabic translations for the interface strings in i18n.rs (GridCraft AR). Every label the other
//! catalogs know, plus the labels and tips the ribbon, dialogs, panes and Backstage pass to
//! [`super::Language::tr`]; anything absent falls back to English.
//!
//! Wording follows the Arabic spreadsheet vocabulary users know (لصق، قص، نسخ، تنسيق شرطي، جدول
//! محوري، تجميد الأجزاء، فرز وتصفية، التحقق من صحة البيانات…). Keyboard shortcuts, function
//! names, cell references and file extensions stay as in the source.

pub(super) fn translate(text: &str) -> Option<&'static str> {
    Some(match text {
        // Home › Clipboard.
        "Paste" => "لصق",
        "Paste Values" => "لصق القيم",
        "Paste Formulas" => "لصق الصيغ",
        "Paste Formatting" => "لصق التنسيق",
        "Transpose" => "تبديل الموضع",
        "Paste Link" => "لصق ارتباط",
        "Paste Special…" => "لصق خاص…",
        "Insert Copied Cells" => "إدراج الخلايا المنسوخة",
        "Insert Copied Cells…" => "إدراج الخلايا المنسوخة…",
        "Insert Cut Cells" => "إدراج الخلايا المقصوصة",
        "Insert Paste" => "لصق مع الإدراج",
        "Cut (⌘X)" => "قص (⌘X)",
        "Copy (⌘C)" => "نسخ (⌘C)",
        "Format Painter (double-click to keep it on)" => "نسخ التنسيق (انقر نقرًا مزدوجًا لإبقائه مفعّلًا)",
        // Home › Font.
        "Bold (⌘B)" => "غامق (⌘B)",
        "Italic (⌘I)" => "مائل (⌘I)",
        "Underline (⌘U)" => "تسطير (⌘U)",
        "Double Underline" => "تسطير مزدوج",
        "Strikethrough" => "يتوسطه خط",
        "Underline" => "تسطير",
        "Increase Font Size" => "زيادة حجم الخط",
        "Decrease Font Size" => "تصغير حجم الخط",
        "Wrap Text" => "التفاف النص",
        "Merge & Center" => "دمج وتوسيط",
        "Merge Cells" => "دمج الخلايا",
        "Merge Across" => "دمج عبر",
        "Unmerge Cells" => "إلغاء دمج الخلايا",
        "Fill Color" => "لون التعبئة",
        "Font Color" => "لون الخط",
        "Borders" => "حدود",
        // Home › Alignment.
        "Top\nAlign" => "محاذاة\nلأعلى",
        "Middle\nAlign" => "محاذاة\nللوسط",
        "Bottom\nAlign" => "محاذاة\nلأسفل",
        "Left" => "يسار",
        "Right" => "يمين",
        "Center" => "توسيط",
        "Horizontal" => "أفقي",
        "Vertical Text" => "نص عمودي",
        "Decrease Indent" => "إنقاص المسافة البادئة",
        "Increase Indent" => "زيادة المسافة البادئة",
        "Orientation" => "اتجاه النص",
        "Angle Counterclockwise" => "زاوية عكس اتجاه عقارب الساعة",
        "Angle Clockwise" => "زاوية باتجاه عقارب الساعة",
        "Rotate Text Up" => "تدوير النص لأعلى",
        "Rotate Text Down" => "تدوير النص لأسفل",
        // Home › Number.
        "Accounting Number Format" => "تنسيق أرقام المحاسبة",
        "Percent Style" => "نمط النسبة المئوية",
        "Comma Style" => "نمط الفاصلة",
        "Increase Decimal" => "زيادة المنازل العشرية",
        "Decrease Decimal" => "إنقاص المنازل العشرية",
        "More Accounting Formats…" => "تنسيقات محاسبة إضافية…",
        // Home › Styles, Cells, Editing.
        "Conditional\nFormatting" => "تنسيق\nشرطي",
        "Conditional Formatting" => "تنسيق شرطي",
        "Format\nas Table" => "تنسيق\nكجدول",
        "Cell\nStyles" => "أنماط\nالخلايا",
        "Format Cells…" => "تنسيق الخلايا…",
        "Format Cell Alignment…" => "تنسيق محاذاة الخلايا…",
        "Format\nPane" => "جزء\nالتنسيق",
        "Format" => "تنسيق",
        "Insert" => "إدراج",
        "Insert Cells…" => "إدراج خلايا…",
        "Delete" => "حذف",
        "Delete Cells…" => "حذف خلايا…",
        "Insert Sheet Rows" => "إدراج صفوف في الورقة",
        "Insert Sheet Columns" => "إدراج أعمدة في الورقة",
        "Delete Sheet Rows" => "حذف صفوف من الورقة",
        "Delete Sheet Columns" => "حذف أعمدة من الورقة",
        "Row Height…" => "ارتفاع الصف…",
        "AutoFit Row Height" => "احتواء تلقائي لارتفاع الصف",
        "Column Width…" => "عرض العمود…",
        "AutoFit Column Width" => "احتواء تلقائي لعرض العمود",
        "Default Width…" => "العرض الافتراضي…",
        "Hide Rows" => "إخفاء الصفوف",
        "Unhide Rows" => "إظهار الصفوف",
        "Hide Columns" => "إخفاء الأعمدة",
        "Unhide Columns" => "إظهار الأعمدة",
        "Lock Cell" => "تأمين الخلية",
        "Clear" => "مسح",
        "Clear All" => "مسح الكل",
        "Clear Formats" => "مسح التنسيقات",
        "Clear Contents" => "مسح المحتويات",
        "Clear Comments and Notes" => "مسح التعليقات والملاحظات",
        "Clear Hyperlinks" => "مسح الارتباطات التشعبية",
        "AutoSum" => "جمع تلقائي",
        "AutoSum (⌘⇧T)" => "جمع تلقائي (⌘⇧T)",
        "Flash\nFill" => "تعبئة\nسريعة",
        "Flash Fill" => "تعبئة سريعة",
        "Fill" => "تعبئة",
        "Clear Print Area" => "مسح ناحية الطباعة",
        "Find &\nSelect" => "بحث\nوتحديد",
        "Find…" => "بحث…",
        "Replace…" => "استبدال…",
        "Go To…" => "الانتقال إلى…",
        "Go To Special…" => "الانتقال إلى خاص…",
        // Insert.
        "PivotTable" => "جدول محوري",
        "Recommended\nCharts" => "المخططات\nالمستحسنة",
        "Table" => "جدول",
        "Pictures" => "صور",
        "Shapes" => "أشكال",
        "Icons" => "أيقونات",
        "Text\nBox" => "مربع\nنص",
        "Comment" => "تعليق",
        "New\nComment" => "تعليق\nجديد",
        "New Note" => "ملاحظة جديدة",
        "Show/Hide Note" => "إظهار/إخفاء الملاحظة",
        "Header &\nFooter" => "رأس\nوتذييل",
        "Text to\nColumns" => "تحويل النص\nإلى أعمدة",
        "Link" => "ارتباط",
        "Symbol" => "رمز",
        "Sparklines" => "خطوط المؤشر",
        "Checkbox" => "خانة اختيار",
        "Insert checkboxes in the selected cells" => "إدراج خانات اختيار في الخلايا المحددة",
        "Header & Footer" => "رأس وتذييل",
        // Draw.
        "Draw with ink" => "الرسم بالحبر",
        "Eraser" => "ممحاة",
        "Ink to\nShape" => "تحويل الحبر\nإلى شكل",
        "Select\nObjects" => "تحديد\nالعناصر",
        // Page Layout.
        "Themes" => "سمات",
        "Margins" => "هوامش",
        "Left Margin" => "الهامش الأيسر",
        "Right Margin" => "الهامش الأيمن",
        "Top Margin" => "الهامش العلوي",
        "Bottom Margin" => "الهامش السفلي",
        "Header Margin" => "هامش الرأس",
        "Footer Margin" => "هامش التذييل",
        "Size" => "الحجم",
        "Print\nArea" => "ناحية\nالطباعة",
        "Set Print Area" => "تعيين ناحية الطباعة",
        "Breaks" => "فواصل",
        "Insert Page Break" => "إدراج فاصل صفحات",
        "Remove Page Break" => "إزالة فاصل الصفحات",
        "Reset All Page Breaks" => "إعادة تعيين كل فواصل الصفحات",
        "Print\nTitles" => "عناوين\nالطباعة",
        "Sheet\nRight-to-Left" => "ورقة من\nاليمين إلى اليسار",
        "Sheet Right-to-Left" => "ورقة من اليمين إلى اليسار",
        "Narrow" => "ضيق",
        "Wide" => "عريض",
        "Normal" => "عادي",
        "Portrait" => "عمودي",
        "Landscape" => "أفقي",
        "Custom Margins…" => "هوامش مخصصة…",
        // Formulas.
        "Insert\nFunction" => "إدراج\nدالة",
        "Name\nManager" => "إدارة\nالأسماء",
        "Define Name" => "تعريف اسم",
        "Use in Formula" => "استخدام في الصيغة",
        "Create from Selection" => "إنشاء من التحديد",
        "Trace Precedents" => "تعقب السابقات",
        "Trace Dependents" => "تعقب التابعات",
        "Remove Arrows" => "إزالة الأسهم",
        "Show Formulas" => "إظهار الصيغ",
        "Error Checking" => "تدقيق الأخطاء",
        "Evaluate Formula" => "تقييم الصيغة",
        "Watch Window" => "نافذة المراقبة",
        "Calculation\nOptions" => "خيارات\nالحساب",
        "Calculate Now" => "احسب الآن",
        "Calculate Sheet" => "حساب الورقة",
        "Automatic" => "تلقائي",
        "Automatic Except for Data Tables" => "تلقائي باستثناء جداول البيانات",
        "Manual" => "يدوي",
        // Data.
        "Sort &\nFilter" => "فرز\nوتصفية",
        "Sort" => "فرز",
        "Sort A to Z" => "فرز من أ إلى ي",
        "Sort Z to A" => "فرز من ي إلى أ",
        "Custom Sort…" => "فرز مخصص…",
        "Filter" => "تصفية",
        "Reapply" => "إعادة التطبيق",
        "Get Data\n(Text/CSV)" => "الحصول على البيانات\n(نص/CSV)",
        "Remove\nDuplicates" => "إزالة\nالتكرارات",
        "Data\nValidation" => "التحقق من\nصحة البيانات",
        "Data Validation" => "التحقق من صحة البيانات",
        "Data Validation…" => "التحقق من صحة البيانات…",
        "Circle Invalid Data" => "تحديد البيانات غير الصالحة بدائرة",
        "Clear Validation" => "مسح التحقق من الصحة",
        "Group" => "تجميع",
        "Ungroup" => "فك التجميع",
        "Show Detail" => "إظهار التفاصيل",
        "Hide Detail" => "إخفاء التفاصيل",
        "Subtotal" => "مجموع فرعي",
        "Data\nAnalysis" => "تحليل\nالبيانات",
        "Data Analysis" => "تحليل البيانات",
        "Solver" => "الحلول (Solver)",
        "What-If\nAnalysis" => "تحليل\nماذا لو",
        "Data Table…" => "جدول بيانات…",
        "Scenario Manager…" => "إدارة السيناريوهات…",
        "Goal Seek…" => "استهداف القيمة…",
        "Forecast\nSheet" => "ورقة\nالتنبؤ",
        // Review.
        "Spelling" => "تدقيق إملائي",
        "Check\nAccessibility" => "التحقق من\nإمكانية الوصول",
        "Threaded Comments" => "تعليقات مترابطة",
        "Show\nComments" => "إظهار\nالتعليقات",
        "Previous" => "السابق",
        "Next" => "التالي",
        "Previous Comment" => "التعليق السابق",
        "Next Comment" => "التعليق التالي",
        "Show All Notes" => "إظهار كل الملاحظات",
        "Convert to Comments" => "تحويل إلى تعليقات",
        "Protect\nWorkbook" => "حماية\nالمصنف",
        "Allow Edit\nRanges" => "السماح بتحرير\nالنطاقات",
        "Allow Edit Ranges" => "السماح بتحرير النطاقات",
        "Unprotect\nSheet" => "إلغاء حماية\nالورقة",
        "Protect Sheet…" => "حماية الورقة…",
        "Comments" => "تعليقات",
        "Notes" => "ملاحظات",
        // View.
        "Page Break\nPreview" => "معاينة\nفواصل الصفحات",
        "Page\nLayout" => "تخطيط\nالصفحة",
        "Formula Bar" => "شريط الصيغة",
        "Gridlines" => "خطوط الشبكة",
        "Headings" => "العناوين",
        "Zoom" => "تكبير/تصغير",
        "100%" => "100%",
        "Zoom to\nSelection" => "تكبير/تصغير\nحسب التحديد",
        "Freeze\nPanes" => "تجميد\nالأجزاء",
        "Freeze Panes" => "تجميد الأجزاء",
        "Freeze Top Row" => "تجميد الصف العلوي",
        "Freeze First Column" => "تجميد العمود الأول",
        "Unfreeze Panes" => "إلغاء تجميد الأجزاء",
        "Dark Mode" => "الوضع الداكن",
        "Interface language" => "لغة الواجهة",
        "Interface size" => "حجم الواجهة",
        "Auto" => "تلقائي",
        "Drag to freeze columns" => "اسحب لتجميد الأعمدة",
        "Drag to freeze rows" => "اسحب لتجميد الصفوف",
        "Command\nPalette" => "لوحة\nالأوامر",
        "Agent\nControl" => "تحكم\nالوكيل",
        "Action\nJournal" => "سجل\nالإجراءات",
        "About\nGridCraft" => "حول\nGridCraft",
        "Share" => "مشاركة",
        // Chart Design and Table Design.
        "Add Chart\nElement" => "إضافة عنصر\nمخطط",
        "Change\nChart Type" => "تغيير نوع\nالمخطط",
        "Switch\nRow/Column" => "تبديل\nالصف/العمود",
        "Select\nData" => "تحديد\nالبيانات",
        "Data Labels: None" => "تسميات البيانات: بلا",
        "Data Labels: Show" => "تسميات البيانات: إظهار",
        "Legend: None" => "وسيلة الإيضاح: بلا",
        "Legend: Bottom" => "وسيلة الإيضاح: أسفل",
        "Legend: Top" => "وسيلة الإيضاح: أعلى",
        "Legend: Right" => "وسيلة الإيضاح: يمين",
        "Gridlines: None" => "خطوط الشبكة: بلا",
        "Gridlines: Show" => "خطوط الشبكة: إظهار",
        "Chart\nTitle" => "عنوان\nالمخطط",
        "Delete\nChart" => "حذف\nالمخطط",
        "Convert\nto Range" => "تحويل\nإلى نطاق",
        "Table\nStyles" => "أنماط\nالجدول",
        "Header Row" => "صف الرأس",
        "Total Row" => "صف الإجمالي",
        "Banded Rows" => "صفوف مُجزّأة",
        "First Column" => "العمود الأول",
        "Last Column" => "العمود الأخير",
        "Banded Columns" => "أعمدة مُجزّأة",
        // Conditional formatting menus.
        "Top 10 Items" => "أعلى 10 عناصر",
        "Top 10%" => "أعلى 10%",
        "Bottom 10 Items" => "أدنى 10 عناصر",
        "Bottom 10%" => "أدنى 10%",
        "Above Average" => "أعلى من المتوسط",
        "Below Average" => "أقل من المتوسط",
        "Clear Rules from Entire Sheet" => "مسح القواعد من الورقة بأكملها",
        "Clear Rules from Selected Cells" => "مسح القواعد من الخلايا المحددة",
        "Constants" => "ثوابت",
        "Data Bars" => "أشرطة البيانات",
        "Color Scales" => "مقاييس الألوان",
        "Icon Sets" => "مجموعات الأيقونات",
        // Ribbon tabs.
        "Home" => "الشريط الرئيسي",
        "Tab|Insert" => "إدراج",
        "Draw" => "رسم",
        "Page Layout" => "تخطيط الصفحة",
        "Formulas" => "صيغ",
        "Data" => "بيانات",
        "Review" => "مراجعة",
        "View" => "عرض",
        "Automate" => "أتمتة",
        "Table Design" => "تصميم الجدول",
        "Chart Design" => "تصميم المخطط",
        // Status bar.
        "Ready" => "جاهز",
        "Enter" => "إدخال",
        "Edit" => "تحرير",
        "Point" => "إشارة",
        "Calculate" => "حساب",
        "Select destination and press Enter or choose Paste" => "حدد الوجهة ثم اضغط Enter أو اختر لصق",
        // Number formats.
        "General" => "عام",
        "Number" => "رقم",
        "Currency" => "عملة",
        "Accounting" => "محاسبة",
        "Date" => "تاريخ",
        "Time" => "وقت",
        "Percentage" => "نسبة مئوية",
        "Fraction" => "كسر",
        "Scientific" => "علمي",
        "Text" => "نص",
        "Special" => "خاص",
        "Custom" => "مخصص",
        "Short Date" => "تاريخ قصير",
        "Long Date" => "تاريخ طويل",
        "More Number Formats…" => "تنسيقات أرقام إضافية…",
        // Function library.
        "Financial" => "مالية",
        "Logical" => "منطقية",
        "Date &\nTime" => "التاريخ\nوالوقت",
        "Lookup &\nReference" => "البحث\nوالمراجع",
        "Math &\nTrig" => "رياضيات\nومثلثات",
        "More\nFunctions" => "دالات\nإضافية",
        "Page Layout|Orientation" => "اتجاه الصفحة",
        // View options.
        "Display theme" => "سمة العرض",
        "Animations" => "الحركات",
        "Glide the selection outline to the cell you move to instead of jumping there" => {
            "انزلاق إطار التحديد إلى الخلية التي تنتقل إليها بدلًا من القفز إليها"
        }
        "System" => "النظام",
        "Light" => "فاتح",
        "Dark" => "داكن",
        "Sheet Options|View" => "عرض",
        "Print" => "طباعة",
        // Page Layout view and the Header & Footer tab.
        "Click to add data" => "انقر لإضافة بيانات",
        "Add header" => "إضافة رأس",
        "Add footer" => "إضافة تذييل",
        "(none)" => "(بلا)",
        "Header" => "رأس",
        "Footer" => "تذييل",
        "Page\nNumber" => "رقم\nالصفحة",
        "Number\nof Pages" => "عدد\nالصفحات",
        "Current\nDate" => "التاريخ\nالحالي",
        "Current\nTime" => "الوقت\nالحالي",
        "File\nPath" => "مسار\nالملف",
        "File\nName" => "اسم\nالملف",
        "Sheet\nName" => "اسم\nالورقة",
        "Go to\nHeader" => "الانتقال\nإلى الرأس",
        "Go to\nFooter" => "الانتقال\nإلى التذييل",
        "Page Number" => "رقم الصفحة",
        "Number of Pages" => "عدد الصفحات",
        "Current Date" => "التاريخ الحالي",
        "Current Time" => "الوقت الحالي",
        "File Path" => "مسار الملف",
        "File Name" => "اسم الملف",
        "Sheet Name" => "اسم الورقة",
        "Go to Header" => "الانتقال إلى الرأس",
        "Different First Page" => "صفحة أولى مختلفة",
        "Different Odd & Even Pages" => "صفحات فردية وزوجية مختلفة",
        "Go to Footer" => "الانتقال إلى التذييل",
        "Ruler" => "مسطرة",
        // File › Backstage view.
        "File" => "ملف",
        "Back" => "رجوع",
        "New" => "جديد",
        "Open" => "فتح",
        "Info" => "معلومات",
        "Save" => "حفظ",
        "Save As" => "حفظ باسم",
        "Export" => "تصدير",
        "Close" => "إغلاق",
        "Options" => "خيارات",
        "Blank workbook" => "مصنف فارغ",
        "Recent" => "الأخيرة",
        "No recent workbooks yet." => "لا توجد مصنفات حديثة بعد.",
        "More workbooks" => "مصنفات إضافية",
        "Modified" => "تاريخ التعديل",
        "Not available" => "غير متاح",
        "Just now" => "الآن",
        "Yesterday" => "أمس",
        "{n} min ago" => "قبل {n} دقيقة",
        "{n} h ago" => "قبل {n} ساعة",
        "{n} days ago" => "قبل {n} يوم",
        "{n} weeks ago" => "قبل {n} أسبوع",
        "{n} months ago" => "قبل {n} شهر",
        "{n} years ago" => "قبل {n} سنة",
        "Browse…" => "استعراض…",
        "Not saved yet" => "لم يُحفظ بعد",
        "Location" => "الموقع",
        "Sheets" => "الأوراق",
        "Protect Workbook" => "حماية المصنف",
        "Sheet structure is locked: sheets can't be added, moved, renamed or deleted." => {
            "بنية الأوراق مؤمّنة: لا يمكن إضافة الأوراق أو نقلها أو إعادة تسميتها أو حذفها."
        }
        "Lock the sheet structure so sheets can't be added, moved, renamed or deleted." => {
            "تأمين بنية الأوراق حتى لا يمكن إضافة الأوراق أو نقلها أو إعادة تسميتها أو حذفها."
        }
        "Properties" => "خصائص",
        "Title" => "العنوان",
        "Subject" => "الموضوع",
        "Author" => "المؤلف",
        "Company" => "الشركة",
        "Keywords" => "الكلمات المفتاحية",
        "Description" => "الوصف",
        "No workbook is open." => "لا يوجد مصنف مفتوح.",
        "Save as type" => "حفظ بنوع",
        "Excel Workbook (*.xlsx)" => "مصنف Excel (*.xlsx)",
        "CSV UTF-8 (Comma delimited) (*.csv)" => "CSV UTF-8 (مفصول بفواصل) (*.csv)",
        "Web Page (*.html)" => "صفحة ويب (*.html)",
        "PDF (*.pdf)" => "PDF (*.pdf)",
        "Download" => "تنزيل",
        "A CSV file, web page or PDF is a copy: the workbook keeps its own file." => {
            "ملف CSV أو صفحة الويب أو PDF نسخة فقط: يحتفظ المصنف بملفه الخاص."
        }
        "Save the workbook under another name or in another folder." => "حفظ المصنف باسم آخر أو في مجلد آخر.",
        "Print Active Sheets" => "طباعة الأوراق النشطة",
        "Print Entire Workbook" => "طباعة المصنف بأكمله",
        "Fit Sheet on One Page" => "احتواء الورقة في صفحة واحدة",
        "Page Setup…" => "إعداد الصفحة…",
        "Page {page} of {pages}" => "صفحة {page} من {pages}",
        "Nothing to print." => "لا يوجد ما يُطبع.",
        "Previous page" => "الصفحة السابقة",
        "Next page" => "الصفحة التالية",
        "Create PDF" => "إنشاء PDF",
        "Save the sheet as a PDF document that keeps its page layout." => "حفظ الورقة كمستند PDF يحافظ على تخطيط صفحاتها.",
        "Change File Type" => "تغيير نوع الملف",
        "CSV (Comma delimited)" => "CSV (مفصول بفواصل)",
        "Save the active sheet as comma-separated text." => "حفظ الورقة النشطة كنص مفصول بفواصل.",
        "Web Page (.html)" => "صفحة ويب (.html)",
        "Save the workbook as a web page." => "حفظ المصنف كصفحة ويب.",
        "Options|General" => "عام",
        "Options|Save" => "حفظ",
        "Options|Editing" => "تحرير",
        "Show formula bar" => "إظهار شريط الصيغة",
        "Enable AutoComplete for cell values" => "تمكين الإكمال التلقائي لقيم الخلايا",
        "Save .xlsx workbooks automatically after every change" => "حفظ مصنفات .xlsx تلقائيًا بعد كل تغيير",
        "Workbook calculation" => "حساب المصنف",
        "Enable iterative calculation" => "تمكين الحساب التكراري",
        "Calculation settings are saved with the workbook." => "تُحفظ إعدادات الحساب مع المصنف.",
        "Arabic workbooks" => "مصنفات عربية",
        "New workbooks start right-to-left with Arial 11 as the default font" => "تبدأ المصنفات الجديدة من اليمين إلى اليسار وبخط Arial 11 افتراضيًا",
        // Dialog buttons and titles.
        "OK" => "موافق",
        "Cancel" => "إلغاء",
        "Don't Save" => "عدم الحفظ",
        "Open…" => "فتح…",
        "Show password" => "إظهار كلمة المرور",
        "Start a new workbook" => "بدء مصنف جديد",
        "Household Budget" => "ميزانية المنزل",
        "Quarterly Sales Dashboard" => "لوحة مبيعات ربع سنوية",
        "Gradebook" => "سجل الدرجات",
        "this workbook" => "هذا المصنف",
        "This workbook" => "هذا المصنف",
        "Do you want to save the changes you made to {title}?" => "هل تريد حفظ التغييرات التي أجريتها على {title}؟",
        "Save a copy as .{format}" => "حفظ نسخة بتنسيق .{format}",
        "Select a chart first." => "حدد مخططًا أولًا.",
        "Text Box" => "مربع نص",
        "Select Data" => "تحديد البيانات",
        "Row Height" => "ارتفاع الصف",
        "Column Width" => "عرض العمود",
        "Standard Width" => "العرض القياسي",
        "Rename Sheet" => "إعادة تسمية الورقة",
        "Move or Copy" => "نقل أو نسخ",
        "Protect Sheet" => "حماية الورقة",
        "Protect Structure and Windows" => "حماية البنية والنوافذ",
        "Unprotect Workbook" => "إلغاء حماية المصنف",
        "Unprotect Sheet" => "إلغاء حماية الورقة",
        "Password" => "كلمة المرور",
        "Unhide" => "إظهار",
        "Paste Special" => "لصق خاص",
        "Series" => "سلسلة",
        "New Comment" => "تعليق جديد",
        "Insert Hyperlink" => "إدراج ارتباط تشعبي",
        "Create Sparklines" => "إنشاء خطوط المؤشر",
        "Page Setup" => "إعداد الصفحة",
        "Convert Text to Columns" => "تحويل النص إلى أعمدة",
        "Remove Duplicates" => "إزالة التكرارات",
        "Goal Seek" => "استهداف القيمة",
        "Create Forecast Worksheet" => "إنشاء ورقة تنبؤ",
        "Solver Parameters" => "معلمات Solver",
        "Solver Results" => "نتائج Solver",
        "Edit Name" => "تحرير الاسم",
        "New Name" => "اسم جديد",
        "Format Cells" => "تنسيق الخلايا",
        "Formula Builder" => "منشئ الصيغ",
        "Find and Replace" => "بحث واستبدال",
        "Go To" => "الانتقال إلى",
        "Insert Calculated Field" => "إدراج حقل محسوب",
        "Go To Special" => "الانتقال إلى خاص",
        "Name Manager" => "إدارة الأسماء",
        "Conditional Formatting Rules Manager" => "إدارة قواعد التنسيق الشرطي",
        "Search Commands" => "البحث عن الأوامر",
        "Agent Control" => "تحكم الوكيل",
        "About GridCraft" => "حول GridCraft",
        "Insert Chart" => "إدراج مخطط",
        "Select Data Source" => "تحديد مصدر البيانات",
        "Action Journal" => "سجل الإجراءات",
        "Macros" => "وحدات الماكرو",
        "Workbook Statistics" => "إحصائيات المصنف",
        "Accessibility" => "إمكانية الوصول",
        "Chart Title" => "عنوان المخطط",
        "Place Picture in Cell" => "وضع صورة في الخلية",
        "Place Picture over Cells" => "وضع صورة فوق الخلايا",
        "Pick From List" => "اختيار من قائمة",
        "Save a Copy" => "حفظ نسخة",
        "Save Changes?" => "حفظ التغييرات؟",
        // Insert / delete cells.
        "Shift cells right" => "إزاحة الخلايا لليمين",
        "Shift cells down" => "إزاحة الخلايا لأسفل",
        "Shift cells left" => "إزاحة الخلايا لليسار",
        "Shift cells up" => "إزاحة الخلايا لأعلى",
        "Entire row" => "صف بأكمله",
        "Entire column" => "عمود بأكمله",
        "Row height (pixels):" => "ارتفاع الصف (بالبكسل):",
        "Column width (characters):" => "عرض العمود (بالأحرف):",
        "Standard column width (pixels):" => "عرض العمود القياسي (بالبكسل):",
        "Name:" => "الاسم:",
        "Before sheet (position, 0 = first):" => "قبل الورقة (الموضع، 0 = الأولى):",
        "Create a copy" => "إنشاء نسخة",
        "Sheets: {sheets}" => "الأوراق: {sheets}",
        // Protection.
        "Password (optional):" => "كلمة المرور (اختياري):",
        "Password:" => "كلمة المرور:",
        "Allow all users of this sheet to:" => "السماح لكل مستخدمي هذه الورقة بـ:",
        "Format cells" => "تنسيق الخلايا",
        "Format columns" => "تنسيق الأعمدة",
        "Format rows" => "تنسيق الصفوف",
        "Insert rows" => "إدراج صفوف",
        "Delete rows" => "حذف صفوف",
        "Use AutoFilter" => "استخدام التصفية التلقائية",
        " (password, unlocked)" => " (كلمة مرور، غير مؤمّن)",
        " (password)" => " (كلمة مرور)",
        "No ranges yet." => "لا توجد نطاقات بعد.",
        "Ranges that stay editable when the sheet is protected:" => "النطاقات التي تبقى قابلة للتحرير عند حماية الورقة:",
        "Action" => "إجراء",
        "Modify" => "تعديل",
        "Unlock with password" => "إلغاء التأمين بكلمة مرور",
        "Title:" => "العنوان:",
        "Refers to cells:" => "يشير إلى الخلايا:",
        "Range password:" => "كلمة مرور النطاق:",
        "New title (Modify):" => "عنوان جديد (تعديل):",
        "Remove the password (Modify)" => "إزالة كلمة المرور (تعديل)",
        "'{file}' is protected." => "'{file}' محمي.",
        "Unhide sheet:" => "إظهار الورقة:",
        "Hidden: {sheets}" => "المخفية: {sheets}",
        // Paste Special.
        "All" => "الكل",
        "Formulas (F)" => "الصيغ (F)",
        "Values (V)" => "القيم (V)",
        "Formats (T)" => "التنسيقات (T)",
        "Comments and notes (C)" => "التعليقات والملاحظات (C)",
        "Validation" => "التحقق من الصحة",
        "All except borders" => "الكل باستثناء الحدود",
        "Column widths" => "عرض الأعمدة",
        "Formulas and number formats" => "الصيغ وتنسيقات الأرقام",
        "Values and number formats" => "القيم وتنسيقات الأرقام",
        "Operation" => "العملية",
        "Add" => "إضافة",
        "Subtract" => "طرح",
        "Multiply" => "ضرب",
        "Divide" => "قسمة",
        "Skip blanks" => "تخطي الفراغات",
        "Transpose (E)" => "تبديل الموضع (E)",
        // Series.
        "Series in" => "السلسلة في",
        "Columns" => "أعمدة",
        "Rows" => "صفوف",
        "Type" => "النوع",
        "Linear" => "خطي",
        "Growth" => "نمو",
        "Date unit" => "وحدة التاريخ",
        "Day" => "يوم",
        "Weekday" => "يوم عمل",
        "Month" => "شهر",
        "Year" => "سنة",
        "Step value:" => "قيمة الخطوة:",
        "Stop value:" => "قيمة التوقف:",
        // Comments, links, sparklines, page setup.
        "Start a conversation:" => "ابدأ محادثة:",
        "Note:" => "ملاحظة:",
        "Display:" => "النص المعروض:",
        "Link to (web address, e-mail or Sheet!A1):" => "ارتباط إلى (عنوان ويب أو بريد إلكتروني أو Sheet!A1):",
        "ScreenTip:" => "تلميح الشاشة:",
        "Data Range:" => "نطاق البيانات:",
        "Location Range:" => "نطاق الموقع:",
        "Markers" => "علامات",
        "Footer:" => "التذييل:",
        "Left margin (in):" => "الهامش الأيسر (بوصة):",
        "Right margin (in):" => "الهامش الأيمن (بوصة):",
        "Top margin (in):" => "الهامش العلوي (بوصة):",
        "Bottom margin (in):" => "الهامش السفلي (بوصة):",
        "Header (in):" => "الرأس (بوصة):",
        "Footer (in):" => "التذييل (بوصة):",
        // Text to Columns.
        "Tab" => "علامة جدولة",
        "Semicolon" => "فاصلة منقوطة",
        "Comma" => "فاصلة",
        "Space" => "مسافة",
        "Other:" => "أخرى:",
        "Treat consecutive delimiters as one" => "اعتبار المحددات المتتالية محددًا واحدًا",
        "Destination:" => "الوجهة:",
        "My data has headers" => "تحتوي بياناتي على رؤوس",
        "Columns (blank = all, e.g. A,C):" => "الأعمدة (فارغ = الكل، مثلًا A,C):",
        // Forecast sheet.
        "Timeline range (blank: first column of the selection):" => "نطاق الخط الزمني (فارغ: العمود الأول من التحديد):",
        "Values range (blank: second column of the selection):" => "نطاق القيم (فارغ: العمود الثاني من التحديد):",
        "Forecast end (date or number; blank: a third of the history):" => "نهاية التنبؤ (تاريخ أو رقم؛ فارغ: ثلث السجل):",
        "Confidence interval" => "فاصل الثقة",
        "Confidence level (%):" => "مستوى الثقة (%):",
        "Seasonality (auto, 0 for none, or a season length):" => "الموسمية (تلقائي، أو 0 لبلا، أو طول الموسم):",
        "Fill missing points using" => "تعبئة النقاط المفقودة باستخدام",
        "Interpolation" => "الاستيفاء",
        "Zeros" => "أصفار",
        "Aggregate duplicates using" => "تجميع التكرارات باستخدام",
        "Average" => "المتوسط",
        "Count" => "العدد",
        "CountA" => "عدد القيم (CountA)",
        "Max" => "الحد الأقصى",
        "Median" => "الوسيط",
        "Min" => "الحد الأدنى",
        "Sum" => "المجموع",
        "Product" => "حاصل الضرب",
        "Include forecast statistics" => "تضمين إحصائيات التنبؤ",
        "Chart" => "مخطط",
        "Line chart" => "مخطط خطي",
        "Column chart" => "مخطط عمودي",
        // Subtotal.
        "At each change in (column letter):" => "عند كل تغيير في (حرف العمود):",
        "Use function" => "استخدام الدالة",
        "Add subtotal to (columns, e.g. C,D):" => "إضافة مجموع فرعي إلى (الأعمدة، مثلًا C,D):",
        // Zoom and names.
        "Magnification" => "التكبير",
        "Custom (%):" => "مخصص (%):",
        "Scope (Workbook or sheet name):" => "النطاق (المصنف أو اسم الورقة):",
        "Comment:" => "التعليق:",
        "Refers to:" => "يشير إلى:",
        // Data validation.
        "Allow" => "السماح",
        "Any value" => "أي قيمة",
        "Whole number" => "عدد صحيح",
        "Decimal" => "عدد عشري",
        "List" => "قائمة",
        "Text length" => "طول النص",
        "between" => "بين",
        "not between" => "ليس بين",
        "equal to" => "يساوي",
        "not equal to" => "لا يساوي",
        "greater than" => "أكبر من",
        "less than" => "أصغر من",
        "greater than or equal to" => "أكبر من أو يساوي",
        "less than or equal to" => "أصغر من أو يساوي",
        "Minimum / Source / Formula:" => "الحد الأدنى / المصدر / الصيغة:",
        "Maximum:" => "الحد الأقصى:",
        "Ignore blank" => "تجاهل الفراغات",
        "In-cell dropdown" => "قائمة منسدلة داخل الخلية",
        "Input message:" => "رسالة الإدخال:",
        "Error style" => "نمط الخطأ",
        "Stop" => "إيقاف",
        "Warning" => "تحذير",
        "Information" => "معلومات",
        "Error message:" => "رسالة الخطأ:",
        // Quick conditional formatting rules.
        "Format cells that are (value):" => "تنسيق الخلايا التي قيمتها:",
        "and:" => "و:",
        "Format cells that contain the text:" => "تنسيق الخلايا التي تحتوي على النص:",
        "Format cells that contain a date occurring:" => "تنسيق الخلايا التي تحتوي على تاريخ يقع:",
        "Today" => "اليوم",
        "Tomorrow" => "غدًا",
        "In the last 7 days" => "في آخر 7 أيام",
        "Last week" => "الأسبوع الماضي",
        "This week" => "هذا الأسبوع",
        "Next week" => "الأسبوع القادم",
        "Last month" => "الشهر الماضي",
        "This month" => "هذا الشهر",
        "Next month" => "الشهر القادم",
        "Format values where this formula is true:" => "تنسيق القيم التي تكون فيها هذه الصيغة صحيحة:",
        "with" => "باستخدام",
        "Light Red Fill with Dark Red Text" => "تعبئة حمراء فاتحة مع نص أحمر داكن",
        "Yellow Fill with Dark Yellow Text" => "تعبئة صفراء مع نص أصفر داكن",
        "Green Fill with Dark Green Text" => "تعبئة خضراء مع نص أخضر داكن",
        "Light Red Fill" => "تعبئة حمراء فاتحة",
        "Red Text" => "نص أحمر",
        "Red Border" => "حد أحمر",
        "Formula:" => "الصيغة:",
        // Go To Special.
        "Select" => "تحديد",
        "Blanks" => "الفراغات",
        "Errors" => "الأخطاء",
        "Numbers" => "الأرقام",
        "Current region" => "المنطقة الحالية",
        "Last cell" => "الخلية الأخيرة",
        "Visible cells only" => "الخلايا المرئية فقط",
        "Conditional formats" => "التنسيقات الشرطية",
        "Data validation" => "التحقق من صحة البيانات",
        // Charts and pictures.
        "Horizontal axis title:" => "عنوان المحور الأفقي:",
        "Vertical axis title:" => "عنوان المحور العمودي:",
        "No picture selected" => "لم يتم تحديد صورة",
        "Picture file path (PNG or JPEG):" => "مسار ملف الصورة (PNG أو JPEG):",
        "Description (optional):" => "الوصف (اختياري):",
        "Fits inside the selected cell and moves with its content." => "تُحتوى داخل الخلية المحددة وتتحرك مع محتواها.",
        "Choose a PNG or JPEG no larger than 16 MB." => "اختر صورة PNG أو JPEG لا يتجاوز حجمها 16 ميغابايت.",
        // About, macros, journal.
        "About" => "حول",
        "Contributors" => "المساهمون",
        "Models" => "النماذج",
        "Version {version}" => "الإصدار {version}",
        "Join the ArtCraft Discord" => "انضم إلى Discord الخاص بـ ArtCraft",
        "This workbook has no scripts or macros." => "لا يحتوي هذا المصنف على برامج نصية أو وحدات ماكرو.",
        "(unnamed)" => "(بلا اسم)",
        "— VBA project (preserved, not run)" => "— مشروع VBA (محفوظ، لا يُشغَّل)",
        "— 1 step" => "— خطوة واحدة",
        "— {steps} steps" => "— {steps} خطوات",
        // Format Cells.
        "Alignment" => "محاذاة",
        "Font" => "خط",
        "Border" => "حدود",
        "Format Cells|Fill" => "تعبئة",
        "Protection" => "حماية",
        "Category:" => "الفئة:",
        "Sample" => "نموذج",
        "Type (custom format code):" => "النوع (رمز التنسيق المخصص):",
        "Vertical" => "عمودي",
        "Justify" => "ضبط",
        "Center Across Selection" => "توسيط عبر التحديد",
        "Distributed" => "موزع",
        "Top" => "أعلى",
        "Bottom" => "أسفل",
        "Indent" => "مسافة بادئة",
        "Orientation (degrees)" => "الاتجاه (بالدرجات)",
        "Wrap text" => "التفاف النص",
        "Shrink to fit" => "تقليص للاحتواء",
        "Bold" => "غامق",
        "Italic" => "مائل",
        "None" => "بلا",
        "Single" => "مفرد",
        "Double" => "مزدوج",
        "Single Accounting" => "محاسبي مفرد",
        "Double Accounting" => "محاسبي مزدوج",
        "Superscript" => "مرتفع",
        "Subscript" => "منخفض",
        "Color:" => "اللون:",
        "Outline" => "مخطط تفصيلي",
        "Diagonal ↘" => "قطري ↘",
        "Diagonal ↗" => "قطري ↗",
        "none" => "بلا",
        "thin" => "رفيع",
        "medium" => "متوسط",
        "thick" => "سميك",
        "dashed" => "متقطع",
        "dotted" => "منقّط",
        "double" => "مزدوج",
        "Background Color:" => "لون الخلفية:",
        "No Color" => "بلا لون",
        "Locked" => "مؤمّنة",
        "Hidden" => "مخفية",
        "Locking cells or hiding formulas has no effect until you protect the worksheet (Review › Protect Sheet)." => {
            "لا يكون لتأمين الخلايا أو إخفاء الصيغ أي تأثير حتى تحمي ورقة العمل (مراجعة › حماية الورقة)."
        }
        // Formula Builder.
        "Search:" => "بحث:",
        "e.g. lookup, average, date" => "مثلًا: بحث، متوسط، تاريخ",
        "Date & Time" => "التاريخ والوقت",
        "Lookup & Reference" => "البحث والمراجع",
        "Math & Trig" => "رياضيات ومثلثات",
        "Statistical" => "إحصائية",
        "Engineering" => "هندسية",
        "Database" => "قاعدة بيانات",
        "Compatibility" => "التوافق",
        "Web" => "ويب",
        "Cube" => "مكعب",
        "Add-in and Automation" => "الوظائف الإضافية والأتمتة",
        // Find and Replace, Go To.
        "Find" => "بحث",
        "Replace" => "استبدال",
        "Find what:" => "البحث عن:",
        "Replace with:" => "استبدال بـ:",
        "Match case" => "مطابقة حالة الأحرف",
        "Match entire cell contents" => "مطابقة محتويات الخلية بالكامل",
        "Within workbook" => "ضمن المصنف",
        "Replace All" => "استبدال الكل",
        "Find All" => "بحث عن الكل",
        "Find Next" => "بحث عن التالي",
        "All done. We made {count} replacements." => "تم الانتهاء. أجرينا {count} استبدالًا.",
        "{count} cell(s) found" => "تم العثور على {count} خلية",
        "Reference:" => "المرجع:",
        "Special…" => "خاص…",
        // Sort.
        "My list has headers" => "تحتوي قائمتي على رؤوس",
        "Sort by" => "فرز حسب",
        "Then by" => "ثم حسب",
        "Cell Values" => "قيم الخلايا",
        "Cell Color" => "لون الخلية",
        "A to Z" => "من أ إلى ي",
        "Z to A" => "من ي إلى أ",
        "+ Add Level" => "+ إضافة مستوى",
        // Name Manager.
        "New…" => "جديد…",
        "Edit…" => "تحرير…",
        "Name" => "الاسم",
        "Value" => "القيمة",
        "Refers To" => "يشير إلى",
        "Scope" => "النطاق",
        "Workbook" => "المصنف",
        "No names defined" => "لم تُعرَّف أسماء",
        // Insert Chart.
        "Recommended Charts" => "المخططات المستحسنة",
        "All Charts" => "كل المخططات",
        "Clustered Column" => "عمودي متفاوت",
        "Stacked Column" => "عمودي مكدس",
        "100% Stacked Column" => "عمودي مكدس 100%",
        "Clustered Bar" => "شريطي متفاوت",
        "Stacked Bar" => "شريطي مكدس",
        "100% Stacked Bar" => "شريطي مكدس 100%",
        "Line" => "خطي",
        "Line with Markers" => "خطي مع علامات",
        "Stacked Line" => "خطي مكدس",
        "Pie" => "دائري",
        "Doughnut" => "دائري مجوف",
        "Area" => "مساحي",
        "Stacked Area" => "مساحي مكدس",
        "Scatter" => "مبعثر",
        "Scatter with Lines" => "مبعثر مع خطوط",
        "Radar" => "نسيجي",
        "Histogram" => "مدرج تكراري",
        "Waterfall" => "انحداري",
        "Funnel" => "قمعي",
        "Combo" => "مختلط",
        "Column or Bar" => "عمودي أو شريطي",
        "Line or Area" => "خطي أو مساحي",
        "Pie or Doughnut" => "دائري أو دائري مجوف",
        "Scatter or Bubble" => "مبعثر أو فقاعي",
        "Statistic" => "إحصائي",
        "Hierarchy & more" => "هرمي والمزيد",
        // Select Data Source.
        "Chart data range:" => "نطاق بيانات المخطط:",
        "Legend Entries (Series)" => "إدخالات وسيلة الإيضاح (السلاسل)",
        "Series{n}" => "سلسلة{n}",
        "Keep" => "إبقاء",
        "Remove" => "إزالة",
        "Horizontal (Category) Axis Labels:" => "تسميات المحور الأفقي (الفئات):",
        // Rules manager, command palette, goal seek.
        "There are no conditional formatting rules on this sheet." => "لا توجد قواعد تنسيق شرطي في هذه الورقة.",
        "applies to" => "تنطبق على",
        "Type a command, e.g. freeze, chart, bold…" => "اكتب أمرًا، مثلًا: تجميد، مخطط، غامق…",
        "Set cell:" => "تعيين الخلية:",
        "To value:" => "إلى القيمة:",
        "By changing cell:" => "بتغيير الخلية:",
        "Goal Seeking found a solution: {value}" => "وجد استهداف القيمة حلًا: {value}",
        // Data Analysis tools.
        "Analysis Tools" => "أدوات التحليل",
        "Anova: Single Factor" => "تحليل التباين (Anova): عامل واحد",
        "Correlation" => "الارتباط",
        "Covariance" => "التغاير",
        "Descriptive Statistics" => "الإحصاء الوصفي",
        "Exponential Smoothing" => "التجانس الأسي",
        "F-Test Two-Sample for Variances" => "اختبار F لعينتين للتباين",
        "Moving Average" => "المتوسط المتحرك",
        "Rank and Percentile" => "الترتيب والنسبة المئوية",
        "Regression" => "الانحدار",
        "t-Test: Paired Two Sample for Means" => "اختبار t: عينتان مزدوجتان للمتوسطات",
        "t-Test: Two-Sample Assuming Equal Variances" => "اختبار t: عينتان بافتراض تساوي التباين",
        "t-Test: Two-Sample Assuming Unequal Variances" => "اختبار t: عينتان بافتراض اختلاف التباين",
        "Output options" => "خيارات الإخراج",
        "Output Range:" => "نطاق الإخراج:",
        "New Worksheet Ply:" => "ورقة عمل جديدة:",
        "Grouped By:" => "مجمّعة حسب:",
        "Labels in first row" => "التسميات في الصف الأول",
        "Input Range:" => "نطاق الإدخال:",
        "Variable 1 Range:" => "نطاق المتغير 1:",
        "Variable 2 Range:" => "نطاق المتغير 2:",
        "Summary statistics" => "إحصائيات موجزة",
        "Confidence Level for Mean (%), blank for none:" => "مستوى الثقة للمتوسط (%)، فارغ لبلا:",
        "Kth Largest (blank for none):" => "الأكبر رقم K (فارغ لبلا):",
        "Kth Smallest (blank for none):" => "الأصغر رقم K (فارغ لبلا):",
        "Bin Range (blank for automatic bins):" => "نطاق الفئات (فارغ لفئات تلقائية):",
        "Pareto (sorted histogram)" => "باريتو (مدرج تكراري مرتّب)",
        "Cumulative Percentage" => "النسبة المئوية التراكمية",
        "Alpha:" => "ألفا:",
        "Input Y Range:" => "نطاق إدخال Y:",
        "Input X Range:" => "نطاق إدخال X:",
        "Constant is Zero" => "الثابت يساوي صفرًا",
        "Confidence Level (%):" => "مستوى الثقة (%):",
        "Residuals" => "البواقي",
        "Standardized Residuals" => "البواقي المعيارية",
        "Normal Probability" => "الاحتمال الطبيعي",
        "Interval:" => "الفاصل:",
        "Standard Errors" => "الأخطاء المعيارية",
        "Damping factor:" => "عامل التخميد:",
        "Hypothesized Mean Difference:" => "فرق المتوسط المفترض:",
        "The output range already holds data. Replace it with the report?" => "يحتوي نطاق الإخراج على بيانات بالفعل. هل تريد استبدالها بالتقرير؟",
        // Solver.
        "Set Objective:" => "تعيين الهدف:",
        "To:" => "إلى:",
        "Value Of:" => "قيمة:",
        "By Changing Variable Cells:" => "بتغيير خلايا المتغيرات:",
        "Subject to the Constraints:" => "وفق القيود:",
        "Make Unconstrained Variables Non-Negative" => "جعل المتغيرات غير المقيدة غير سالبة",
        "Select a Solving Method:" => "اختر طريقة الحل:",
        "GRG Nonlinear" => "GRG غير خطي",
        "Solve" => "حل",
        "Keep Solver Solution" => "الاحتفاظ بحل Solver",
        "Restore Original Values" => "استعادة القيم الأصلية",
        // Spelling.
        "The spelling check is complete for the entire sheet." => "اكتمل التدقيق الإملائي للورقة بأكملها.",
        "Not in Dictionary ({cell}):" => "ليست في القاموس ({cell}):",
        "Suggestions:" => "الاقتراحات:",
        "(No suggestions)" => "(لا توجد اقتراحات)",
        "Ignore Once" => "تجاهل مرة واحدة",
        "Add to Dictionary" => "إضافة إلى القاموس",
        "Change" => "تغيير",
        "Change All" => "تغيير الكل",
        // Currencies.
        "$ English (United States)" => "$ الإنجليزية (الولايات المتحدة)",
        "£ English (United Kingdom)" => "£ الإنجليزية (المملكة المتحدة)",
        "¥ Japanese" => "¥ اليابانية",
        "€ Euro" => "€ اليورو",
        // Ribbon tips and labels without a separate catalog line.
        "Add Chart Element" => "إضافة عنصر مخطط",
        "Align Left" => "محاذاة لليسار",
        "Align Right" => "محاذاة لليمين",
        "Bottom Align" => "محاذاة لأسفل",
        "Calculate Now (F9)" => "احسب الآن (F9)",
        "Calculate Sheet (⇧F9)" => "حساب الورقة (⇧F9)",
        "Calculation Options" => "خيارات الحساب",
        "Cell Styles" => "أنماط الخلايا",
        "Change Chart Type" => "تغيير نوع المخطط",
        "Check Accessibility" => "التحقق من إمكانية الوصول",
        "Clear Filter" => "مسح التصفية",
        "Comments pane" => "جزء التعليقات",
        "Convert the last ink stroke to a shape" => "تحويل آخر ضربة حبر إلى شكل",
        "Convert to Range" => "تحويل إلى نطاق",
        "Copy" => "نسخ",
        "Custom Sort" => "فرز مخصص",
        "Cut" => "قص",
        "Define Name…" => "تعريف اسم…",
        "Delete Comment" => "حذف التعليق",
        "Delete Sheet" => "حذف الورقة",
        "Delete chart" => "حذف المخطط",
        "Delete…" => "حذف…",
        "Dismiss" => "تجاهل",
        "Down" => "لأسفل",
        "Up" => "لأعلى",
        "Edit chart title" => "تحرير عنوان المخطط",
        "Erase ink strokes" => "محو ضربات الحبر",
        "Every command run in this session (replayable)" => "كل أمر نُفِّذ في هذه الجلسة (قابل لإعادة التشغيل)",
        "Expand formula bar" => "توسيع شريط الصيغة",
        "Filter (⌘⇧F)" => "تصفية (⌘⇧F)",
        "Filter by Selected Cell's Value" => "تصفية حسب قيمة الخلية المحددة",
        "Find & Select" => "بحث وتحديد",
        "Flash Fill (⌘E)" => "تعبئة سريعة (⌘E)",
        "Forecast Sheet" => "ورقة التنبؤ",
        "Format Chart pane" => "جزء تنسيق المخطط",
        "Format as Table" => "تنسيق كجدول",
        "Hide" => "إخفاء",
        "Hide Sheet" => "إخفاء الورقة",
        "Highlighter" => "قلم تمييز",
        "How agents drive GridCraft (MCP / control channel)" => "كيف يتحكم الوكلاء في GridCraft (MCP / قناة التحكم)",
        "Import a text or CSV file" => "استيراد ملف نصي أو CSV",
        "Insert Function" => "إدراج دالة",
        "Insert Function (⇧F3)" => "إدراج دالة (⇧F3)",
        "Insert Link (⌘K)" => "إدراج ارتباط (⌘K)",
        "Insert Sheet" => "إدراج ورقة",
        "Insert Slicer" => "إدراج مقسم طريقة العرض",
        "Insert\nSlicer" => "إدراج\nمقسم العرض",
        "Insert a picture" => "إدراج صورة",
        "Insert a slicer to filter a table" => "إدراج مقسم طريقة عرض لتصفية جدول",
        "Insert an icon" => "إدراج أيقونة",
        "Insert…" => "إدراج…",
        "Link…" => "ارتباط…",
        "Macros and scripts in this workbook (Alt+F8)" => "وحدات الماكرو والبرامج النصية في هذا المصنف (Alt+F8)",
        "Middle Align" => "محاذاة للوسط",
        "More commands" => "أوامر إضافية",
        "Move or Copy Sheet…" => "نقل الورقة أو نسخها…",
        "New sheet (⇧F11)" => "ورقة جديدة (⇧F11)",
        "Next sheet" => "الورقة التالية",
        "Previous sheet" => "الورقة السابقة",
        "Page Breaks" => "فواصل الصفحات",
        "Paper Size" => "حجم الورق",
        "Paste (⌘V)" => "لصق (⌘V)",
        "Pick From Drop-down List…" => "اختيار من القائمة المنسدلة…",
        "Print Area" => "ناحية الطباعة",
        "Print Titles" => "عناوين الطباعة",
        "Protect Workbook structure" => "حماية بنية المصنف",
        "Redo (⌘Y)" => "إعادة (⌘Y)",
        "Undo (⌘Z)" => "تراجع (⌘Z)",
        "Undo list" => "قائمة التراجع",
        "Save (⌘S)" => "حفظ (⌘S)",
        "Search and run any command" => "ابحث عن أي أمر ونفّذه",
        "Search commands (⌘⇧Space)" => "البحث عن الأوامر (⌘⇧Space)",
        "Select and move ink and shapes" => "تحديد الحبر والأشكال ونقلها",
        "Series…" => "سلسلة…",
        "Share: export or save a copy" => "مشاركة: تصدير أو حفظ نسخة",
        "Show Formulas (⌃`)" => "إظهار الصيغ (⌃`)",
        "Slicer" => "مقسم طريقة العرض",
        "Sort & Filter" => "فرز وتصفية",
        "Spelling (F7)" => "تدقيق إملائي (F7)",
        "Switch Row/Column" => "تبديل الصف/العمود",
        "Table (⌘T)" => "جدول (⌘T)",
        "Table Styles" => "أنماط الجدول",
        "Text to Columns" => "تحويل النص إلى أعمدة",
        "Top Align" => "محاذاة لأعلى",
        "Unhide Sheet…" => "إظهار الورقة…",
        "Version, contributors and the AI models that helped" => "الإصدار والمساهمون ونماذج الذكاء الاصطناعي التي ساعدت",
        "What-If Analysis" => "تحليل ماذا لو",
        "Zoom In" => "تكبير",
        "Zoom Out" => "تصغير",
        "Zoom to 100%" => "تكبير/تصغير إلى 100%",
        "Zoom to Selection" => "تكبير/تصغير حسب التحديد",
        "Zoom…" => "تكبير/تصغير…",
        "AutoSave" => "حفظ تلقائي",
        "Pen\nBlack" => "قلم\nأسود",
        "Pen\nBlue" => "قلم\nأزرق",
        "Pen\nRed" => "قلم\nأحمر",
        "Protect\nSheet" => "حماية\nالورقة",
        "Watch\nWindow" => "نافذة\nالمراقبة",
        "Workbook\nStatistics" => "إحصائيات\nالمصنف",
        // Sheet tab menu and status bar.
        "Rename" => "إعادة تسمية",
        "Move or Copy…" => "نقل أو نسخ…",
        "Duplicate" => "تكرار",
        "Tab Color" => "لون علامة التبويب",
        "Unhide…" => "إظهار…",
        "Unprotect Sheet…" => "إلغاء حماية الورقة…",
        "Select All Sheets" => "تحديد كل الأوراق",
        "All sheets" => "كل الأوراق",
        "Macros preserved (not executed)" => "وحدات الماكرو محفوظة (غير مُشغَّلة)",
        "Sum: {}" => "المجموع: {}",
        "Count: {}" => "العدد: {}",
        "Average: {}" => "المتوسط: {}",
        // PivotTable pane.
        "Calculated Field…" => "حقل محسوب…",
        "Choose fields to add to report:" => "اختر الحقول لإضافتها إلى التقرير:",
        "Compact" => "مضغوط",
        "Tabular" => "جدولي",
        "Filters" => "عوامل التصفية",
        "Values" => "القيم",
        "Group by Months" => "تجميع حسب الأشهر",
        "Group by Quarters" => "تجميع حسب الأرباع",
        "Group by Years" => "تجميع حسب السنوات",
        "Move to Column Labels" => "نقل إلى تسميات الأعمدة",
        "Move to Report Filter" => "نقل إلى عامل تصفية التقرير",
        "Move to Row Labels" => "نقل إلى تسميات الصفوف",
        "Move to Values" => "نقل إلى القيم",
        "PivotTable Fields" => "حقول الجدول المحوري",
        "Refresh" => "تحديث",
        "Remove Field" => "إزالة الحقل",
        "Report Layout" => "تخطيط التقرير",
        "Show as % of Grand Total" => "إظهار كنسبة مئوية من الإجمالي الكلي",
        "Show as No Calculation" => "إظهار بلا حساب",
        "Show as Rank" => "إظهار كترتيب",
        "Show as Running Total" => "إظهار كإجمالي تراكمي",
        "Summarize by Average" => "تلخيص بالمتوسط",
        "Summarize by Count" => "تلخيص بالعدد",
        "Summarize by Max" => "تلخيص بالحد الأقصى",
        "Summarize by Min" => "تلخيص بالحد الأدنى",
        "Summarize by Sum" => "تلخيص بالمجموع",
        "Use a field's menu to move it between areas:" => "استخدم قائمة الحقل لنقله بين المناطق:",
        // Kashida (justified Arabic text) — Home › Alignment.
        "Kashida" => "كشيدة",
        "Low" => "منخفضة",
        "Medium" => "متوسطة",
        "High" => "مرتفعة",
        "Off" => "بلا",
        _ => return explain(text),
    })
}

/// The UI kit's explanations (crates/ui-egui/src/ui_kit/*.rs), one Arabic sentence per entry —
/// kept apart from the labels so the two can be checked separately.
fn explain(text: &str) -> Option<&'static str> {
    Some(match text {
        "Puts the cells you cut or copied into the sheet, starting at the selected cell. Example: copy A1:A3, click C1 and paste; the three values appear in C1:C3." => {
            "يضع الخلايا التي قصصتها أو نسختها في الورقة بدءًا من الخلية المحددة. مثال: انسخ A1:A3 ثم انقر C1 والصق؛ تظهر القيم الثلاث في C1:C3."
        }
        "Takes the selected cells out of the sheet and keeps them on the clipboard, so you can paste them somewhere else. Example: cut a column of names and paste it beside another table." => {
            "يزيل الخلايا المحددة من الورقة ويحتفظ بها في الحافظة لتلصقها في مكان آخر. مثال: قصّ عمود الأسماء والصقه بجانب جدول آخر."
        }
        "Keeps a copy of the selected cells on the clipboard to paste elsewhere; the originals stay where they are. Example: copy this month's header row and paste it into next month's sheet." => {
            "يحتفظ بنسخة من الخلايا المحددة في الحافظة لتلصقها في مكان آخر؛ وتبقى الخلايا الأصلية في مكانها. مثال: انسخ صف العناوين لهذا الشهر والصقه في ورقة الشهر القادم."
        }
        "Copies the look of the selected cell (fill, font, borders, number format) and paints it onto the cells you click next; double-click the button to paint many places. Example: click a yellow bold title, then click another title to make it look the same." => {
            "ينسخ مظهر الخلية المحددة (التعبئة والخط والحدود وتنسيق الأرقام) ويطبّقه على الخلايا التي تنقرها بعد ذلك؛ انقر الزر نقرًا مزدوجًا لتطبيقه في مواضع كثيرة. مثال: انقر عنوانًا أصفر غامقًا، ثم انقر عنوانًا آخر ليأخذ المظهر نفسه."
        }
        "Makes the text in the selected cells one step bigger. Example: a title in 11 points becomes 12 points." => {
            "يكبّر النص في الخلايا المحددة درجة واحدة. مثال: عنوان بحجم ١١ نقطة يصبح ١٢ نقطة."
        }
        "Makes the text in the selected cells one step smaller. Example: a title in 12 points becomes 11 points." => {
            "يصغّر النص في الخلايا المحددة درجة واحدة. مثال: عنوان بحجم ١٢ نقطة يصبح ١١ نقطة."
        }
        "Makes the text in the selected cells bold: thicker, darker letters that stand out. Example: the header row of a table is made bold." => {
            "يجعل النص في الخلايا المحددة غامقًا: حروفًا أثخن وأدكن تلفت النظر. مثال: صف العناوين في الجدول يُجعل غامقًا."
        }
        "Slants the text in the selected cells to the side, like handwriting. Example: a note under a table is written slanted." => {
            "يميل النص في الخلايا المحددة إلى الجانب كخط اليد. مثال: ملاحظة تحت الجدول تُكتب مائلة."
        }
        "Draws a line through the middle of the text in the selected cells, as if crossed out. Example: a cancelled order is crossed out but still readable." => {
            "يرسم خطًا في منتصف النص في الخلايا المحددة كأنه مشطوب. مثال: طلب ملغى يُشطب ويبقى مقروءًا."
        }
        "Colours the background of the selected cells. Example: the total row is filled light yellow so it stands out." => {
            "يلوّن خلفية الخلايا المحددة. مثال: صف المجموع يُملأ بالأصفر الفاتح ليبرز."
        }
        "Colours the text in the selected cells; the background stays as it is. Example: the negative numbers are written in red." => {
            "يلوّن النص في الخلايا المحددة؛ وتبقى الخلفية كما هي. مثال: الأرقام السالبة تُكتب بالأحمر."
        }
        "Turns the text in the selected cells: slanted, vertical or rotated, so long headers fit over narrow columns. Example: the column titles are written at an angle." => {
            "يدير النص في الخلايا المحددة: مائلًا أو عموديًا أو مُدارًا، لتتسع العناوين الطويلة فوق الأعمدة الضيقة. مثال: عناوين الأعمدة تُكتب بزاوية."
        }
        "Breaks long text onto several lines inside its cell, making the row taller, instead of letting it spill over the next cells. Example: a long address shows on three lines in one cell." => {
            "يقسم النص الطويل على عدة أسطر داخل خليته فيزيد ارتفاع الصف، بدل أن يمتد فوق الخلايا المجاورة. مثال: عنوان سكن طويل يظهر في ثلاثة أسطر داخل خلية واحدة."
        }
        "Moves the text in the selected cells one step back towards the cell's edge. Example: an indented item moves back in line with the others." => {
            "يرجع النص في الخلايا المحددة خطوة واحدة نحو حافة الخلية. مثال: بند مزاح للداخل يعود إلى صف البنود الأخرى."
        }
        "Moves the text in the selected cells one step away from the cell's edge. Example: the items under a heading are pushed in to show they belong to it." => {
            "يبعد النص في الخلايا المحددة خطوة واحدة عن حافة الخلية. مثال: البنود تحت عنوان تُزاح للداخل لتدل على أنها تابعة له."
        }
        "Shows the selected numbers as percentages: 0.25 becomes 25%. Example: a share of 0.5 shows as 50%." => {
            "يعرض الأرقام المحددة نسبًا مئوية: ٠٫٢٥ يصبح ٢٥٪. مثال: حصة ٠٫٥ تظهر ٥٠٪."
        }
        "Shows the selected numbers with a thousands separator and two decimals. Example: 1250 becomes 1,250.00." => {
            "يعرض الأرقام المحددة بفاصل الآلاف وخانتين عشريتين. مثال: ١٢٥٠ يصبح ١٬٢٥٠٫٠٠."
        }
        "Shows one more digit after the decimal point in the selected cells. Example: 3.1 becomes 3.14." => {
            "يعرض خانة إضافية بعد الفاصلة العشرية في الخلايا المحددة. مثال: ٣٫١ يصبح ٣٫١٤."
        }
        "Shows one digit fewer after the decimal point in the selected cells, rounding what you see. Example: 3.14 becomes 3.1." => {
            "يعرض خانة أقل بعد الفاصلة العشرية في الخلايا المحددة مع تقريب ما تراه. مثال: ٣٫١٤ يصبح ٣٫١."
        }
        "Colours cells automatically by their values, with rules that keep working as the numbers change: highlight the large ones, add bars or colour scales. Example: every sale above 1000 turns green by itself." => {
            "يلوّن الخلايا تلقائيًا بحسب قيمها، بقواعد تظل تعمل كلما تغيرت الأرقام: تمييز الكبيرة منها، أو إضافة أشرطة أو تدرجات لونية. مثال: كل مبيعة تزيد على ١٠٠٠ تتحول إلى الأخضر من تلقاء نفسها."
        }
        "Turns the selected cells into a table with a styled header, banded rows and filter buttons. Example: a plain list of orders becomes a striped table with filter arrows." => {
            "يحوّل الخلايا المحددة إلى جدول بعنوان منسّق وصفوف متناوبة اللون وأزرار تصفية. مثال: قائمة طلبات عادية تصبح جدولًا مخططًا بأسهم تصفية."
        }
        "Applies a ready-made look (title, heading, total, good, bad…) to the selected cells in one click. Example: the last row gets the Total style: bold with a line above it." => {
            "يطبّق مظهرًا جاهزًا (عنوان، عنوان فرعي، مجموع، جيد، سيئ…) على الخلايا المحددة بنقرة واحدة. مثال: الصف الأخير يأخذ نمط المجموع: غامق وفوقه خط."
        }
        "Adds empty cells, rows, columns or a new sheet at the selection, pushing the existing ones aside. Example: insert a row above row 3 to add a forgotten item." => {
            "يضيف خلايا أو صفوفًا أو أعمدة فارغة أو ورقة جديدة عند التحديد، ويزيح الموجود جانبًا. مثال: أدرج صفًا فوق الصف ٣ لإضافة بند منسي."
        }
        "Removes the selected cells, rows, columns or sheet; the others move up or left to close the gap. Example: delete the empty row between two tables." => {
            "يزيل الخلايا أو الصفوف أو الأعمدة أو الورقة المحددة؛ ويتحرك الباقي إلى الأعلى أو الجانب لسدّ الفراغ. مثال: احذف الصف الفارغ بين جدولين."
        }
        "Changes the size of rows and columns, hides or shows them, and manages the sheet: rename, protect, move. Example: make column A wide enough for the names." => {
            "يغيّر حجم الصفوف والأعمدة ويخفيها أو يظهرها، ويدير الورقة: إعادة التسمية والحماية والنقل. مثال: اجعل العمود A واسعًا بما يكفي للأسماء."
        }
        "Copies the selected cell's value or formula into the cells next to it (down, right, up or left), or continues a series. Example: a formula in B2 is filled down to B10." => {
            "ينسخ قيمة الخلية المحددة أو معادلتها إلى الخلايا المجاورة (أسفل أو يمين أو أعلى أو يسار)، أو يكمل سلسلة. مثال: معادلة في B2 تُملأ إلى الأسفل حتى B10."
        }
        "Empties the selected cells: everything, only the formatting, only the contents, or only comments and links. Example: clear the contents but keep the yellow fill." => {
            "يفرغ الخلايا المحددة: كل شيء، أو التنسيق فقط، أو المحتوى فقط، أو التعليقات والروابط فقط. مثال: امسح المحتوى واحتفظ بالتعبئة الصفراء."
        }
        "Puts the rows in order (A to Z, smallest to largest) or shows only the rows that match a condition. Example: sort a list of names alphabetically, or show only the unpaid invoices." => {
            "يرتب الصفوف (من أ إلى ي، من الأصغر إلى الأكبر) أو يعرض الصفوف المطابقة لشرط فقط. مثال: رتّب قائمة أسماء أبجديًا، أو اعرض الفواتير غير المسددة فقط."
        }
        "Searches the sheet for a word or number, replaces it, jumps to a cell, or selects all cells of one kind. Example: find every «بغداد» and replace it with «البصرة»." => {
            "يبحث في الورقة عن كلمة أو رقم، أو يستبدله، أو ينتقل إلى خلية، أو يحدد كل الخلايا من نوع واحد. مثال: ابحث عن كل «بغداد» واستبدلها بـ«البصرة»."
        }
        "Pastes the copied cells as they are: values, formulas and formatting together. Example: a formatted total with its formula comes in complete." => {
            "يلصق الخلايا المنسوخة كما هي: القيم والمعادلات والتنسيق معًا. مثال: مجموع منسّق يأتي كاملًا مع معادلته."
        }
        "Pastes only the results: the numbers and text, without formulas or formatting. Example: a cell showing 150 from =A1*3 is pasted as the plain number 150." => {
            "يلصق النتائج فقط: الأرقام والنصوص، دون المعادلات أو التنسيق. مثال: خلية تعرض ١٥٠ ناتجة من =A1*3 تُلصق رقمًا عاديًا ١٥٠."
        }
        "Pastes only the formulas, without the formatting; they recalculate for their new place. Example: =SUM(A1:A3) pasted into column B becomes =SUM(B1:B3)." => {
            "يلصق المعادلات فقط دون التنسيق؛ ويعاد حسابها في موضعها الجديد. مثال: =SUM(A1:A3) إذا لُصقت في العمود B تصبح =SUM(B1:B3)."
        }
        "Pastes only the look (fill, font, borders, number format), leaving the destination's values as they are. Example: the yellow fill and bold of one header are copied onto another header." => {
            "يلصق المظهر فقط (التعبئة والخط والحدود وتنسيق الأرقام)، ويترك قيم الوجهة كما هي. مثال: التعبئة الصفراء والخط الغامق لعنوان تُنسخ إلى عنوان آخر."
        }
        "Pastes everything with a twist: Transpose turns the copied rows into columns and columns into rows; Paste Link pastes formulas that point back to the copied cells, so they follow every change. Example: a header row A1:D1 becomes a column A1:A4." => {
            "يلصق كل شيء مع تغيير: «تبديل الموضع» يحوّل الصفوف المنسوخة إلى أعمدة والأعمدة إلى صفوف؛ و«لصق ارتباط» يلصق معادلات تشير إلى الخلايا المنسوخة فتتبع كل تغيير فيها. مثال: صف عناوين A1:D1 يصبح عمودًا A1:A4."
        }
        "Opens a box with every paste choice: values, formulas, formats, comments, column widths, transpose, link, or add and multiply with the pasted numbers. Example: paste only the column widths of another table." => {
            "يفتح مربعًا فيه كل خيارات اللصق: القيم أو المعادلات أو التنسيقات أو التعليقات أو عرض الأعمدة أو تبديل الموضع أو الارتباط، أو الجمع والضرب مع الأرقام الملصقة. مثال: الصق عرض أعمدة جدول آخر فقط."
        }
        "Draws one line under the text in the selected cells. Example: a column title with a single line under it." => {
            "يرسم خطًا واحدًا تحت النص في الخلايا المحددة. مثال: عنوان عمود تحته خط واحد."
        }
        "Draws two lines under the text in the selected cells, as accountants mark a final total. Example: the grand total has a double underline." => {
            "يرسم خطين تحت النص في الخلايا المحددة كما يعلّم المحاسبون المجموع النهائي. مثال: المجموع الكلي تحته خط مزدوج."
        }
        "Turns the text in the selected cells to the chosen angle: slanted up or down, vertical letters, rotated up or down, or back to horizontal. Example: a long header is written at an angle over a narrow column." => {
            "يدير النص في الخلايا المحددة إلى الزاوية المختارة: مائلًا للأعلى أو للأسفل، أو حروفًا عمودية، أو مُدارًا للأعلى أو للأسفل، أو يعيده أفقيًا. مثال: عنوان طويل يُكتب بزاوية فوق عمود ضيق."
        }
        "Slants the text upwards, from bottom-left to top-right, at 45 degrees. Example: the month names rise over their narrow columns." => {
            "يميل النص إلى الأعلى، من أسفل اليسار إلى أعلى اليمين، بزاوية ٤٥ درجة. مثال: أسماء الأشهر ترتفع فوق أعمدتها الضيقة."
        }
        "Slants the text downwards, from top-left to bottom-right, at 45 degrees. Example: the month names lean down over their columns." => {
            "يميل النص إلى الأسفل، من أعلى اليسار إلى أسفل اليمين، بزاوية ٤٥ درجة. مثال: أسماء الأشهر تنحدر فوق أعمدتها."
        }
        "Stacks the letters one under the other, upright, in a tall cell. Example: a one-word header reads from top to bottom." => {
            "يرصّ الحروف واحدًا تحت الآخر بوضع قائم في خلية طويلة. مثال: عنوان من كلمة واحدة يُقرأ من الأعلى إلى الأسفل."
        }
        "Rotates the text 90 degrees so it reads from bottom to top. Example: a label beside a tall merged cell is read looking up." => {
            "يدير النص ٩٠ درجة ليُقرأ من الأسفل إلى الأعلى. مثال: تسمية بجانب خلية مدمجة طويلة تُقرأ بالنظر إلى الأعلى."
        }
        "Rotates the text 90 degrees so it reads from top to bottom. Example: a label on the right edge of a table is read looking down." => {
            "يدير النص ٩٠ درجة ليُقرأ من الأعلى إلى الأسفل. مثال: تسمية على الحافة اليمنى للجدول تُقرأ بالنظر إلى الأسفل."
        }
        "Opens the Format Cells box with every setting in one place: number format, alignment, font, borders, fill and protection. Example: give a cell a date format, centre it and add a border, all at once." => {
            "يفتح مربع تنسيق الخلايا وفيه كل الإعدادات في مكان واحد: تنسيق الأرقام والمحاذاة والخط والحدود والتعبئة والحماية. مثال: أعطِ خلية تنسيق تاريخ ووسّطها وأضف لها حدًا، دفعة واحدة."
        }
        "Opens the Format Cells box on its Alignment tab: horizontal and vertical alignment, indent, wrapping, shrink to fit, merging and text angle. Example: centre a title vertically in a tall cell." => {
            "يفتح مربع تنسيق الخلايا على تبويب المحاذاة: المحاذاة الأفقية والعمودية والمسافة البادئة والالتفاف والتقليص للملاءمة والدمج وزاوية النص. مثال: وسّط عنوانًا عموديًا في خلية طويلة."
        }
        "Opens the Format Cells box on its Number tab, where you choose among all the number, currency, accounting, date, percentage and custom formats. Example: show amounts in Iraqi dinars with no decimals." => {
            "يفتح مربع تنسيق الخلايا على تبويب الأرقام، حيث تختار من كل تنسيقات الأرقام والعملة والمحاسبة والتاريخ والنسبة المئوية والتنسيقات المخصصة. مثال: اعرض المبالغ بالدينار العراقي دون خانات عشرية."
        }
        "Joins the selected cells into one big cell and centres the text in it. Example: a title over four columns is merged and centred above them." => {
            "يدمج الخلايا المحددة في خلية واحدة كبيرة ويوسّط النص فيها. مثال: عنوان فوق أربعة أعمدة يُدمج ويُوسّط فوقها."
        }
        "Joins the selected cells row by row: each row becomes one wide cell, the rows stay separate. Example: two title rows each become a wide cell of their own." => {
            "يدمج الخلايا المحددة صفًا صفًا: يصبح كل صف خلية واحدة عريضة، وتبقى الصفوف منفصلة. مثال: صفا عنوان يصبح كل منهما خلية عريضة مستقلة."
        }
        "Joins the selected cells into one cell without changing the alignment; only the top-left value is kept. Example: four cells become one, and the text stays at its edge." => {
            "يدمج الخلايا المحددة في خلية واحدة دون تغيير المحاذاة؛ ولا يبقى إلا قيمة الخلية الأولى. مثال: أربع خلايا تصبح واحدة، ويبقى النص عند حافتها."
        }
        "Splits a merged cell back into its original cells; the value stays in the first one. Example: a merged title cell becomes four cells again." => {
            "يفكّ الخلية المدمجة إلى خلاياها الأصلية؛ وتبقى القيمة في الخلية الأولى. مثال: خلية عنوان مدمجة تعود أربع خلايا."
        }
        "Shows the selected numbers as money in the chosen currency ($, £, €, ¥), with the symbol at the edge of the cell, thousands separators and two decimals. Example: 1250 becomes $ 1,250.00." => {
            "يعرض الأرقام المحددة مبالغ مالية بالعملة المختارة ($ أو £ أو € أو ¥)، مع الرمز عند حافة الخلية وفواصل الآلاف وخانتين عشريتين. مثال: ١٢٥٠ يصبح $ ١٬٢٥٠٫٠٠."
        }
        "Opens a box asking how to make room: shift the existing cells right or down, or insert whole rows or columns. Example: shift cells down to add one item in the middle of a list." => {
            "يفتح مربعًا يسألك كيف تفسح المكان: إزاحة الخلايا الموجودة إلى اليمين أو الأسفل، أو إدراج صفوف أو أعمدة كاملة. مثال: أزح الخلايا إلى الأسفل لإضافة بند في وسط قائمة."
        }
        "Adds an empty row above each selected row; the rows below move down. Example: a new row appears above row 3 for the missing item." => {
            "يضيف صفًا فارغًا فوق كل صف محدد؛ وتتحرك الصفوف التي تحته إلى الأسفل. مثال: يظهر صف جديد فوق الصف ٣ للبند الناقص."
        }
        "Adds an empty column beside each selected column; the other columns move over. Example: a new column appears before C for the quantity." => {
            "يضيف عمودًا فارغًا بجانب كل عمود محدد؛ وتتحرك الأعمدة الأخرى جانبًا. مثال: يظهر عمود جديد قبل C للكمية."
        }
        "Adds a new blank sheet to the workbook, with its own tab at the bottom. Example: a tab «ورقة٢» appears beside «ورقة١»." => {
            "يضيف ورقة جديدة فارغة إلى المصنف لها تبويب خاص في الأسفل. مثال: يظهر تبويب «ورقة٢» بجانب «ورقة١»."
        }
        "Opens a box asking how to close the gap: shift the remaining cells left or up, or delete whole rows or columns. Example: delete and shift up to drop one item from a list." => {
            "يفتح مربعًا يسألك كيف تسدّ الفراغ: إزاحة الخلايا المتبقية إلى اليسار أو الأعلى، أو حذف صفوف أو أعمدة كاملة. مثال: احذف وأزح إلى الأعلى لإسقاط بند من قائمة."
        }
        "Removes the selected rows entirely; the rows below move up to fill the gap. Example: row 3 is deleted and row 4 becomes row 3." => {
            "يزيل الصفوف المحددة بكاملها؛ وتتحرك الصفوف التي تحتها إلى الأعلى لسدّ الفراغ. مثال: يُحذف الصف ٣ فيصبح الصف ٤ هو الصف ٣."
        }
        "Removes the selected columns entirely; the other columns move over to fill the gap. Example: column B is deleted and column C becomes B." => {
            "يزيل الأعمدة المحددة بكاملها؛ وتتحرك الأعمدة الأخرى لسدّ الفراغ. مثال: يُحذف العمود B فيصبح العمود C هو B."
        }
        "Removes the current sheet and everything on it from the workbook; this cannot be undone. Example: an empty «ورقة٣» is deleted." => {
            "يزيل الورقة الحالية وكل ما فيها من المصنف؛ ولا يمكن التراجع عن ذلك. مثال: تُحذف «ورقة٣» الفارغة."
        }
        "Opens a box where you type an exact height for the selected rows. Example: set the title row to 30 points." => {
            "يفتح مربعًا تكتب فيه ارتفاعًا محددًا للصفوف المحددة. مثال: اجعل صف العنوان ٣٠ نقطة."
        }
        "Makes the selected rows exactly as tall as their tallest text needs. Example: a row with wrapped text grows to show all its lines." => {
            "يجعل ارتفاع الصفوف المحددة بقدر ما يحتاجه أطول نص فيها تمامًا. مثال: صف فيه نص ملتف يكبر ليعرض كل أسطره."
        }
        "Opens a box where you type an exact width for the selected columns. Example: set column A to 20 characters wide." => {
            "يفتح مربعًا تكتب فيه عرضًا محددًا للأعمدة المحددة. مثال: اجعل عرض العمود A عشرين حرفًا."
        }
        "Makes the selected columns exactly as wide as their longest entry. Example: a column of names widens so no name is cut off." => {
            "يجعل عرض الأعمدة المحددة بقدر أطول مدخل فيها تمامًا. مثال: عمود الأسماء يتسع فلا يُقطع أي اسم."
        }
        "Sets the standard width for every column of the sheet that you have not resized by hand. Example: all the columns become 12 characters wide." => {
            "يحدد العرض القياسي لكل أعمدة الورقة التي لم تغيّر عرضها بيدك. مثال: تصبح كل الأعمدة بعرض ١٢ حرفًا."
        }
        "Hides the selected rows; their numbers are skipped and their data stays. Example: rows 2 and 3 disappear, and row 4 follows row 1." => {
            "يخفي الصفوف المحددة؛ فتُتخطى أرقامها وتبقى بياناتها. مثال: يختفي الصفان ٢ و٣، ويلي الصفُ ٤ الصفَ ١."
        }
        "Hides the selected columns; their letters are skipped and their data stays. Example: column B disappears, and C follows A." => {
            "يخفي الأعمدة المحددة؛ فتُتخطى حروفها وتبقى بياناتها. مثال: يختفي العمود B، ويلي C العمودَ A."
        }
        "Shows again the rows hidden inside the selection. Example: select rows 1 to 4 and the hidden rows 2 and 3 come back." => {
            "يعيد إظهار الصفوف المخفية داخل التحديد. مثال: حدد الصفوف من ١ إلى ٤ فيعود الصفان المخفيان ٢ و٣."
        }
        "Shows again the columns hidden inside the selection. Example: select columns A to C and the hidden column B comes back." => {
            "يعيد إظهار الأعمدة المخفية داخل التحديد. مثال: حدد الأعمدة من A إلى C فيعود العمود المخفي B."
        }
        "Hides the current sheet: its tab disappears, but its data and formulas keep working. Example: a sheet of exchange rates is hidden from view." => {
            "يخفي الورقة الحالية: يختفي تبويبها، لكن بياناتها ومعادلاتها تظل تعمل. مثال: ورقة أسعار الصرف تُخفى عن النظر."
        }
        "Opens a list of the hidden sheets so you can bring one back. Example: pick the hidden rates sheet and its tab returns." => {
            "يفتح قائمة بالأوراق المخفية لتعيد إحداها. مثال: اختر ورقة الأسعار المخفية فيعود تبويبها."
        }
        "Lets you type a new name for the current sheet's tab. Example: «ورقة١» becomes «المبيعات»." => {
            "يتيح لك كتابة اسم جديد لتبويب الورقة الحالية. مثال: «ورقة١» تصبح «المبيعات»."
        }
        "Moves the current sheet to another place in the tab row or into another workbook, or makes a copy of it. Example: copy January's sheet to start February." => {
            "ينقل الورقة الحالية إلى موضع آخر في صف التبويبات أو إلى مصنف آخر، أو ينشئ نسخة منها. مثال: انسخ ورقة كانون الثاني لتبدأ بها شباط."
        }
        "Locks the sheet so nobody can change its cells, optionally with a password; cells you unlocked first stay editable. Example: protect an invoice so only the quantity cells can be changed." => {
            "يقفل الورقة فلا يستطيع أحد تغيير خلاياها، مع كلمة مرور إن شئت؛ وتبقى الخلايا التي فككت قفلها قبل ذلك قابلة للتحرير. مثال: احمِ فاتورة بحيث لا تتغير إلا خلايا الكمية."
        }
        "Marks the selected cells as locked or unlocked; the lock takes effect once the sheet is protected. Example: unlock the input cells, then protect the sheet." => {
            "يعلّم الخلايا المحددة مقفلة أو غير مقفلة؛ ويسري القفل عند حماية الورقة. مثال: فكّ قفل خلايا الإدخال ثم احمِ الورقة."
        }
        "Copies the top cell of the selection into every cell below it. Example: a formula in B2 is copied down to B6." => {
            "ينسخ الخلية العليا من التحديد إلى كل خلية تحتها. مثال: معادلة في B2 تُنسخ إلى الأسفل حتى B6."
        }
        "Copies the leftmost cell of the selection into every cell to its right. Example: the formula in B2 is copied across to E2." => {
            "ينسخ الخلية الواقعة في أقصى يسار التحديد إلى كل خلية على يمينها. مثال: المعادلة في B2 تُنسخ عبر الصف حتى E2."
        }
        "Copies the bottom cell of the selection into every cell above it. Example: the value in B6 is copied up to B2." => {
            "ينسخ الخلية السفلى من التحديد إلى كل خلية فوقها. مثال: القيمة في B6 تُنسخ إلى الأعلى حتى B2."
        }
        "Copies the rightmost cell of the selection into every cell to its left. Example: the value in E2 is copied back to B2." => {
            "ينسخ الخلية الواقعة في أقصى يمين التحديد إلى كل خلية على يسارها. مثال: القيمة في E2 تُنسخ رجوعًا حتى B2."
        }
        "Fills the selection with a series: numbers growing by a step, dates by day, month or year, or a growth factor. Example: 1, 2, 3, 4 down a column, or the first day of every month." => {
            "يملأ التحديد بسلسلة: أرقام تزيد بخطوة ثابتة، أو تواريخ باليوم أو الشهر أو السنة، أو معامل نمو. مثال: ١، ٢، ٣، ٤ نزولًا في عمود، أو اليوم الأول من كل شهر."
        }
        "Looks at the examples you typed and fills the rest of the column the same way. Example: type the first name from «أحمد علي» into B2, and Flash Fill writes the first name for every row." => {
            "ينظر في الأمثلة التي كتبتها ويملأ بقية العمود على المنوال نفسه. مثال: اكتب الاسم الأول من «أحمد علي» في B2، فيكتب التعبئة السريعة الاسم الأول لكل صف."
        }
        "Empties the selected cells completely: contents, formatting, comments and links. Example: a yellow bold 150 becomes a plain empty cell." => {
            "يفرغ الخلايا المحددة تمامًا: المحتوى والتنسيق والتعليقات والروابط. مثال: ١٥٠ أصفر غامق يصبح خلية فارغة عادية."
        }
        "Removes the formatting of the selected cells but keeps what is in them. Example: a yellow bold 150 becomes a plain 150." => {
            "يزيل تنسيق الخلايا المحددة ويبقي ما فيها. مثال: ١٥٠ أصفر غامق يصبح ١٥٠ عاديًا."
        }
        "Deletes what is in the selected cells but keeps their formatting. Example: the yellow fill stays and the number goes." => {
            "يحذف ما في الخلايا المحددة ويبقي تنسيقها. مثال: تبقى التعبئة الصفراء ويذهب الرقم."
        }
        "Removes the comments and notes attached to the selected cells; the values stay. Example: the little comment marks in the corners disappear." => {
            "يزيل التعليقات والملاحظات المرفقة بالخلايا المحددة؛ وتبقى القيم. مثال: تختفي علامات التعليق الصغيرة في الزوايا."
        }
        "Removes the links from the selected cells; the text stays but is no longer blue or clickable. Example: a web address becomes ordinary text." => {
            "يزيل الروابط من الخلايا المحددة؛ ويبقى النص لكنه لا يعود أزرق ولا قابلًا للنقر. مثال: عنوان موقع على الويب يصبح نصًا عاديًا."
        }
        "Orders the rows by the selected column from smallest to largest: numbers first, then A to Z, oldest dates first. Example: 9, 3, 5 becomes 3, 5, 9." => {
            "يرتب الصفوف بحسب العمود المحدد من الأصغر إلى الأكبر: الأرقام أولًا، ثم من أ إلى ي، والتواريخ الأقدم أولًا. مثال: ٩، ٣، ٥ تصبح ٣، ٥، ٩."
        }
        "Orders the rows by the selected column from largest to smallest: Z to A, newest dates first. Example: 3, 5, 9 becomes 9, 5, 3." => {
            "يرتب الصفوف بحسب العمود المحدد من الأكبر إلى الأصغر: من ي إلى أ، والتواريخ الأحدث أولًا. مثال: ٣، ٥، ٩ تصبح ٩، ٥، ٣."
        }
        "Opens a box to sort by several columns at once, each ascending or descending, or by colour. Example: sort by city, then by name within each city." => {
            "يفتح مربعًا للترتيب بحسب عدة أعمدة معًا، كل منها تصاعديًا أو تنازليًا، أو بحسب اللون. مثال: رتّب بحسب المدينة، ثم بحسب الاسم داخل كل مدينة."
        }
        "Adds filter buttons to the header row so you can show only the rows that match. Example: show only the rows where the city is «بغداد»." => {
            "يضيف أزرار تصفية إلى صف العناوين لتعرض الصفوف المطابقة فقط. مثال: اعرض الصفوف التي مدينتها «بغداد» فقط."
        }
        "Shows all the rows again by removing every filter condition; the filter buttons stay. Example: the hidden rows come back." => {
            "يعيد إظهار كل الصفوف بإزالة كل شروط التصفية؛ وتبقى أزرار التصفية. مثال: تعود الصفوف المخفية."
        }
        "Runs the current filter and sort again after the data changed. Example: a row you edited to match the filter now appears." => {
            "يعيد تطبيق التصفية والترتيب الحاليين بعد تغيّر البيانات. مثال: صف عدّلته ليطابق التصفية يظهر الآن."
        }
        "Searches the sheet or the whole workbook for a word or number and jumps to each match. Example: find every cell containing 2024." => {
            "يبحث في الورقة أو في المصنف كله عن كلمة أو رقم وينتقل إلى كل نتيجة. مثال: ابحث عن كل خلية تحتوي ٢٠٢٤."
        }
        "Finds a word or number and replaces it with another, one match at a time or all at once. Example: replace every «بغداد» with «البصرة»." => {
            "يبحث عن كلمة أو رقم ويستبدله بآخر، نتيجة واحدة في كل مرة أو كلها دفعة واحدة. مثال: استبدل كل «بغداد» بـ«البصرة»."
        }
        "Jumps to the cell or range you type, or to a named range. Example: type D20 and the selection moves there." => {
            "ينتقل إلى الخلية أو النطاق الذي تكتبه، أو إلى نطاق مسمّى. مثال: اكتب D20 فينتقل التحديد إليها."
        }
        "Selects every cell of one kind in the sheet at once: formulas, typed values, blanks, notes, conditional formats or validation rules. Example: select all the formula cells to check them." => {
            "يحدد كل الخلايا من نوع واحد في الورقة دفعة واحدة: المعادلات أو القيم المكتوبة أو الفارغة أو الملاحظات أو التنسيقات الشرطية أو قواعد التحقق. مثال: حدد كل خلايا المعادلات لمراجعتها."
        }
        "Selects every cell that contains a formula. Example: all the =SUM cells are selected at once." => {
            "يحدد كل خلية تحتوي معادلة. مثال: تُحدد كل خلايا =SUM دفعة واحدة."
        }
        "Selects every cell that has a note or comment attached. Example: the three cells with remarks are selected." => {
            "يحدد كل خلية مرفق بها ملاحظة أو تعليق. مثال: تُحدد الخلايا الثلاث التي فيها ملاحظات."
        }
        "Selects every cell that has conditional formatting. Example: the cells that turn green above 1000 are selected." => {
            "يحدد كل خلية فيها تنسيق شرطي. مثال: تُحدد الخلايا التي تتحول إلى الأخضر فوق ١٠٠٠."
        }
        "Selects every cell that holds a typed value (number or text), not a formula. Example: the entered amounts are selected, the totals are not." => {
            "يحدد كل خلية تحمل قيمة مكتوبة (رقمًا أو نصًا) لا معادلة. مثال: تُحدد المبالغ المدخلة، ولا تُحدد المجاميع."
        }
        "Selects every cell that has a data-validation rule (a list or a limit on what can be typed). Example: the cells with a drop-down list are selected." => {
            "يحدد كل خلية فيها قاعدة تحقق من البيانات (قائمة أو حد لما يمكن كتابته). مثال: تُحدد الخلايا التي فيها قائمة منسدلة."
        }
        "Takes you back to the start screen, where you open a recent workbook or begin a new one; the file you have open stays as it is. Example: press it to find yesterday's file in the recent list." => {
            "يعيدك إلى شاشة البداية حيث تفتح مصنفًا حديثًا أو تبدأ مصنفًا جديدًا؛ والملف المفتوح يبقى كما هو. مثال: اضغطه لتجد ملف الأمس في قائمة الملفات الأخيرة."
        }
        "Writes the workbook to its file on the computer so nothing is lost if the program closes; the first time, it asks where to put the file. Example: after typing the day's figures, press it and the «unsaved» mark on the title goes away." => {
            "يكتب المصنف في ملفه على الحاسوب كي لا يضيع شيء إذا أُغلق البرنامج؛ وفي المرة الأولى يسألك أين تضع الملف. مثال: بعد كتابة أرقام اليوم اضغطه فتختفي علامة «غير محفوظ» من العنوان."
        }
        "Opens the list of your recent changes so you can take back several at once: pick how far back to go. Example: choose the third row to undo the last three edits together." => {
            "يفتح قائمة تغييراتك الأخيرة لتتراجع عن عدة تغييرات دفعة واحدة: اختر إلى أي حدّ ترجع. مثال: اختر الصف الثالث لتتراجع عن آخر ثلاثة تعديلات معًا."
        }
        "Shows the rest of the quick buttons that did not fit in the title bar, in a small menu. Example: in a narrow window, Print and Export are found here." => {
            "يعرض بقية الأزرار السريعة التي لم تتسع لها شريط العنوان في قائمة صغيرة. مثال: في نافذة ضيقة تجد هنا زرّي الطباعة والتصدير."
        }
        "Finds any button by typing part of its name, then runs it from the list; quicker than hunting through the tabs. Example: type «تجميد» and pick Freeze Panes." => {
            "يجد أي زر بكتابة جزء من اسمه ثم يشغّله من القائمة؛ أسرع من البحث بين التبويبات. مثال: اكتب «تجميد» واختر تجميد الأجزاء."
        }
        "Exports the workbook as a PDF or saves a copy in another format or place, ready to send to someone. Example: make a PDF of the monthly report to send to the commander." => {
            "يصدّر المصنف بصيغة PDF أو يحفظ نسخة بصيغة أو مكان آخر جاهزة للإرسال إلى شخص ما. مثال: اصنع PDF من التقرير الشهري لإرساله إلى الآمر."
        }
        "Opens the comments pane listing every note attached to cells in the workbook, so you can read and answer them in one place. Example: see the three notes a colleague left on the budget sheet." => {
            "يفتح لوحة التعليقات التي تعرض كل الملاحظات الملحقة بالخلايا في المصنف لتقرأها وتردّ عليها في مكان واحد. مثال: اطّلع على الملاحظات الثلاث التي تركها زميلك على ورقة الميزانية."
        }
        "Switches the interface between light and dark colours; your numbers and formatting do not change. Example: at night the white sheet turns dark grey with light text." => {
            "يبدّل الواجهة بين الألوان الفاتحة والداكنة؛ وأرقامك وتنسيقك لا يتغيران. مثال: في الليل تصير الورقة البيضاء رمادية داكنة بنص فاتح."
        }
        "Summarises a long list into a small table of totals: choose a column to group by and a column to add up, and every group gets one row. Example: ٢٠٠ rows of sales become one row per city with that city's total." => {
            "يلخّص قائمة طويلة في جدول صغير من المجاميع: اختر عمودًا للتجميع وعمودًا للجمع، فيحصل كل مجموعة على صف واحد. مثال: ٢٠٠ صف من المبيعات تصير صفًا واحدًا لكل مدينة مع مجموع تلك المدينة."
        }
        "Turns the selected cells into a table: a coloured header row with filter arrows, striped rows, and anything you type under the last row joins it automatically. Example: a list of names and dates gets a blue header and alternating bands." => {
            "يحوّل الخلايا المحددة إلى جدول: صف عناوين ملوّن بأسهم تصفية، وصفوف مخططة، وكل ما تكتبه تحت آخر صف ينضم إليه تلقائيًا. مثال: قائمة أسماء وتواريخ تحصل على عناوين زرقاء وأشرطة متناوبة."
        }
        "Places a photo or picture file from your computer on top of the sheet; drag it to move it and pull its corners to resize it. Example: the unit's logo sits above the table on the report sheet." => {
            "يضع صورة أو ملف صورة من حاسوبك فوق الورقة؛ اسحبها لتحريكها واسحب زواياها لتغيير حجمها. مثال: شعار الوحدة يجلس فوق الجدول في ورقة التقرير."
        }
        "Draws a shape over the sheet (a rectangle, a circle, an arrow, a star) that you can drag, resize and colour. Example: a red arrow pointing at the highest number in a chart." => {
            "يرسم شكلًا فوق الورقة (مستطيلًا أو دائرة أو سهمًا أو نجمة) يمكنك سحبه وتغيير حجمه وتلوينه. مثال: سهم أحمر يشير إلى أعلى رقم في مخطط."
        }
        "Puts a small simple picture (a star, a flag, a tool, a tick) from the built-in icon library on the sheet. Example: a star icon beside the title of the best month." => {
            "يضع صورة صغيرة بسيطة (نجمة أو علمًا أو أداة أو علامة صح) من مكتبة الأيقونات المدمجة على الورقة. مثال: أيقونة نجمة بجانب عنوان أفضل شهر."
        }
        "Looks at the selected numbers and offers charts that suit them (bars, lines, a pie); pick one and it appears beside the data. Example: twelve monthly totals become a bar chart with one bar per month." => {
            "ينظر في الأرقام المحددة ويقترح مخططات تناسبها (أعمدة أو خطوط أو دائرة)؛ اختر واحدًا فيظهر بجانب البيانات. مثال: اثنا عشر مجموعًا شهريًا تصير مخطط أعمدة بعمود لكل شهر."
        }
        "Adds a panel of buttons, one per value in a column; click a button and the table shows only the rows with that value. Example: buttons «بغداد», «البصرة», «الموصل» — press one and only that city's rows stay." => {
            "يضيف لوحة أزرار، زرًا لكل قيمة في عمود؛ اضغط زرًا فيعرض الجدول الصفوف التي تحمل تلك القيمة فقط. مثال: أزرار «بغداد» و«البصرة» و«الموصل»، اضغط واحدًا فتبقى صفوف تلك المدينة وحدها."
        }
        "Draws a tiny chart inside one cell from a row of numbers, so each row shows its own trend at a glance. Example: beside each product's twelve monthly figures, a small line rising or falling." => {
            "يرسم مخططًا صغيرًا داخل خلية واحدة من صف أرقام، فيُظهر كل صف اتجاهه بنظرة. مثال: بجانب أرقام كل منتج الشهرية الاثني عشر خط صغير يصعد أو يهبط."
        }
        "Puts a tick box in each selected cell; clicking a box switches it between TRUE and FALSE. Example: a «تم» column where you tick each task as it is finished." => {
            "يضع مربع اختيار في كل خلية محددة؛ والنقر على المربع يبدّله بين TRUE وFALSE. مثال: عمود «تم» تؤشّر فيه كل مهمة عند إنجازها."
        }
        "Makes the selected cell a link that opens a web page, a file or another place in the workbook when clicked. Example: the cell «موقع الوزارة» opens the ministry's website." => {
            "يجعل الخلية المحددة رابطًا يفتح صفحة ويب أو ملفًا أو مكانًا آخر في المصنف عند النقر عليه. مثال: الخلية «موقع الوزارة» تفتح موقع الوزارة."
        }
        "Attaches a note to the selected cell; a small red corner marks it and the note shows when you point at the cell. Example: on a strange figure, write «راجع هذا الرقم» for a colleague." => {
            "يلحق ملاحظة بالخلية المحددة؛ زاوية حمراء صغيرة تشير إليها وتظهر الملاحظة حين تشير إلى الخلية. مثال: على رقم غريب اكتب «راجع هذا الرقم» لزميلك."
        }
        "Puts a box of free text that floats above the cells; move and resize it anywhere, not tied to the grid. Example: a note «أرقام مؤقتة» placed in the corner of a chart." => {
            "يضع صندوق نص حرّ يطفو فوق الخلايا؛ حرّكه وغيّر حجمه في أي مكان من غير ارتباط بالشبكة. مثال: ملاحظة «أرقام مؤقتة» موضوعة في زاوية مخطط."
        }
        "Adds text that prints at the top and bottom of every page: a title, the date, the page number. Example: «وزارة الدفاع» at the top of each printed page and «صفحة ١» at the bottom." => {
            "يضيف نصًا يُطبع في أعلى كل صفحة وأسفلها: عنوانًا أو تاريخًا أو رقم صفحة. مثال: «وزارة الدفاع» في أعلى كل صفحة مطبوعة و«صفحة ١» في أسفلها."
        }
        "Opens a table of special characters that are not on the keyboard and puts the one you pick into the cell. Example: insert «≠», «±» or «©» after a number." => {
            "يفتح جدولًا بالرموز الخاصة التي ليست على لوحة المفاتيح ويضع ما تختاره في الخلية. مثال: أدرج «≠» أو «±» أو «©» بعد رقم."
        }
        "Switches from drawing back to the normal pointer so you can click ink strokes and shapes to move, resize or delete them. Example: drag the circle you drew to sit over a different cell." => {
            "يعيدك من الرسم إلى المؤشر العادي لتنقر على خطوط الحبر والأشكال فتحرّكها أو تغيّر حجمها أو تحذفها. مثال: اسحب الدائرة التي رسمتها لتجلس فوق خلية أخرى."
        }
        "Lets you draw freehand over the sheet with the mouse or a pen, like a marker on paper; the cells underneath are not changed. Example: circle a wrong number and draw an arrow to the right one." => {
            "يتيح لك الرسم بحرية فوق الورقة بالفأرة أو القلم كقلم تخطيط على ورق؛ والخلايا تحته لا تتغير. مثال: حوّط رقمًا خاطئًا وارسم سهمًا إلى الرقم الصحيح."
        }
        "Rubs out ink strokes you drew; click or drag over a stroke and it disappears, leaving the cells as they were. Example: wipe away the circle after the number is fixed." => {
            "يمسح خطوط الحبر التي رسمتها؛ انقر أو اسحب فوق خط فيختفي وتبقى الخلايا كما كانت. مثال: امسح الدائرة بعد تصحيح الرقم."
        }
        "Replaces the stroke you just drew with the clean shape it looks like: a wobbly ring becomes a perfect circle, a rough box a rectangle. Example: a hand-drawn loop around a cell becomes a neat oval." => {
            "يستبدل الخط الذي رسمته للتو بالشكل النظيف الذي يشبهه: حلقة مهزوزة تصير دائرة كاملة، وصندوق خشن مستطيلًا. مثال: حلقة مرسومة باليد حول خلية تصير شكلًا بيضويًا أنيقًا."
        }
        "Changes the whole workbook's look in one go: the colours, the fonts and the chart styles all switch together. Example: table headers turn from blue to green and the headings change font." => {
            "يغيّر مظهر المصنف كله دفعة واحدة: الألوان والخطوط وأنماط المخططات تتبدل معًا. مثال: عناوين الجداول تتحول من الأزرق إلى الأخضر وتتغير خطوط العناوين."
        }
        "Sets the empty space between the printed cells and the edge of the paper: Normal, Wide, Narrow or your own numbers. Example: Narrow fits two more columns on the page." => {
            "يضبط الفراغ بين الخلايا المطبوعة وحافة الورق: عادي أو واسع أو ضيق أو أرقامك الخاصة. مثال: الضيق يُدخل عمودين إضافيين في الصفحة."
        }
        "Chooses whether the sheet prints on upright (portrait) or sideways (landscape) paper. Example: a wide table with twelve month columns prints sideways so it fits." => {
            "يختار هل تُطبع الورقة على ورق عمودي أم أفقي (جانبي). مثال: جدول عريض بأعمدة اثني عشر شهرًا يُطبع جانبيًا ليتسع."
        }
        "Picks the paper the sheet prints on (A4, A3, Letter…) so the page breaks match the printer. Example: switch from A4 to A3 to fit the whole table on one sheet." => {
            "يختار الورق الذي تُطبع عليه الورقة (A4 أو A3 أو Letter…) كي تطابق فواصل الصفحات الطابعة. مثال: بدّل من A4 إلى A3 ليتسع الجدول كله في ورقة واحدة."
        }
        "Marks the selected cells as the only part of the sheet that prints; everything outside is left out. Example: print just the summary block, not the working columns beside it." => {
            "يجعل الخلايا المحددة الجزء الوحيد من الورقة الذي يُطبع؛ وكل ما خارجها يُترك. مثال: اطبع كتلة الملخص فقط لا أعمدة العمل المجاورة لها."
        }
        "Decides where one printed page ends and the next begins; add a break above a row or before a column, or remove the ones you made. Example: start every department on a new page." => {
            "يقرر أين تنتهي صفحة مطبوعة وتبدأ التالية؛ أضف فاصلًا فوق صف أو قبل عمود، أو أزل ما أضفته. مثال: ابدأ كل قسم في صفحة جديدة."
        }
        "Repeats chosen rows or columns at the top or side of every printed page, so long tables keep their headings. Example: the header row with «الاسم» and «الرتبة» prints on every page." => {
            "يكرّر صفوفًا أو أعمدة تختارها في أعلى كل صفحة مطبوعة أو جانبها، فتحتفظ الجداول الطويلة بعناوينها. مثال: صف العناوين الذي فيه «الاسم» و«الرتبة» يُطبع في كل صفحة."
        }
        "Flips the sheet so column A is on the right and the row numbers sit on the right edge, the natural way for Arabic. Example: the first column of names starts at the right side of the window." => {
            "يقلب الورقة فيصير العمود A على اليمين وأرقام الصفوف على الحافة اليمنى، كما يناسب العربية. مثال: عمود الأسماء الأول يبدأ من الجانب الأيمن للنافذة."
        }
        "The usual margins: about ٢ cm at the top and bottom and ١٫٨ cm at the sides. Example: the standard spacing for an ordinary report page." => {
            "الهوامش المعتادة: نحو ٢ سم في الأعلى والأسفل و١٫٨ سم في الجانبين. مثال: التباعد القياسي لصفحة تقرير عادية."
        }
        "Large margins: more empty paper around the cells, room for notes or a binder punch. Example: a document that will be filed in a ring binder." => {
            "هوامش كبيرة: ورق فارغ أكثر حول الخلايا، ومتسع للملاحظات أو لثقب المصنّف. مثال: مستند سيُحفظ في مصنّف حلقي."
        }
        "Small margins: the cells reach almost to the edge of the paper, so more columns fit on each page. Example: a wide table fits on one page instead of two." => {
            "هوامش صغيرة: الخلايا تصل إلى حافة الورق تقريبًا، فتتسع أعمدة أكثر في كل صفحة. مثال: جدول عريض يتسع في صفحة واحدة بدل صفحتين."
        }
        "Opens the page setup dialog, where you type your own margins and set the orientation, paper size, scaling and header and footer together. Example: set a ٣ cm left margin and shrink the sheet to ٨٠٪." => {
            "يفتح مربع حوار إعداد الصفحة حيث تكتب هوامشك بنفسك وتضبط الاتجاه وحجم الورق والتحجيم والرأس والتذييل معًا. مثال: اضبط هامشًا أيسر ٣ سم وصغّر الورقة إلى ٨٠٪."
        }
        "Prints on upright paper, taller than it is wide: more rows per page, fewer columns. Example: a long list of names prints as a tall page." => {
            "يطبع على ورق عمودي، طوله أكبر من عرضه: صفوف أكثر في الصفحة وأعمدة أقل. مثال: قائمة أسماء طويلة تُطبع صفحة طويلة."
        }
        "Prints on sideways paper, wider than it is tall: more columns per page, fewer rows. Example: a table with twelve month columns fits across the page." => {
            "يطبع على ورق جانبي، عرضه أكبر من طوله: أعمدة أكثر في الصفحة وصفوف أقل. مثال: جدول بأعمدة اثني عشر شهرًا يتسع عرض الصفحة."
        }
        "Set makes the selected cells the only part that prints; Clear removes that mark so the whole sheet prints again. Example: set it on the summary block, clear it when you need the full sheet." => {
            "«تعيين» يجعل الخلايا المحددة الجزء الوحيد الذي يُطبع؛ و«مسح» يزيل تلك العلامة فتُطبع الورقة كلها من جديد. مثال: عيّنه على كتلة الملخص، وامسحه حين تحتاج الورقة كاملة."
        }
        "Insert starts a new printed page above the selected row (or before the selected column); Reset All removes every break you added so the sheet paginates by itself again. Example: a break above row ٢٠ so the second department starts on a new page." => {
            "«إدراج» يبدأ صفحة مطبوعة جديدة فوق الصف المحدد (أو قبل العمود المحدد)؛ و«إعادة تعيين الكل» يزيل كل الفواصل التي أضفتها فتتقسم الورقة بنفسها من جديد. مثال: فاصل فوق الصف ٢٠ ليبدأ القسم الثاني في صفحة جديدة."
        }
        "Removes the page break next to the selected cell, so the rows flow onto the same page again. Example: the break above row ٢٠ goes and the department continues on page ١." => {
            "يزيل فاصل الصفحة المجاور للخلية المحددة، فتنساب الصفوف في الصفحة نفسها من جديد. مثال: يزول الفاصل فوق الصف ٢٠ ويستمر القسم في الصفحة ١."
        }
        "Opens a list of every function with a short description of each; pick one and it is written into the cell with its brackets ready. Example: choose SUM and the cell shows =SUM( waiting for the cells to add." => {
            "يفتح قائمة بكل الدوال مع وصف قصير لكل واحدة؛ اختر واحدة فتُكتب في الخلية وقوساها جاهزان. مثال: اختر SUM فتعرض الخلية =SUM( بانتظار الخلايا التي ستجمعها."
        }
        "Writes a SUM formula in the empty cell under (or beside) your numbers and adds them up at once; the arrow next to it offers Average, Count, Max and Min too. Example: under ١٢، ٣٤ and ٢١ it writes =SUM(B1:B3) and shows ٦٧." => {
            "يكتب صيغة SUM في الخلية الفارغة تحت أرقامك (أو بجانبها) ويجمعها فورًا؛ والسهم المجاور يقدّم المعدل والعدّ والأكبر والأصغر أيضًا. مثال: تحت ١٢ و٣٤ و٢١ يكتب =SUM(B1:B3) ويعرض ٦٧."
        }
        "Lists every name you gave to cells and ranges, with what each one points to, so you can rename, fix or delete them. Example: see that «المبيعات» means B2:B9 and change it to B2:B12." => {
            "يعرض كل اسم أعطيته لخلايا ونطاقات مع ما يشير إليه كل واحد، لتعيد تسميته أو تصلحه أو تحذفه. مثال: ترى أن «المبيعات» تعني B2:B9 فتغيّرها إلى B2:B12."
        }
        "Gives the selected cells a word to call them by, so formulas can say =SUM(المبيعات) instead of =SUM(B2:B9). Example: select the sales column and name it «المبيعات»." => {
            "يعطي الخلايا المحددة كلمة تُنادى بها، فتقول الصيغ =SUM(المبيعات) بدل =SUM(B2:B9). مثال: حدّد عمود المبيعات وسمّه «المبيعات»."
        }
        "While you type a formula, drops down the list of your names so you can put one in without remembering the cells. Example: in =SUM( pick «المبيعات» from the list." => {
            "أثناء كتابة الصيغة يُنزل قائمة أسمائك لتضع واحدًا من غير أن تتذكر الخلايا. مثال: في =SUM( اختر «المبيعات» من القائمة."
        }
        "Names several ranges at once from their headings: select the table with its header row and each column is named after its heading. Example: columns «الاسم», «الرتبة», «الراتب» each get that name." => {
            "يسمّي عدة نطاقات دفعة واحدة من عناوينها: حدّد الجدول مع صف عناوينه فيُسمّى كل عمود باسم عنوانه. مثال: الأعمدة «الاسم» و«الرتبة» و«الراتب» يحصل كل واحد على ذلك الاسم."
        }
        "Draws arrows from the cells a formula reads into the formula cell, so you see where its number comes from. Example: the total cell gets arrows from the three cells it adds." => {
            "يرسم أسهمًا من الخلايا التي تقرؤها الصيغة إلى خلية الصيغة، فترى من أين يأتي رقمها. مثال: خلية المجموع تحصل على أسهم من الخلايا الثلاث التي تجمعها."
        }
        "Draws arrows from the selected cell to every formula that uses it, so you see what changes if you change it. Example: the price cell points to the three totals built on it." => {
            "يرسم أسهمًا من الخلية المحددة إلى كل صيغة تستخدمها، فترى ما الذي يتغير إذا غيّرتها. مثال: خلية السعر تشير إلى المجاميع الثلاثة المبنية عليها."
        }
        "Clears all the tracing arrows from the sheet; the cells and formulas are untouched. Example: after checking the totals, the blue arrows disappear." => {
            "يمسح كل أسهم التتبع من الورقة؛ والخلايا والصيغ لا تُمسّ. مثال: بعد فحص المجاميع تختفي الأسهم الزرقاء."
        }
        "Shows the formula text in every cell instead of its result, so you can read and compare them all at once; press again to go back to the numbers. Example: ٦٧ turns into =SUM(B1:B3) in its cell." => {
            "يعرض نص الصيغة في كل خلية بدل نتيجتها لتقرأها وتقارنها كلها دفعة واحدة؛ اضغط مرة أخرى لتعود الأرقام. مثال: ٦٧ تتحول إلى =SUM(B1:B3) في خليتها."
        }
        "Looks through the sheet for formulas with errors (#DIV/0!, #REF!, #NAME?) and takes you to each one with a hint about the cause. Example: it stops on a cell dividing by an empty cell." => {
            "يبحث في الورقة عن الصيغ التي فيها أخطاء (#DIV/0! أو #REF! أو #NAME?) ويأخذك إلى كل واحدة مع تلميح عن سببها. مثال: يتوقف عند خلية تقسم على خلية فارغة."
        }
        "Works out the selected formula one step at a time, showing each part turn into its value, so you can find where it goes wrong. Example: =A1*2+B1 becomes =٣*2+B1, then =٦+B1, then ١٠." => {
            "يحسب الصيغة المحددة خطوة خطوة ويُريك كل جزء يتحول إلى قيمته، لتجد أين تخطئ. مثال: =A1*2+B1 تصير =٣*2+B1 ثم =٦+B1 ثم ١٠."
        }
        "Opens a small panel that keeps showing the values of cells you choose, even when they are far away or on another sheet. Example: watch the grand total while you edit rows at the bottom." => {
            "يفتح لوحة صغيرة تظل تعرض قيم خلايا تختارها حتى لو كانت بعيدة أو في ورقة أخرى. مثال: راقب المجموع الكلي وأنت تعدّل الصفوف في الأسفل."
        }
        "Chooses whether formulas recalculate by themselves after every change (Automatic) or only when you ask (Manual). Example: on a huge slow workbook, switch to Manual and press F9 when ready." => {
            "يختار هل تُعاد حسابات الصيغ من تلقاء نفسها بعد كل تغيير (تلقائي) أم عند طلبك فقط (يدوي). مثال: في مصنف ضخم بطيء بدّل إلى يدوي واضغط F9 حين تكون جاهزًا."
        }
        "Recalculates every formula in every open workbook right now; needed when calculation is set to Manual. Example: after many edits, press it and all the totals update together." => {
            "يعيد حساب كل صيغة في كل المصنفات المفتوحة الآن؛ وتحتاجه حين يكون الحساب يدويًا. مثال: بعد تعديلات كثيرة اضغطه فتتحدث كل المجاميع معًا."
        }
        "Recalculates the formulas on the current sheet only, leaving other sheets as they are; quicker than the whole workbook. Example: refresh the totals on the «آذار» sheet without touching the rest." => {
            "يعيد حساب صيغ الورقة الحالية فقط ويترك الأوراق الأخرى كما هي؛ أسرع من المصنف كله. مثال: حدّث مجاميع ورقة «آذار» من غير أن تمسّ البقية."
        }
        "Every formula recalculates the moment any cell changes; what you see is always up to date. Example: type a new price and the total changes at once." => {
            "كل صيغة تُعاد حسابها لحظة تغيّر أي خلية؛ وما تراه محدّث دائمًا. مثال: اكتب سعرًا جديدًا فيتغير المجموع فورًا."
        }
        "Formulas recalculate at once, except what-if data tables, which wait until you press F9; data tables are slow on big sheets. Example: totals update instantly while the big scenario table refreshes only on request." => {
            "الصيغ تُعاد حسابها فورًا، ما عدا جداول البيانات (ماذا لو) التي تنتظر حتى تضغط F9؛ فجداول البيانات بطيئة في الأوراق الكبيرة. مثال: المجاميع تتحدث فورًا بينما جدول السيناريوهات الكبير لا يتحدث إلا عند الطلب."
        }
        "Nothing recalculates until you press F9 (Calculate Now); edits are fast but totals may be stale until then. Example: change twenty prices, then press F9 once to update everything." => {
            "لا يُعاد حساب شيء حتى تضغط F9 (احسب الآن)؛ التعديلات سريعة لكن المجاميع قد تبقى قديمة حتى ذلك الحين. مثال: غيّر عشرين سعرًا ثم اضغط F9 مرة واحدة ليتحدث كل شيء."
        }
        "Reads a plain text file whose values are separated by commas or tabs and puts them into cells, one line per row. Example: the export from the attendance machine becomes a sheet with a column for each field." => {
            "يقرأ ملفًا نصيًا عاديًا تفصل قيمه فواصل أو علامات جدولة ويضعها في خلايا، سطرًا لكل صف. مثال: ملف التصدير من جهاز الحضور يصير ورقة فيها عمود لكل حقل."
        }
        "Reorders the rows so the selected column runs from smallest to largest: numbers ascending, words in alphabetical order, dates oldest first. Example: salaries from the lowest to the highest." => {
            "يعيد ترتيب الصفوف ليجري العمود المحدد من الأصغر إلى الأكبر: الأرقام تصاعديًا، والكلمات بترتيب الحروف، والتواريخ الأقدم أولًا. مثال: الرواتب من الأدنى إلى الأعلى."
        }
        "Reorders the rows so the selected column runs from largest to smallest: numbers descending, words from the last letter to the first, dates newest first. Example: salaries from the highest to the lowest." => {
            "يعيد ترتيب الصفوف ليجري العمود المحدد من الأكبر إلى الأصغر: الأرقام تنازليًا، والكلمات من آخر الحروف إلى أولها، والتواريخ الأحدث أولًا. مثال: الرواتب من الأعلى إلى الأدنى."
        }
        "Opens a dialog to sort by several columns in turn, or by colour, each ascending or descending. Example: first by department, then by name within each department." => {
            "يفتح مربع حوار للفرز بعدة أعمدة بالتتابع أو باللون، كل واحد تصاعديًا أو تنازليًا. مثال: أولًا حسب القسم ثم حسب الاسم داخل كل قسم."
        }
        "Puts a small arrow on every heading; click one to show only the rows that match what you pick, hiding the rest without deleting them. Example: show only the rows where the city is «البصرة»." => {
            "يضع سهمًا صغيرًا على كل عنوان؛ انقر واحدًا ليعرض الصفوف التي تطابق ما تختاره فقط ويخفي البقية من غير حذفها. مثال: اعرض الصفوف التي مدينتها «البصرة» فقط."
        }
        "Removes every filter so all the hidden rows come back, keeping the filter arrows in place. Example: after looking at one city, all cities show again." => {
            "يزيل كل التصفيات فتعود كل الصفوف المخفية، وتبقى أسهم التصفية في مكانها. مثال: بعد النظر في مدينة واحدة تظهر كل المدن من جديد."
        }
        "Runs the current filter and sort again after you changed or added data, so rows that no longer match disappear and new ones that match appear. Example: a new «البصرة» row joins the filtered list." => {
            "يشغّل التصفية والفرز الحاليين من جديد بعد أن غيّرت البيانات أو أضفت إليها، فتختفي الصفوف التي لم تعد تطابق وتظهر الجديدة التي تطابق. مثال: صف «البصرة» الجديد ينضم إلى القائمة المصفّاة."
        }
        "Splits one column into several at a separator you choose (a comma, a space, a dash) or at fixed widths. Example: «علي، بغداد» becomes «علي» in one cell and «بغداد» in the next." => {
            "يقسم عمودًا واحدًا إلى عدة أعمدة عند فاصل تختاره (فاصلة أو مسافة أو شرطة) أو بعرض ثابت. مثال: «علي، بغداد» تصير «علي» في خلية و«بغداد» في الخلية التالية."
        }
        "Looks at the example you typed beside the first row, works out the pattern and fills the rest of the column the same way. Example: type the first name taken from «علي حسن» and the other first names fill in." => {
            "ينظر في المثال الذي كتبته بجانب الصف الأول ويستنتج النمط ويملأ بقية العمود بالطريقة نفسها. مثال: اكتب الاسم الأول المأخوذ من «علي حسن» فتُملأ بقية الأسماء الأولى."
        }
        "Deletes rows that repeat an earlier row in the chosen columns, keeping the first of each; the sheet tells you how many went. Example: a name entered twice stays once." => {
            "يحذف الصفوف التي تكرر صفًا سابقًا في الأعمدة المختارة ويبقي الأول من كل واحد؛ وتخبرك الورقة كم صفًا حُذف. مثال: اسم أُدخل مرتين يبقى مرة واحدة."
        }
        "Limits what can be typed in the selected cells: a list of allowed words, a number range, a date; anything else is refused with a message. Example: the «الرتبة» column accepts only ranks from your list." => {
            "يحدّ ما يمكن كتابته في الخلايا المحددة: قائمة كلمات مسموحة أو نطاق أرقام أو تاريخ؛ وما عدا ذلك يُرفض برسالة. مثال: عمود «الرتبة» لا يقبل إلا الرتب من قائمتك."
        }
        "Tools for trying out numbers: Goal Seek finds the input that gives a result you want, Scenario Manager saves sets of inputs to compare, Data Table shows a formula for many inputs at once. Example: find the price that gives a profit of exactly ١٠٠٠." => {
            "أدوات لتجربة الأرقام: الاستهداف يجد المدخل الذي يعطي النتيجة التي تريدها، ومدير السيناريوهات يحفظ مجموعات مدخلات للمقارنة، وجدول البيانات يعرض صيغة لمدخلات كثيرة دفعة واحدة. مثال: جد السعر الذي يعطي ربحًا قدره ١٠٠٠ بالضبط."
        }
        "Makes a new sheet with a chart and a table that continue your dated figures into the future, with a band showing how sure the guess is. Example: twelve months of sales become a forecast for the next six." => {
            "ينشئ ورقة جديدة فيها مخطط وجدول يواصلان أرقامك المؤرخة إلى المستقبل، مع نطاق يبيّن مدى الثقة بالتخمين. مثال: اثنا عشر شهرًا من المبيعات تصير توقعًا للأشهر الستة التالية."
        }
        "Joins the selected rows or columns into an outline level you can fold shut with a minus sign and open again with a plus. Example: fold the twenty detail rows of a department into one line." => {
            "يضم الصفوف أو الأعمدة المحددة في مستوى مخطط تفصيلي يمكنك طيّه بعلامة ناقص وفتحه من جديد بعلامة زائد. مثال: اطوِ صفوف التفاصيل العشرين لقسم ما في سطر واحد."
        }
        "Removes the outline level from the selected rows or columns; they stay where they are but can no longer be folded. Example: the bracket beside the department rows disappears." => {
            "يزيل مستوى المخطط التفصيلي من الصفوف أو الأعمدة المحددة؛ تبقى في مكانها لكن لا يمكن طيّها بعد ذلك. مثال: يختفي القوس الذي بجانب صفوف القسم."
        }
        "Inserts a total row under each group of a sorted column and a grand total at the end, with outline buttons to fold the groups. Example: a sum under each department's rows and the overall sum at the bottom." => {
            "يدرج صف مجموع تحت كل مجموعة من عمود مفروز ومجموعًا كليًا في النهاية، مع أزرار مخطط تفصيلي لطي المجموعات. مثال: مجموع تحت صفوف كل قسم والمجموع الكلي في الأسفل."
        }
        "Opens the folded group so its rows (or columns) are visible again. Example: the department's twenty rows reappear under its total." => {
            "يفتح المجموعة المطوية فتظهر صفوفها (أو أعمدتها) من جديد. مثال: صفوف القسم العشرون تظهر من جديد تحت مجموعه."
        }
        "Folds the group shut so only its summary row (or column) stays visible. Example: the twenty detail rows vanish and only the department total remains." => {
            "يطوي المجموعة فلا يبقى ظاهرًا إلا صف ملخصها (أو عمودها). مثال: صفوف التفاصيل العشرون تختفي ولا يبقى إلا مجموع القسم."
        }
        "Opens the statistics tools: descriptive statistics, histogram, correlation, regression, moving average and more, each writing its results to a new range. Example: a histogram of ages in ten-year bands." => {
            "يفتح أدوات الإحصاء: الإحصاء الوصفي والمدرج التكراري والارتباط والانحدار والمتوسط المتحرك وغيرها، وكل واحدة تكتب نتائجها في نطاق جديد. مثال: مدرج تكراري للأعمار في فئات من عشر سنوات."
        }
        "Finds the input values that make a formula as large, as small or exactly as you want, while keeping the limits you set. Example: how many of each item to buy for the most profit within the budget." => {
            "يجد قيم المدخلات التي تجعل صيغة ما أكبر ما يمكن أو أصغر ما يمكن أو بالضبط كما تريد، مع الالتزام بالحدود التي تضعها. مثال: كم تشتري من كل صنف لأعلى ربح ضمن الميزانية."
        }
        "Opens the dialog where you set the rule for the selected cells (a list, a number range, a date, a text length), the message shown while typing and the error shown when a value is refused. Example: allow whole numbers from ١ to ١٠٠." => {
            "يفتح مربع الحوار الذي تضع فيه قاعدة الخلايا المحددة (قائمة أو نطاق أرقام أو تاريخ أو طول نص)، والرسالة التي تظهر أثناء الكتابة، والخطأ الذي يظهر حين تُرفض قيمة. مثال: اسمح بالأعداد الصحيحة من ١ إلى ١٠٠."
        }
        "Draws a red ring round every cell whose value breaks its validation rule, so you can find mistakes typed before the rule existed. Example: two salaries above the allowed maximum get red rings." => {
            "يرسم حلقة حمراء حول كل خلية تخالف قيمتها قاعدة التحقق، لتجد الأخطاء التي كُتبت قبل وجود القاعدة. مثال: راتبان فوق الحد الأقصى المسموح يحصلان على حلقتين حمراوين."
        }
        "Removes the validation rule from the selected cells so anything may be typed in them again, and clears the red rings. Example: the «الرتبة» column stops checking against the list." => {
            "يزيل قاعدة التحقق من الخلايا المحددة فيمكن كتابة أي شيء فيها من جديد، ويمسح الحلقات الحمراء. مثال: عمود «الرتبة» يتوقف عن المطابقة مع القائمة."
        }
        "Finds the input value that makes a formula come out at the number you want: tell it the formula cell, the target, and which cell to change. Example: which price makes the profit cell exactly ١٠٠٠." => {
            "يجد قيمة المدخل التي تجعل صيغة ما تخرج بالرقم الذي تريده: أخبره بخلية الصيغة والهدف والخلية التي يغيّرها. مثال: أي سعر يجعل خلية الربح ١٠٠٠ بالضبط."
        }
        "Saves several sets of input values under names so you can switch between them and compare their results in a summary. Example: «متفائل», «متوقع» and «متشائم» each with its own sales figures." => {
            "يحفظ عدة مجموعات من قيم المدخلات بأسماء لتبدّل بينها وتقارن نتائجها في ملخص. مثال: «متفائل» و«متوقع» و«متشائم» ولكل واحد أرقام مبيعاته."
        }
        "Fills a block with the result of one formula for every input value along a row or column (or both), so you see all the outcomes side by side. Example: the monthly payment for each interest rate and each loan length." => {
            "يملأ كتلة بنتيجة صيغة واحدة لكل قيمة مدخل على طول صف أو عمود (أو كليهما)، فترى كل النتائج جنبًا إلى جنب. مثال: القسط الشهري لكل سعر فائدة ولكل مدة قرض."
        }
        "Checks the spelling of the words in your sheet and suggests a correction for each word it does not know; you accept it or skip it. Example: «recieved» is flagged and «received» offered." => {
            "يدقّق إملاء الكلمات في ورقتك ويقترح تصحيحًا لكل كلمة لا يعرفها؛ فتقبله أو تتجاوزه. مثال: تُعلَّم كلمة «الطبب» ويُقترح «الطب»."
        }
        "Shows a count of what is in the workbook: how many sheets, filled cells, formulas, tables and charts. Example: 3 sheets, 120 cells with data, 45 formulas." => {
            "يعرض إحصاءً لما في المصنّف: كم ورقة، وكم خلية مملوءة، وكم صيغة وجدولًا ومخططًا. مثال: ٣ أوراق، و١٢٠ خلية فيها بيانات، و٤٥ صيغة."
        }
        "Looks for things that make the sheet hard to use for people with a screen reader or weak eyesight, and lists them with a fix. Example: a picture with no description, or grey text on a grey fill." => {
            "يبحث عمّا يصعّب استعمال الورقة على من يقرأ بقارئ الشاشة أو يضعف بصره، ويسردها مع طريقة إصلاحها. مثال: صورة من غير وصف، أو نص رمادي على تعبئة رمادية."
        }
        "Removes the comment attached to the selected cell, with all its replies; the cell's value stays. Example: the comment mark in the corner of B3 disappears." => {
            "يحذف التعليق المرفق بالخلية المحددة مع كل الردود عليه؛ وتبقى قيمة الخلية. مثال: تختفي علامة التعليق من زاوية الخلية B3."
        }
        "Opens a list of every comment in the sheet at the side of the window, so you can read them, reply and resolve them without hunting for the cells. Example: three comments on three different cells, listed together." => {
            "يفتح في جانب النافذة قائمة بكل تعليقات الورقة، لتقرأها وتردّ عليها وتُنهيها من غير أن تبحث عن خلاياها. مثال: ثلاثة تعليقات على ثلاث خلايا مختلفة، مسرودة معًا."
        }
        "Opens the choices for notes: a note is a yellow box of text attached to one cell, like a sticky note, shown when you point at the cell. Example: a note on B3 saying «check this figure»." => {
            "يفتح خيارات الملاحظات: الملاحظة صندوق أصفر من النص مرفق بخلية واحدة، كالورقة اللاصقة، يظهر حين تشير إلى الخلية. مثال: ملاحظة على B3 تقول «راجع هذا الرقم»."
        }
        "Locks the list of sheets: nobody can add, delete, rename, hide or move a sheet until the workbook is unprotected (with the password, if you set one). Example: the «+» for a new sheet stops working." => {
            "يقفل قائمة الأوراق: فلا يستطيع أحد إضافة ورقة أو حذفها أو إعادة تسميتها أو إخفائها أو نقلها حتى تُرفع الحماية عن المصنّف (بكلمة السر إن وضعتها). مثال: يتوقف زر «+» لورقة جديدة عن العمل."
        }
        "Chooses a few ranges that people may still fill in once the sheet is protected, each with its own password if you like. Example: the whole sheet is locked except B2:C6, where the clerk types the figures." => {
            "يختار نطاقات قليلة يبقى للناس أن يملؤوها بعد حماية الورقة، ولكل منها كلمة سر إن شئت. مثال: الورقة كلها مقفلة إلا B2:C6 حيث يكتب الكاتب الأرقام."
        }
        "Attaches a yellow note to the selected cell and lets you type in it; a small red corner on the cell shows the note is there. Example: a note on B3 saying «check this figure»." => {
            "يرفق ملاحظة صفراء بالخلية المحددة ويتيح لك الكتابة فيها؛ وتظهر زاوية حمراء صغيرة على الخلية تدل على وجود الملاحظة. مثال: ملاحظة على B3 تقول «راجع هذا الرقم»."
        }
        "Shows the selected cell's note all the time, or hides it again so it appears only when you point at the cell. Example: the note on B3 stays open while you work." => {
            "يُظهر ملاحظة الخلية المحددة طوال الوقت، أو يخفيها ثانية فلا تظهر إلا حين تشير إلى الخلية. مثال: تبقى ملاحظة B3 مفتوحة وأنت تعمل."
        }
        "Shows every note in the sheet at once, open beside its cell, so you can read them all in one look. Example: the notes on B3, C5 and D2 all open together." => {
            "يُظهر كل ملاحظات الورقة دفعة واحدة، كلٌّ مفتوحة بجانب خليتها، لتقرأها كلها بنظرة واحدة. مثال: تُفتح ملاحظات B3 وC5 وD2 معًا."
        }
        "Turns the sheet's notes into comments: the text stays, and people can now reply under it with their name and the time. Example: the yellow note on B3 becomes a comment thread." => {
            "يحوّل ملاحظات الورقة إلى تعليقات: يبقى النص، ويصير للناس أن يردّوا تحته بأسمائهم ووقت الرد. مثال: تصبح الملاحظة الصفراء على B3 سلسلة تعليقات."
        }
        "Opens a box where you pick how large the sheet is drawn on the screen: 200% for big cells, 50% to see more of the sheet at once. The numbers themselves do not change. Example: choose 75% and twice as many rows fit on the screen." => {
            "يفتح مربعًا تختار فيه حجم رسم الورقة على الشاشة: ٢٠٠٪ لخلايا كبيرة، و٥٠٪ لترى من الورقة أكثر دفعة واحدة. والأرقام نفسها لا تتغير. مثال: اختر ٧٥٪ فيتّسع على الشاشة ضعف عدد الصفوف."
        }
        "Puts the sheet back at its normal size after you zoomed in or out. Example: a sheet shown at 50% returns to 100%, cells at their usual size." => {
            "يعيد الورقة إلى حجمها الطبيعي بعد أن كبّرتها أو صغّرتها. مثال: ورقة معروضة بنسبة ٥٠٪ تعود إلى ١٠٠٪، والخلايا بحجمها المعتاد."
        }
        "Enlarges the sheet until the cells you selected fill the window, so a small table is easy to read. Example: select A1:D6 and those cells grow to fill the screen." => {
            "يكبّر الورقة حتى تملأ الخلايا التي حددتها النافذة، فيسهل قراءة جدول صغير. مثال: حدد A1:D6 فتكبر تلك الخلايا حتى تملأ الشاشة."
        }
        "Opens the choices for keeping rows or columns in place while you scroll: the first row, the first column, or everything above and to the left of the selected cell. Example: freeze the title row and it stays visible while you scroll down a long list." => {
            "يفتح خيارات تثبيت الصفوف أو الأعمدة في مكانها وأنت تمرّر: الصف الأول، أو العمود الأول، أو كل ما فوق الخلية المحددة وعن يسارها. مثال: ثبّت صف العناوين فيبقى ظاهرًا وأنت تمرّر قائمة طويلة."
        }
        "Keeps every row above the selected cell and every column to its left in place while the rest of the sheet scrolls. Example: select B2, and row 1 and column A stay visible as you scroll." => {
            "يبقي كل صف فوق الخلية المحددة وكل عمود عن يسارها في مكانه بينما يتحرك باقي الورقة. مثال: حدد B2 فيبقى الصف ١ والعمود A ظاهرين وأنت تمرّر."
        }
        "Keeps the first row in place while you scroll down, so the column titles are always visible. Example: scroll to row 500 and the titles in row 1 are still at the top." => {
            "يبقي الصف الأول في مكانه وأنت تمرّر إلى الأسفل، فتظل عناوين الأعمدة ظاهرة دائمًا. مثال: مرّر إلى الصف ٥٠٠ وما تزال عناوين الصف ١ في الأعلى."
        }
        "Keeps the first column in place while you scroll to the right, so the names in column A are always visible. Example: scroll to column Z and the names in A are still at the side." => {
            "يبقي العمود الأول في مكانه وأنت تمرّر إلى الجانب، فتظل الأسماء في العمود A ظاهرة دائمًا. مثال: مرّر إلى العمود Z وما تزال الأسماء في A على الجانب."
        }
        "Lets every row and column scroll again; nothing stays fixed on the screen. Example: the frozen title row scrolls away with the rest of the sheet." => {
            "يدع كل الصفوف والأعمدة تتحرك من جديد؛ فلا يبقى شيء مثبتًا على الشاشة. مثال: يتحرك صف العناوين المثبت مع باقي الورقة."
        }
        "Lists the scripts saved in this workbook — recorded sequences of commands — and runs the one you pick. Example: a script that sorts the list, adds totals and bolds the title, in one click." => {
            "يسرد النصوص البرمجية المحفوظة في هذا المصنّف — وهي سلاسل أوامر مسجَّلة — ويشغّل ما تختاره منها. مثال: نص يرتّب القائمة ويضيف المجاميع ويجعل العنوان عريضًا، بنقرة واحدة."
        }
        "Opens a search box where you type a few letters of what you want to do and pick the command from the list, instead of looking for its button. Example: type «freeze» and choose Freeze Top Row." => {
            "يفتح مربع بحث تكتب فيه بضعة حروف مما تريد فعله ثم تختار الأمر من القائمة، بدل البحث عن زره. مثال: اكتب «تثبيت» واختر تثبيت الصف العلوي."
        }
        "Explains how a program or an AI assistant can work the sheet for you: every button is also a command it can send, and it sees the same workbook. Example: an assistant fills B2:B9 and makes the chart while you watch." => {
            "يشرح كيف يستطيع برنامج أو مساعد ذكي أن يعمل على الورقة بدلًا منك: كل زر هو أيضًا أمر يمكنه إرساله، وهو يرى المصنّف نفسه. مثال: يملأ المساعد B2:B9 ويصنع المخطط وأنت تشاهد."
        }
        "Shows the list of everything done since the workbook was opened, step by step, so you can see what happened or run the same steps again. Example: «set B2», «bold», «sort A:C» in order." => {
            "يعرض قائمة بكل ما جرى منذ فتح المصنّف، خطوة خطوة، لترى ما حدث أو تعيد تنفيذ الخطوات نفسها. مثال: «ضبط B2» ثم «عريض» ثم «ترتيب A:C» بالترتيب."
        }
        "Shows which version of GridCraft this is, who made it, and the AI models that helped build it. Example: the version number to quote when you report a problem." => {
            "يعرض أي إصدار من GridCraft هذا، ومن صنعه، ونماذج الذكاء الاصطناعي التي ساعدت في بنائه. مثال: رقم الإصدار الذي تذكره حين تبلّغ عن مشكلة."
        }
        "Turns the table back into ordinary cells: the values and colours stay, but the filter arrows, the automatic growth and the table name are gone. Example: the table «Sales» becomes plain cells A1:C8." => {
            "يعيد الجدول خلايا عادية: تبقى القيم والألوان، لكن تذهب أسهم التصفية والنمو التلقائي واسم الجدول. مثال: يصبح الجدول «المبيعات» خلايا عادية A1:C8."
        }
        "Adds a box of buttons, one for each value in a column, so the table is filtered by clicking a button instead of opening the filter menu. Example: buttons «Baghdad», «Basra», «Erbil»; click one and only its rows show." => {
            "يضيف صندوقًا من الأزرار، زرًّا لكل قيمة في عمود، فيُصفّى الجدول بنقر زر بدل فتح قائمة التصفية. مثال: أزرار «بغداد» و«البصرة» و«أربيل»؛ انقر أحدها فلا تظهر إلا صفوفه."
        }
        "Opens a gallery of ready colour schemes for the table: the header colour, the stripes, the borders. Example: pick the blue style and the header turns blue with light blue stripes." => {
            "يفتح معرضًا من مخططات الألوان الجاهزة للجدول: لون الرأس، والخطوط المتناوبة، والحدود. مثال: اختر النمط الأزرق فيصير الرأس أزرق والخطوط أزرق فاتحًا."
        }
        "Shows or hides the table's title row, the one with the column names and filter arrows. Example: hide it and the table starts with the first data row." => {
            "يُظهر صف عناوين الجدول أو يخفيه، وهو الصف الذي فيه أسماء الأعمدة وأسهم التصفية. مثال: أخفِه فيبدأ الجدول بأول صف من البيانات."
        }
        "Adds a row at the bottom of the table that sums (or counts, averages…) each column, and keeps it up to date. Example: a Total row showing the sum of the Amount column." => {
            "يضيف في أسفل الجدول صفًا يجمع كل عمود (أو يعدّه، أو يحسب متوسطه…) ويبقيه محدّثًا. مثال: صف «المجموع» يعرض مجموع عمود المبلغ."
        }
        "Shades every other row of the table so a long row is easy to follow with the eye. Example: rows 2, 4 and 6 light blue, rows 3, 5 and 7 white." => {
            "يظلّل صفًا من كل صفين في الجدول حتى يسهل على العين تتبّع الصف الطويل. مثال: الصفوف ٢ و٤ و٦ أزرق فاتح، والصفوف ٣ و٥ و٧ بيضاء."
        }
        "Makes the first column of the table stand out, in bold, since it usually holds the names. Example: the Name column in bold." => {
            "يبرز العمود الأول في الجدول بخط عريض، إذ يحمل الأسماء عادةً. مثال: عمود الاسم بخط عريض."
        }
        "Makes the last column of the table stand out, in bold, since it often holds the total. Example: the Total column in bold." => {
            "يبرز العمود الأخير في الجدول بخط عريض، إذ يحمل المجموع غالبًا. مثال: عمود المجموع بخط عريض."
        }
        "Shades every other column of the table so the columns are easy to tell apart. Example: columns B and D light blue, A and C white." => {
            "يظلّل عمودًا من كل عمودين في الجدول حتى يسهل التمييز بين الأعمدة. مثال: العمودان B وD أزرق فاتح، وA وC أبيضان."
        }
        "Opens the list of parts you can add to or take off the chart: the legend and where it sits, the value labels on the bars, the gridlines. Example: add a legend under the chart and labels above the bars." => {
            "يفتح قائمة الأجزاء التي يمكنك إضافتها إلى المخطط أو نزعها منه: مفتاح الرموز وموضعه، وتسميات القيم على الأعمدة، وخطوط الشبكة. مثال: أضف مفتاح الرموز تحت المخطط وتسميات فوق الأعمدة."
        }
        "Swaps what the chart groups by: the rows become the series and the columns the categories. Example: bars grouped by month with one colour per branch become bars grouped by branch with one colour per month." => {
            "يبدّل ما يجمّع المخطط بحسبه: تصبح الصفوف سلاسل والأعمدة فئات. مثال: أعمدة مجمّعة بحسب الشهر بلون لكل فرع تصبح أعمدة مجمّعة بحسب الفرع بلون لكل شهر."
        }
        "Opens a box where you choose which cells the chart draws, which series to keep and which cells hold the category names. Example: widen the range from B2:B7 to B2:C7 and a second series appears." => {
            "يفتح مربعًا تختار فيه الخلايا التي يرسمها المخطط، وأي السلاسل تبقى، وأي الخلايا تحمل أسماء الفئات. مثال: وسّع النطاق من B2:B7 إلى B2:C7 فتظهر سلسلة ثانية."
        }
        "Picks another kind of chart for the same numbers: columns, bars, a line, a pie, an area, a scatter… Example: the column chart becomes a line chart." => {
            "يختار نوعًا آخر من المخططات للأرقام نفسها: أعمدة، أو أشرطة، أو خط، أو دائرة، أو مساحة، أو نقاط… مثال: يصبح مخطط الأعمدة مخططًا خطيًا."
        }
        "Opens a pane at the side with the chart's settings: colours of the bars, the axis range, the fonts, the borders. Example: change the first series from blue to green." => {
            "يفتح في الجانب جزءًا فيه إعدادات المخطط: ألوان الأعمدة، ومدى المحور، والخطوط، والحدود. مثال: غيّر السلسلة الأولى من الأزرق إلى الأخضر."
        }
        "Opens a box to write the chart's title, or to clear it. Example: the title «Sales 2026» appears above the bars." => {
            "يفتح مربعًا تكتب فيه عنوان المخطط، أو تمسحه. مثال: يظهر العنوان «مبيعات ٢٠٢٦» فوق الأعمدة."
        }
        "Removes the selected chart from the sheet; the numbers it drew stay in their cells. Example: the chart is gone, B2:B7 is untouched." => {
            "يزيل المخطط المحدد من الورقة؛ وتبقى الأرقام التي رسمها في خلاياها. مثال: يذهب المخطط، ويبقى B2:B7 كما هو."
        }
        "Turns a part of the chart on or off, or moves it: the legend (bottom, right, top or none), the value labels on the bars, the gridlines behind them. Example: Legend: Bottom puts the series names under the chart." => {
            "يشغّل جزءًا من المخطط أو يطفئه أو ينقله: مفتاح الرموز (أسفل أو يمين أو أعلى أو بلا)، وتسميات القيم على الأعمدة، وخطوط الشبكة خلفها. مثال: «مفتاح الرموز: أسفل» يضع أسماء السلاسل تحت المخطط."
        }
        "Colours the cells that rank highest or lowest in the selection — the top 10 items, the top 10%, the bottom 10, or those above or below the average — and keeps the colours right when the numbers change. Example: the ten biggest sales turn red." => {
            "يلوّن الخلايا الأعلى أو الأدنى رتبةً في التحديد — أكبر ١٠ عناصر، أو أعلى ١٠٪، أو أدنى ١٠، أو ما فوق المتوسط أو دونه — ويبقي الألوان صحيحة حين تتغير الأرقام. مثال: تصير أكبر عشر مبيعات حمراء."
        }
        "Removes the conditional formatting rules — from the selected cells only, or from the whole sheet — so the cells show their ordinary colours again. Example: the red top-ten cells go back to white." => {
            "يزيل قواعد التنسيق الشرطي — من الخلايا المحددة وحدها، أو من الورقة كلها — فتعود الخلايا إلى ألوانها العادية. مثال: تعود خلايا العشرة الكبار الحمراء بيضاء."
        }
        _ => return None,
    })
}
