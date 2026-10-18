//! Kashida justification for Arabic cell text (GridCraft AR): a justified or distributed Arabic
//! line fills its cell the way Excel (and service writing) does, by lengthening the joining stroke
//! between two letters rather than only widening spaces. egui cannot stretch a glyph, so the
//! stretch is made of tatweel characters (U+0640) inserted at the best joint of each word before
//! layout: before the last letter when the word allows it («علــى», «الطبــي»), else after a
//! seen-family letter («مســتندات»), then the other classic places — the joint rules of WordCraft
//! AR, here on `char`s because the cell text is shaped by egui itself.
//!
//! The grid breaks the text into lines itself ([`layout`]), measures each word with the cell's
//! font, hands the kashida budget (a share of the line's slack) to the joints and the rest to the
//! gaps between words, and draws every line as its own galley ([`line_job`]) whose words sit in
//! visual order, so a right-to-left paragraph reads correctly across lines too. Pure functions,
//! unit-tested with a fake measure.

use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontId};
use gridcraft_engine::model::style::{HAlign, KashidaLevel};

/// The stretch character.
pub const TATWEEL: char = '\u{0640}';
/// Letters that join on both sides.
const DUAL: &str = "بتثجحخسشصضطظعغفقكلمنهيئپچژگکی";
/// Letters that join only to the letter before them (right-joining).
const RIGHT: &str = "اأإآدذرزوؤةىء";
const ALEFS: &str = "اأإآ";
/// Tatweels one joint takes at most, and one line at most: bounds on the work a huge cell asks for.
const MAX_PER_JOINT: usize = 400;
const MAX_PER_LINE: usize = 2000;

/// Combining marks (harakat, Quranic marks): transparent to joining.
pub fn is_mark(c: char) -> bool {
    matches!(c, '\u{064B}'..='\u{0652}' | '\u{0670}' | '\u{0610}'..='\u{061A}' | '\u{06D6}'..='\u{06ED}')
}

/// A letter of the Arabic script (not a digit, mark or punctuation): a strong right-to-left char.
pub fn is_arabic_letter(c: char) -> bool {
    if is_mark(c) || matches!(c, '\u{0660}'..='\u{0669}' | '\u{06F0}'..='\u{06F9}' | '\u{060C}' | '\u{061B}' | '\u{061F}' | '\u{066A}'..='\u{066D}') {
        return false;
    }
    matches!(c, '\u{0620}'..='\u{064A}' | '\u{066E}'..='\u{06D3}' | '\u{06FA}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}' | '\u{FB50}'..='\u{FDFF}' | '\u{FE70}'..='\u{FEFF}')
}

/// Whether the text has Arabic letters at all (the only case the kashida layout is used for).
pub fn has_arabic(text: &str) -> bool {
    text.chars().any(is_arabic_letter)
}

/// Whether a paragraph reads right to left: its first strong character is an Arabic letter.
pub fn is_rtl(text: &str) -> bool {
    text.chars().find(|c| is_arabic_letter(*c) || c.is_alphabetic()).is_some_and(is_arabic_letter)
}

fn joins_forward(a: char) -> bool {
    DUAL.contains(a) || a == TATWEEL
}

/// Char positions of a word before which a tatweel may be inserted: the base letter before the
/// position joins the letter at it (both Arabic, the lam-alef ligature kept whole, marks
/// transparent). An existing tatweel is itself a joint: more may go before it. Sorted.
pub fn joints(word: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut prev: Option<char> = None;
    for (i, c) in word.chars().enumerate() {
        if is_mark(c) {
            continue;
        }
        if let Some(a) = prev {
            if c == TATWEEL {
                if joins_forward(a) {
                    out.push(i);
                }
            } else if joins_forward(a) && (DUAL.contains(c) || RIGHT.contains(c)) && !(a == 'ل' && ALEFS.contains(c)) {
                out.push(i);
            }
        }
        prev = Some(c);
    }
    out
}

/// Priority of inserting at `pos` (lower is better), the service-writing scheme: before the last
/// letter, else after س ش ص ض, else before a Baa-family letter followed by ر ز ي, else anywhere.
fn priority(chars: &[char], pos: usize) -> u8 {
    let before = chars.get(..pos).map(|s| s.iter().rev().copied().find(|c| !is_mark(*c) && *c != TATWEEL)).unwrap_or(None).unwrap_or(' ');
    let mut rest = chars.get(pos..).into_iter().flatten().copied().filter(|c| !is_mark(*c) && *c != TATWEEL);
    let nxt = rest.next().unwrap_or(' ');
    let nxt2 = rest.next();
    if nxt2.is_none() {
        1
    } else if "سشصض".contains(before) {
        2
    } else if "بتثنيئپ".contains(nxt) && nxt2.is_some_and(|c| "رزيىئ".contains(c)) {
        5
    } else {
        8
    }
}

