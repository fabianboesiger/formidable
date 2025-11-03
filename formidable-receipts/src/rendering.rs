use formidable::Form;

mod typst_wrapper;

pub enum RenderingError {}

pub async fn to_pdf<F>(form: F) -> Result<Vec<u8>, RenderingError>
where
    F: Form,
{
    use typst_wrapper::TypstWrapperWorld;

    let content = format!(
        r#"
        #set page(
            paper: "a4",
            header: align(right)[Form Title],
            numbering: "1"
        )
    "#
    );

    // All the abstraction needed is here (!)
    let world = TypstWrapperWorld::new(
        std::env::current_dir().unwrap().display().to_string(),
        content,
    );

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling Typst document");

    // Output to pdf
    let pdf =
        typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default()).expect("Error exporting PDF");
    //fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    //let output = typst_html::html(&document).expect("Error exporting HTML");

    Ok(pdf)
}
