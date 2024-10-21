// The tests are based on the Finnish specification for 6 dot math braille on the braille authority's web page (https://www.pistekirjoitus.fi/julkaisut/matematiikka-ja-tietotekniikka/) titled "Matematiikan, fysiikan ja kemain pistemerkinnät". Roughly translates to "Braille for mathematics, physics and chemistry." These tests are based on the edition published in 2022.
//
// Changes to the specifications in the rules and tests in MathCAT
// Some changes have been made to the rules and these tests test against the modified rules. The changes have been made, because the specification is for printed braille and intended for people authoring mathematics braille. Some things have been changed to be consistent in all situations and to work in the braille display context.

// UEB tests for the basic mathml tags
// Initial tests are from BANA guidelines, mostly about initial chars for code switching
//   http://www.brailleauthority.org/ueb/ueb_math_guidance/final_for_posting_ueb_math_guidance_may_2019_102419.pdf
// These tests start with "bana_"
//
// Many come from (refer to) https://iceb.org/guidelines_for_technical_material_2014.pdf
// For example, "fraction_6_1_1" is a fraction example from section 6.1, and is the first example there.
use crate::common::*;

#[test]
fn calculation_marks_1() {
    let expr = "<math><mn>3</mn><mo>+</mo><mn>4</mn><mo>=</mo><mn>7</mn></math>";
    test_braille("Finnish", expr, "⠼⠉⠀⠖⠼⠙⠀⠶⠼⠛");
}

#[test]
fn calculation_marks_7() {
    let expr = "<math><mrow><mi>&#x03C0;</mi><mo>&#x2248;</mo><mn>3,14</mn></mrow></math>";
    test_braille("Finnish", expr, "⠨⠏⠀⠸⠶⠼⠉⠂⠁⠙");
}

#[test]
fn fraction_10() {
    let expr = "<math>
        <mfrac><mn>3</mn><mn>4</mn></mfrac><mo>+</mo>
        <mfrac><mn>1</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn></math>";
    test_braille("Finnish", expr, "⠼⠉⠲⠀⠖⠼⠁⠲⠀⠶⠼⠁");
}

#[test]
fn fraction_13() {
    let expr = "<math><mfrac><mn>5</mn><mn>4</mn></mfrac><mo>=</mo><mn>1</mn><mfrac><mn>1</mn><mn>4</mn></mfrac></math>";
    test_braille("Finnish", expr, "⠼⠑⠲⠀⠶⠼⠁⠼⠁⠲");
}

#[test]
fn fraction_equations_3() {
    let expr = "<math>
        <mfrac>
            <mrow><mn>4</mn><mi>x</mi></mrow>
            <mrow><mn>6</mn><mo>(</mo><mn>1</mn><mo>&#x2212;</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac>
    </math>";
    test_braille("Finnish", expr, "⠼⠙⠀⠭⠀⠌⠦⠼⠋⠀⠦⠼⠁⠀⠤⠭⠴⠴");
}

#[test]
fn fraction_equations_6() {
    let expr = "<math><mfrac>
        <mrow><mfrac><mn>1</mn><mn>2</mn></mfrac><mo>+</mo><mfrac><mn>1</mn><mn>3</mn></mfrac></mrow>
        <mrow><mfrac><mn>1</mn><mn>4</mn></mfrac><mo>&#x2212;</mo><mfrac><mn>1</mn><mn>5</mn></mfrac></mrow>
    </mfrac></math>";
    test_braille("Finnish", expr, "⠦⠼⠁⠆⠀⠖⠼⠁⠒⠠⠴⠀⠌⠦⠼⠁⠲⠀⠤⠼⠁⠢⠠⠴");
}

#[test]
fn powers_3() {
    let expr = "<math><msup><mn>2</mn><mn>3</mn></msup><mo>+</mo><mn>5</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠉⠀⠖⠼⠑");
}

#[test]
fn powers_6() {
    let expr = "<math>
        <msup><mn>2</mn><mn>20</mn></msup>
        <mo>=</mo>
        <mn>1</mn><mtext>&#x2009;</mtext><mn>048</mn><mtext>&#x2009;</mtext><mn>576</mn>
    </math>
   ";
    test_braille("Finnish", expr, "⠼⠃⠬⠼⠃⠚⠀⠶⠼⠁⠄⠚⠙⠓⠄⠑⠛⠋");
}

#[test]
fn roots_5() {
    let expr = "<math><msup><mn>27</mn><mfrac><mn>1</mn><mn>3</mn></mfrac></msup><mo>=</mo>
                        <mroot><mn>27</mn><mn>3</mn></mroot><mo>=</mo><mn>3</mn></math>";
    test_braille("Finnish", expr, "⠼⠃⠛⠬⠼⠁⠒⠀⠶⠩⠼⠉⠐⠼⠃⠛⠀⠶⠼⠉");
}

#[test]
fn roots_7() {
    let expr = "<math><msqrt><mn>20</mn><mo>+</mo><mn>5</mn></msqrt><mo>=</mo><msqrt><mn>25</mn></msqrt><mo>=</mo><mn>5</mn></math>";
    test_braille("Finnish", expr, "⠩⠦⠼⠃⠚⠀⠖⠼⠑⠀⠴⠀⠶⠩⠼⠃⠑⠀⠶⠼⠑");
}

#[test]
fn vectors_1() {
    let expr = "<math><mover><mi>a</mi><mo>&#xAF;</mo></mover></math>";
    test_braille("Finnish", expr, "⠁⠱");
}

#[test]
fn chemistry_2_4() {
    // From MathType
    let expr = "<math><msub><mtext>C</mtext><mn>2</mn></msub><msub><mtext>H</mtext><mn>5</mn></msub><mtext>OH</mtext></math>";
    test_braille("Finnish", expr, "⠠⠉⠼⠆⠠⠓⠼⠢⠠⠕⠠⠓");
}

#[test]
fn chemistry_2_8() {
    // From MathType
    let expr = "<math><msup><mrow><mtext>Cu</mtext></mrow><mrow><mn>2</mn><mo>+</mo></mrow></msup></math>";
    test_braille("Finnish", expr, "⠠⠉⠥⠬⠦⠼⠃⠀⠖⠴⠀");
}