/// The one joint of a word that takes its kashida: the best by priority, ties to the one nearer
/// the word's end. `None` for a word with no joint (Latin, digits, non-joining letters).
pub fn best_joint(word: &str) -> Option<usize> {
    let chars: Vec<char> = word.chars().collect();
    let mut best: Option<(u8, usize)> = None;
    for j in joints(word) {
        let pr = priority(&chars, j);
        if best.is_none_or(|(bp, bj)| pr < bp || (pr == bp && j > bj)) {
            best = Some((pr, j));
        }
    }
    best.map(|(_, j)| j)
}

/// Share of a line's slack that goes to kashida for an alignment and level (the rest widens
/// spaces). `None`: this alignment never fills its lines.
pub fn share(h: HAlign, level: KashidaLevel) -> Option<f32> {
    let of_level = match level {
        KashidaLevel::Off => None,
        KashidaLevel::Low => Some(0.35),
        KashidaLevel::Medium => Some(0.6),
        KashidaLevel::High => Some(0.9),
    };
    match h {
        HAlign::Justify => Some(of_level.unwrap_or(0.0)),
        HAlign::Distributed => Some(of_level.unwrap_or(0.6)),
        _ => None,
    }
}

/// Whether every line is filled (Distributed) or the last line of a paragraph stays as it is.
pub fn stretches_last(h: HAlign) -> bool {
    h == HAlign::Distributed
}

/// One laid-out line of a cell.
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    /// The words, tatweels inserted, in logical order.
    pub words: Vec<String>,
    /// Width of the gap between two words (the space, widened by its part of the slack).
    pub gap: f32,
    /// Measured width of the words and gaps.
    pub width: f32,
    /// The paragraph reads right to left.
    pub rtl: bool,
    /// The line was filled to the target width (false: a last line, or one with nothing to fill).
    pub filled: bool,
}

fn word_with_tatweels(word: &str, pos: usize, n: usize) -> String {
    let mut s = String::with_capacity(word.len() + n * TATWEEL.len_utf8());
    for (i, c) in word.chars().enumerate() {
        if i == pos {
            s.extend(std::iter::repeat_n(TATWEEL, n));
        }
        s.push(c);
    }
    if pos >= word.chars().count() {
        s.extend(std::iter::repeat_n(TATWEEL, n));
    }
    s
}

/// Lines of `text` for a cell `width` wide: words are measured with `measure` and broken greedily
/// (a word wider than the cell stands alone on its line); each line to fill gets `share` of its
/// slack as kashida at the best joint of each of its words, round robin, and the rest in its
/// gaps. A line without gaps takes all its slack as kashida. Paragraphs (`\n`) start new lines;
/// their last line is filled only when `stretch_last`.
pub fn layout(text: &str, width: f32, share: f32, stretch_last: bool, measure: &mut dyn FnMut(&str) -> f32) -> Vec<Line> {
    let width = if width.is_finite() && width > 0.0 { width } else { f32::INFINITY };
    let share = if share.is_finite() { share.clamp(0.0, 1.0) } else { 0.0 };
    let mut space_w = measure(" ");
    if !(space_w.is_finite() && space_w > 0.0) {
        space_w = (measure("a a") - measure("aa")).max(1.0);
    }
    let tatweel_w = measure(&TATWEEL.to_string());
    let tatweel_w = if tatweel_w.is_finite() && tatweel_w > 0.5 { Some(tatweel_w) } else { None };
    let mut out = Vec::new();
    for para in text.split('\n') {
        let rtl = is_rtl(para);
        let words: Vec<&str> = para.split(' ').filter(|w| !w.is_empty()).collect();
        if words.is_empty() {
            out.push(Line { words: Vec::new(), gap: space_w, width: 0.0, rtl, filled: false });
            continue;
        }
        let widths: Vec<f32> = words.iter().map(|w| measure(w).max(0.0)).collect();
        // Greedy line breaks.
        let mut lines: Vec<(usize, usize)> = Vec::new();
        let mut start = 0;
        let mut w = 0.0;
        for (i, ww) in widths.iter().enumerate() {
            let add = if i == start { *ww } else { space_w + *ww };
            if i > start && w + add > width + 0.01 {
                lines.push((start, i));
                start = i;
                w = *ww;
            } else {
                w += add;
            }
        }
        lines.push((start, words.len()));
        let n_lines = lines.len();
        for (li, (a, b)) in lines.into_iter().enumerate() {
            let last = li + 1 == n_lines;
            let line_words: Vec<&str> = words.get(a..b).unwrap_or_default().to_vec();
            let natural: f32 = widths.get(a..b).unwrap_or_default().iter().sum::<f32>() + space_w * (line_words.len().saturating_sub(1)) as f32;
            let slack = width - natural;
            let fill = (!last || stretch_last) && width.is_finite() && slack > 0.0;
            if !fill {
                out.push(Line { words: line_words.iter().map(|w| w.to_string()).collect(), gap: space_w, width: natural, rtl, filled: false });
                continue;
            }
            let gaps = line_words.len().saturating_sub(1);
            let budget = if gaps == 0 { slack } else { slack * share };
            // Round robin over the words that have a joint, one tatweel at a time.
            let mut stretched: Vec<(usize, usize, usize)> =
                line_words.iter().enumerate().filter_map(|(k, w)| best_joint(w).map(|j| (k, j, 0))).collect();
            if let Some(tw) = tatweel_w
                && !stretched.is_empty()
            {
                let mut used = 0.0;
                let mut total = 0;
                'fill: loop {
                    let mut any = false;
                    for s in stretched.iter_mut() {
                        if used + tw > budget + 0.01 || total >= MAX_PER_LINE {
                            break 'fill;
                        }
                        if s.2 < MAX_PER_JOINT {
                            s.2 += 1;
                            used += tw;
                            total += 1;
                            any = true;
                        }
                    }
                    if !any {
                        break;
                    }
                }
            }
            let mut owned: Vec<String> = line_words.iter().map(|w| w.to_string()).collect();
            let mut measured: Vec<f32> = widths.get(a..b).unwrap_or_default().to_vec();
            for (k, j, n) in &stretched {
                if *n > 0
                    && let (Some(w), Some(m)) = (owned.get_mut(*k), measured.get_mut(*k))
                {
                    *w = word_with_tatweels(w, *j, *n);
                    *m = measure(w).max(0.0);
                }
            }
            // The estimate was by the tatweel's own advance; shaping may differ, so take tatweels
            // back until the measured line fits.
            let line_w = |m: &[f32]| m.iter().sum::<f32>() + space_w * gaps as f32;
            let mut guard = MAX_PER_LINE;
            while line_w(&measured) > width + 0.01 && guard > 0 {
                guard -= 1;
                let Some(s) = stretched.iter_mut().filter(|s| s.2 > 0).max_by_key(|s| s.2) else { break };
                s.2 -= 1;
                if let (Some(src), Some(w), Some(m)) = (line_words.get(s.0), owned.get_mut(s.0), measured.get_mut(s.0)) {
                    *w = word_with_tatweels(src, s.1, s.2);
                    *m = measure(w).max(0.0);
                }
            }
            let total = line_w(&measured);
            let gap = if gaps > 0 && total < width { space_w + (width - total) / gaps as f32 } else { space_w };
            let final_w = measured.iter().sum::<f32>() + gap * gaps as f32;
            out.push(Line { words: owned, gap, width: final_w.min(width.max(total)), rtl, filled: true });
        }
    }
    out
}

/// Indices of a line's words in display order, left to right: a right-to-left line reverses them
/// and keeps runs of left-to-right words (numbers, Latin) in their own order.
pub fn visual_order(words: &[String], rtl: bool) -> Vec<usize> {
    let n = words.len();
    if !rtl {
        return (0..n).collect();
    }
    let mut order: Vec<usize> = (0..n).rev().collect();
    let ltr = |i: usize| words.get(i).is_some_and(|w| !has_arabic(w));
    let mut k = 0;
    while k < n {
        if order.get(k).is_some_and(|i| ltr(*i)) {
            let start = k;
            while k < n && order.get(k).is_some_and(|i| ltr(*i)) {
                k += 1;
            }
            if let Some(run) = order.get_mut(start..k) {
                run.reverse();
            }
        } else {
            k += 1;
        }
    }
    order
}

/// The galley job of one line: its words in visual order, each its own section (so the shaper
/// never reorders words across sections), the gap carried as the section's leading space.
pub fn line_job(line: &Line, font: FontId, color: Color32) -> LayoutJob {
    let mut job = LayoutJob { wrap: egui::text::TextWrapping::no_max_width(), ..LayoutJob::default() };
    let format = TextFormat::simple(font, color);
    for (k, i) in visual_order(&line.words, line.rtl).into_iter().enumerate() {
        if let Some(w) = line.words.get(i) {
            // A positive leading space keeps every word a section of its own (`append` merges
            // sections with none); the space character itself is not drawn.
            job.append(w, if k == 0 { 0.0 } else { line.gap.max(0.01) }, format.clone());
        }
    }
    if line.words.is_empty() {
        job.append(" ", 0.0, format);
    }
    job
}

/// Tatweels in a text.
pub fn tatweel_count(text: &str) -> usize {
    text.chars().filter(|c| *c == TATWEEL).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chars_at(word: &str, positions: &[usize]) -> Vec<char> {
        let c: Vec<char> = word.chars().collect();
        positions.iter().map(|p| c[*p - 1]).collect()
    }

    #[test]
    fn joints_follow_arabic_joining() {
        // ع-ل, ل-ى ; then ا does not join forward; ل-ط, ط-ب, ب-ي
        assert_eq!(joints("على"), vec![1, 2]);
        assert_eq!(joints("الطبي"), vec![2, 3, 4]);
        assert!(joints("أولاً").is_empty(), "lam-alef kept whole, و does not join");
        assert!(joints("Total").is_empty(), "Latin text has no joints");
        assert!(joints("١٢٣").is_empty(), "digits have no joints");
        // Marks are transparent: the joint sits after them.
        assert_eq!(joints("عَلَى"), vec![2, 4]);
    }

    #[test]
    fn existing_tatweel_is_a_joint() {
        assert_eq!(joints("جـ"), vec![1]);
        assert_eq!(joints("جـد"), vec![1, 2]);
        assert_eq!(best_joint("جـد"), Some(2));
    }

    #[test]
    fn one_stretch_per_word_before_the_last_letter() {
        // على: before ى; الطبي: before ي; مستندات: after س (seen family) since ا does not join and د
        // is right-joining, so no joint stands before the last letter.
        let letters: Vec<char> = ["على", "الطبي", "مستندات"].iter().map(|w| chars_at(w, &[best_joint(w).unwrap()])[0]).collect();
        assert_eq!(letters, vec!['ل', 'ب', 'س']);
        assert_eq!(best_joint("Total"), None);
        assert_eq!(word_with_tatweels("على", 2, 3), "علـــى");
    }

    /// A fake measure: every char 10 wide, a space 4, a tatweel 3.
    fn fake(s: &str) -> f32 {
        s.chars()
            .map(|c| {
                if c == ' ' {
                    4.0
                } else if c == TATWEEL {
                    3.0
                } else {
                    10.0
                }
            })
            .sum()
    }

    #[test]
    fn justify_fills_every_line_but_the_last_and_measures_within_the_target() {
        let text = "على الطبي مستندات على الطبي";
        let lines = layout(text, 100.0, 0.6, false, &mut fake);
        assert_eq!(lines.len(), 3, "{lines:?}");
        // Line 1: على الطبي (30+4+50 = 84, slack 16): 60 % → 9.6 → 3 tatweels, round robin → 2 + 1.
        let l = &lines[0];
        assert!(l.filled);
        assert_eq!(l.words, vec!["علــى", "الطبـي"]);
        assert!(l.width <= 100.0 + 0.01 && l.width > 99.9, "{l:?}");
        assert!(l.gap > 4.0, "the rest widens the gap: {l:?}");
        // Line 2: مستندات على (70+4+30 = 104 > 100) → مستندات alone (70, no gaps): all slack is kashida.
        let l = &lines[1];
        assert_eq!(l.words.len(), 1);
        assert_eq!(tatweel_count(&l.words[0]), 10);
        assert!(l.width <= 100.0 + 0.01, "{l:?}");
        // Last line stays as written.
        let l = &lines[2];
        assert!(!l.filled);
        assert_eq!(l.words, vec!["على", "الطبي"]);
        assert_eq!(l.gap, 4.0);
        assert!(lines.iter().all(|l| l.rtl));
    }

    #[test]
    fn distributed_fills_the_last_line_too_and_off_uses_spaces_only() {
        let lines = layout("على الطبي", 100.0, 0.6, true, &mut fake);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].filled && tatweel_count(&lines[0].words.concat()) == 3);
        let plain = layout("على الطبي مستندات", 100.0, 0.0, false, &mut fake);
        assert!(plain[0].filled);
        assert_eq!(tatweel_count(&plain[0].words.concat()), 0);
        assert!((plain[0].gap - 20.0).abs() < 0.01, "the whole slack is in the gap: {plain:?}");
    }

    #[test]
    fn latin_text_gets_no_tatweel_and_mixed_lines_keep_order() {
        let lines = layout("Total due now", 60.0, 0.9, false, &mut fake);
        assert_eq!(lines.len(), 3, "{lines:?}");
        assert_eq!(tatweel_count(&lines.iter().flat_map(|l| l.words.iter()).cloned().collect::<String>()), 0);
        assert!(!lines[0].rtl);
        assert_eq!(visual_order(&["a".to_string(), "b".to_string()], false), vec![0, 1]);
        let words: Vec<String> = ["على", "2024", "A1", "الطبي"].iter().map(|s| s.to_string()).collect();
        assert_eq!(visual_order(&words, true), vec![3, 1, 2, 0], "numbers keep their own order inside a right-to-left line");
        assert!(!has_arabic("Total 2024"));
        assert!(has_arabic("Total على"));
        assert!(!is_rtl("Total على") && is_rtl("٥ على"));
    }

    #[test]
    fn measured_overrun_is_taken_back() {
        // A measure where a tatweel next to a letter costs more than alone (shaping surprise).
        let mut odd = |s: &str| -> f32 { fake(s) + if s.contains(TATWEEL) && s.chars().count() > 1 { 5.0 } else { 0.0 } };
        let lines = layout("على الطبي مستندات", 100.0, 1.0, false, &mut odd);
        assert!(lines[0].width <= 100.0 + 0.01, "{:?}", lines[0]);
    }

    #[test]
    fn hostile_widths_and_empty_text_never_panic() {
        for w in [0.0, -5.0, f32::NAN, f32::INFINITY, 1e-9] {
            let l = layout("على الطبي", w, 0.6, true, &mut fake);
            assert!(!l.is_empty());
        }
        assert_eq!(layout("", 100.0, 0.6, true, &mut fake).len(), 1);
        assert_eq!(layout("\n\n", 100.0, 0.6, true, &mut fake).len(), 3);
        let _ = layout("ب".repeat(5000).as_str(), 50.0, 1.0, true, &mut fake);
        let _ = layout("ـ ـ ـ", 500.0, 1.0, true, &mut fake);
    }

    #[test]
    fn line_job_lays_out_every_word_as_its_own_section_within_the_target() {
        let lines = layout("على الطبي مستندات على", 120.0, 0.6, false, &mut fake);
        let job = line_job(&lines[0], FontId::proportional(14.0), Color32::BLACK);
        assert_eq!(job.sections.len(), lines[0].words.len());
        assert!(job.sections[1].leading_space > 4.0);
        assert!(!job.text.contains(' '));
        assert_eq!(tatweel_count(&job.text), tatweel_count(&lines[0].words.concat()));
        // Real layout: the kashida line fills the cell with egui's own measure and never overruns.
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(Default::default(), |ui| {
            let ctx = ui.ctx();
            let font = FontId::proportional(14.0);
            let mut measure = |s: &str| ctx.fonts_mut(|f| f.layout_no_wrap(s.to_string(), font.clone(), Color32::BLACK).size().x);
            // (egui's bundled fonts have no Arabic: every letter and the tatweel draw as the
            // replacement glyph, which is fine for measuring.)
            let target = 300.0;
            let lines = layout("على الطبي مستندات على الطبي مستندات على", target, 0.9, false, &mut measure);
            assert!(lines.len() >= 2, "{lines:?}");
            let mut tatweels = 0;
            for l in lines.iter().filter(|l| l.filled) {
                let galley = ctx.fonts_mut(|f| f.layout_job(line_job(l, font.clone(), Color32::BLACK)));
                assert!(galley.size().x <= target + 1.0, "{} > {target}: {l:?}", galley.size().x);
                assert!(galley.size().x >= target - 1.0 || l.words.len() == 1, "{} < {target}: {l:?}", galley.size().x);
                tatweels += tatweel_count(&l.words.concat());
            }
            assert!(tatweels > 0, "{lines:?}");
        });
        // No renderer consumes the font texture here.
        output.textures_delta.clear();
    }

    #[test]
    fn shares_follow_the_level_and_alignment() {
        assert_eq!(share(HAlign::Justify, KashidaLevel::Off), Some(0.0));
        assert_eq!(share(HAlign::Justify, KashidaLevel::Low), Some(0.35));
        assert_eq!(share(HAlign::Justify, KashidaLevel::High), Some(0.9));
        assert_eq!(share(HAlign::Distributed, KashidaLevel::Off), Some(0.6));
        assert_eq!(share(HAlign::Distributed, KashidaLevel::Low), Some(0.35));
        assert_eq!(share(HAlign::Left, KashidaLevel::High), None);
        assert!(stretches_last(HAlign::Distributed) && !stretches_last(HAlign::Justify));
    }
}
